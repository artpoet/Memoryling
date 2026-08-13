use rusqlite::{params, OptionalExtension, Transaction, TransactionBehavior};
use sha2::{Digest, Sha256};

use crate::memory::store::MemoryStore;

use super::model::{
    CompiledContext, DailyAttemptSummary, DailyCitation, DailyInsight, DailyScoutSettings,
    DailyScoutState, LocalNow, ProviderInsight, MODEL_ID, PROVIDER_ID,
};

impl MemoryStore {
    pub(crate) fn daily_settings(&self) -> Result<Option<DailyScoutSettings>, String> {
        let connection = self.open_connection()?;
        connection
            .query_row(
                "SELECT enabled, locale, delivery_hour, delivery_minute,
                        consent_scope_json, consent_scope_hash
                 FROM daily_scout_settings WHERE singleton_id = 1",
                [],
                |row| {
                    Ok(DailyScoutSettings {
                        enabled: row.get::<_, i64>(0)? == 1,
                        locale: row.get(1)?,
                        delivery_hour: row.get(2)?,
                        delivery_minute: row.get(3)?,
                        consent_scope_json: row.get(4)?,
                        consent_scope_hash: row.get(5)?,
                    })
                },
            )
            .optional()
            .map_err(|_| "Memoryling could not read Daily Scout settings.".to_string())
    }

    pub(crate) fn save_daily_settings(
        &self,
        locale: &str,
        delivery_hour: u8,
        delivery_minute: u8,
        consent_scope_json: &str,
        consent_scope_hash: &str,
        updated_at: &str,
    ) -> Result<(), String> {
        let connection = self.open_connection()?;
        connection
            .execute(
                "INSERT INTO daily_scout_settings
                    (singleton_id, enabled, provider, model, locale, delivery_hour,
                     delivery_minute, consent_schema_version, consent_revision,
                     consent_scope_json, consent_scope_hash, updated_at)
                 VALUES (1, 1, ?1, ?2, ?3, ?4, ?5, 1, 1, ?6, ?7, ?8)
                 ON CONFLICT(singleton_id) DO UPDATE SET
                    enabled = 1,
                    provider = excluded.provider,
                    model = excluded.model,
                    locale = excluded.locale,
                    delivery_hour = excluded.delivery_hour,
                    delivery_minute = excluded.delivery_minute,
                    consent_schema_version = excluded.consent_schema_version,
                    consent_revision = excluded.consent_revision,
                    consent_scope_json = excluded.consent_scope_json,
                    consent_scope_hash = excluded.consent_scope_hash,
                    updated_at = excluded.updated_at",
                params![
                    PROVIDER_ID,
                    MODEL_ID,
                    locale,
                    delivery_hour,
                    delivery_minute,
                    consent_scope_json,
                    consent_scope_hash,
                    updated_at
                ],
            )
            .map_err(|_| "Memoryling could not save Daily Scout settings.".to_string())?;
        Ok(())
    }

    pub(crate) fn disable_daily_scout(&self, updated_at: &str) -> Result<(), String> {
        let connection = self.open_connection()?;
        connection
            .execute(
                "UPDATE daily_scout_settings SET enabled = 0, updated_at = ?1
                 WHERE singleton_id = 1",
                [updated_at],
            )
            .map_err(|_| "Memoryling could not turn off Daily Scout.".to_string())?;
        Ok(())
    }

    pub(crate) fn reserve_daily_attempt(
        &self,
        now: &LocalNow,
        context_hash: &str,
    ) -> Result<bool, String> {
        let mut connection = self.open_connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| "Memoryling could not reserve today's Daily Scout attempt.".to_string())?;
        let latest_date = transaction
            .query_row(
                "SELECT MAX(local_date) FROM daily_search_attempts",
                [],
                |row| row.get::<_, Option<String>>(0),
            )
            .map_err(|_| "Memoryling could not inspect the daily search budget.".to_string())?;
        if latest_date
            .as_deref()
            .is_some_and(|date| date >= now.local_date.as_str())
        {
            transaction
                .commit()
                .map_err(|_| "Memoryling could not close the daily budget check.".to_string())?;
            return Ok(false);
        }
        transaction
            .execute(
                "INSERT INTO daily_search_attempts
                    (local_date, timezone, started_at, status, context_hash)
                 VALUES (?1, ?2, ?3, 'running', ?4)",
                params![now.local_date, now.timezone, now.timestamp, context_hash],
            )
            .map_err(|_| "Memoryling could not reserve today's Daily Scout attempt.".to_string())?;
        transaction
            .commit()
            .map_err(|_| "Memoryling could not commit the daily search budget.".to_string())?;
        Ok(true)
    }

    pub(crate) fn recover_interrupted_daily_attempts(&self, timestamp: &str) -> Result<(), String> {
        let connection = self.open_connection()?;
        connection
            .execute(
                "UPDATE daily_search_attempts
                 SET status = 'failed', completed_at = ?1, error_code = 'interrupted'
                 WHERE status = 'running'",
                [timestamp],
            )
            .map_err(|_| {
                "Memoryling could not recover the previous Daily Scout attempt.".to_string()
            })?;
        Ok(())
    }

    pub(crate) fn finish_daily_attempt_success(
        &self,
        now: &LocalNow,
        context: &CompiledContext,
        insight: &ProviderInsight,
        relevance_reason: &str,
    ) -> Result<(), String> {
        let mut connection = self.open_connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| "Memoryling could not save today's Daily Scout result.".to_string())?;
        let running = transaction
            .query_row(
                "SELECT COUNT(*) FROM daily_search_attempts
                 WHERE local_date = ?1 AND status = 'running'",
                [&now.local_date],
                |row| row.get::<_, i64>(0),
            )
            .map_err(|_| "Memoryling could not verify today's Daily Scout attempt.".to_string())?;
        if running != 1 {
            return Err("Today's Daily Scout attempt is no longer active.".to_string());
        }
        for source_id in &context.source_ids {
            let exists = transaction
                .query_row(
                    "SELECT COUNT(*) FROM source_imports WHERE source_id = ?1",
                    [source_id],
                    |row| row.get::<_, i64>(0),
                )
                .map_err(|_| "Memoryling could not verify the insight source.".to_string())?;
            if exists != 1 {
                return Err("The approved work source changed during the daily search.".to_string());
            }
        }
        let insight_id = hex_hash(&format!(
            "daily-insight|{}|{}|{}",
            now.local_date, context.context_hash, insight.pet_message
        ));
        transaction
            .execute(
                "INSERT INTO daily_insights
                    (id, local_date, provider, model, pet_message, strength,
                     relevance_reason, searched_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    insight_id,
                    now.local_date,
                    PROVIDER_ID,
                    MODEL_ID,
                    insight.pet_message,
                    insight.strength,
                    relevance_reason,
                    now.timestamp
                ],
            )
            .map_err(|_| "Memoryling could not save today's Daily Scout insight.".to_string())?;
        for source_id in &context.source_ids {
            transaction
                .execute(
                    "INSERT INTO daily_insight_sources (insight_id, source_id)
                     VALUES (?1, ?2)",
                    params![insight_id, source_id],
                )
                .map_err(|_| "Memoryling could not save the insight lineage.".to_string())?;
        }
        for (position, citation) in insight.citations.iter().take(3).enumerate() {
            transaction
                .execute(
                    "INSERT INTO daily_insight_citations
                        (insight_id, position, title, url)
                     VALUES (?1, ?2, ?3, ?4)",
                    params![insight_id, position as i64, citation.title, citation.url],
                )
                .map_err(|_| "Memoryling could not save the insight citations.".to_string())?;
        }
        transaction
            .execute(
                "UPDATE daily_search_attempts
                 SET status = 'succeeded', completed_at = ?2, error_code = NULL
                 WHERE local_date = ?1 AND status = 'running'",
                params![now.local_date, now.timestamp],
            )
            .map_err(|_| {
                "Memoryling could not complete today's Daily Scout attempt.".to_string()
            })?;
        transaction
            .commit()
            .map_err(|_| "Memoryling could not commit today's Daily Scout result.".to_string())
    }

    pub(crate) fn finish_daily_attempt_failure(
        &self,
        local_date: &str,
        timestamp: &str,
        error_code: &str,
    ) -> Result<(), String> {
        let connection = self.open_connection()?;
        connection
            .execute(
                "UPDATE daily_search_attempts
                 SET status = 'failed', completed_at = ?2, error_code = ?3
                 WHERE local_date = ?1 AND status = 'running'",
                params![local_date, timestamp, error_code],
            )
            .map_err(|_| "Memoryling could not record today's Daily Scout failure.".to_string())?;
        Ok(())
    }

    pub(crate) fn mark_latest_daily_insight_read(&self, timestamp: &str) -> Result<(), String> {
        let connection = self.open_connection()?;
        connection
            .execute(
                "UPDATE daily_insights SET read_at = ?1
                 WHERE id = (SELECT id FROM daily_insights ORDER BY searched_at DESC LIMIT 1)",
                [timestamp],
            )
            .map_err(|_| "Memoryling could not update the Daily Scout message.".to_string())?;
        Ok(())
    }

    pub(crate) fn clear_daily_scout_history(&self) -> Result<(), String> {
        let connection = self.open_connection()?;
        connection
            .execute("DELETE FROM daily_insights", [])
            .map_err(|_| "Memoryling could not clear Daily Scout history.".to_string())?;
        Ok(())
    }

    pub(crate) fn has_daily_citation_url(&self, url: &str) -> Result<bool, String> {
        let connection = self.open_connection()?;
        connection
            .query_row(
                "SELECT EXISTS(
                    SELECT 1 FROM daily_insight_citations WHERE url = ?1
                 )",
                [url],
                |row| row.get::<_, bool>(0),
            )
            .map_err(|_| "Memoryling could not verify the Daily Scout source link.".to_string())
    }

    pub(crate) fn daily_scout_state(
        &self,
        has_api_key: bool,
        context: Option<&CompiledContext>,
        now: &LocalNow,
    ) -> Result<DailyScoutState, String> {
        let connection = self.open_connection()?;
        let settings = self.daily_settings()?;
        let attempt = connection
            .query_row(
                "SELECT local_date, status, error_code
                 FROM daily_search_attempts WHERE local_date = ?1",
                [&now.local_date],
                |row| {
                    Ok(DailyAttemptSummary {
                        local_date: row.get(0)?,
                        status: row.get(1)?,
                        error_code: row.get(2)?,
                    })
                },
            )
            .optional()
            .map_err(|_| "Memoryling could not read today's Daily Scout attempt.".to_string())?;
        let latest = latest_insight(&connection)?;
        let enabled = settings.as_ref().is_some_and(|value| value.enabled);
        let status = if !enabled {
            "off"
        } else if !has_api_key {
            "needs-key"
        } else if context.is_none() {
            "needs-memory"
        } else if latest.as_ref().is_some_and(|insight| !insight.read) {
            "ready"
        } else {
            match attempt.as_ref().map(|attempt| attempt.status.as_str()) {
                Some("running") => "running",
                Some("failed") => "failed",
                Some("succeeded") => "complete",
                _ => "scheduled",
            }
        };
        Ok(DailyScoutState {
            enabled,
            has_api_key,
            can_enable: has_api_key && context.is_some(),
            provider: PROVIDER_ID.to_string(),
            model: MODEL_ID.to_string(),
            delivery_time: settings
                .as_ref()
                .map(DailyScoutSettings::delivery_time)
                .unwrap_or_else(|| "10:00".to_string()),
            status: status.to_string(),
            context_preview: context.map(|value| value.outbound.clone()),
            latest_insight: latest,
            today_attempt: attempt,
        })
    }
}

pub(crate) fn invalidate_daily_scout_for_source(
    transaction: &Transaction<'_>,
    source_id: &str,
) -> Result<(), String> {
    transaction
        .execute(
            "DELETE FROM daily_insights
             WHERE id IN (
                SELECT insight_id FROM daily_insight_sources WHERE source_id = ?1
             )",
            [source_id],
        )
        .map_err(|_| "Memoryling could not remove dependent Daily Scout insights.".to_string())?;
    transaction
        .execute(
            "UPDATE daily_scout_settings
             SET enabled = 0, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
             WHERE singleton_id = 1",
            [],
        )
        .map_err(|_| "Memoryling could not invalidate Daily Scout consent.".to_string())?;
    Ok(())
}

fn latest_insight(connection: &rusqlite::Connection) -> Result<Option<DailyInsight>, String> {
    let row = connection
        .query_row(
            "SELECT id, local_date, provider, model, pet_message, strength,
                    relevance_reason, searched_at, read_at
             FROM daily_insights ORDER BY searched_at DESC LIMIT 1",
            [],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, String>(6)?,
                    row.get::<_, String>(7)?,
                    row.get::<_, Option<String>>(8)?,
                ))
            },
        )
        .optional()
        .map_err(|_| "Memoryling could not read the latest Daily Scout insight.".to_string())?;
    let Some((
        id,
        local_date,
        provider,
        model,
        pet_message,
        strength,
        relevance_reason,
        searched_at,
        read_at,
    )) = row
    else {
        return Ok(None);
    };
    let citations = {
        let mut statement = connection
            .prepare(
                "SELECT title, url FROM daily_insight_citations
                 WHERE insight_id = ?1 ORDER BY position",
            )
            .map_err(|_| "Memoryling could not prepare the insight sources.".to_string())?;
        let rows = statement
            .query_map([&id], |row| {
                Ok(DailyCitation {
                    title: row.get(0)?,
                    url: row.get(1)?,
                })
            })
            .map_err(|_| "Memoryling could not read the insight sources.".to_string())?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Memoryling could not collect the insight sources.".to_string())?
    };
    Ok(Some(DailyInsight {
        id,
        local_date,
        provider,
        model,
        pet_message,
        strength,
        relevance_reason,
        searched_at,
        read: read_at.is_some(),
        citations,
    }))
}

fn hex_hash(value: &str) -> String {
    Sha256::digest(value.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
