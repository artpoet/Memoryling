use std::{collections::HashSet, fs, path::PathBuf, time::Duration};

use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;

use super::{
    adapter::stable_id,
    model::{
        ActiveMemorySource, AgentActivity, AgentOperationRenderState, AgentOperationSummary,
        BodyModule, ConsentScopeV1, CreatureEnvelope, CreatureMark, CreatureMotion,
        CreaturePalette, CreatureRenderMark, CreatureRenderMarkStyle, CreatureRenderState,
        CreatureStage, DailyScoutRenderState, ImportState, LineageSource, MemoryState, PetDialogue,
        PreparedImport, RealMemoryAccess, CODEX_MEMORY_ADAPTER_ID, CODEX_THREAD_ADAPTER_ID,
        DERIVATION_VERSION, STORE_SCHEMA_VERSION,
    },
};

const MIGRATION_0001: &str = include_str!("../../migrations/0001_first_memory.sql");
const MIGRATION_0002: &str = include_str!("../../migrations/0002_source_consent_scope.sql");
const MIGRATION_0003: &str = include_str!("../../migrations/0003_daily_memory_scout.sql");
const MIGRATION_0004: &str = include_str!("../../migrations/0004_agent_memory_sync.sql");
const MIGRATION_0005: &str = include_str!("../../migrations/0005_agent_operation_protocol.sql");

pub(crate) struct MemoryStore {
    path: PathBuf,
}

impl MemoryStore {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub(crate) fn state(&self) -> Result<MemoryState, String> {
        let connection = self.open_connection()?;
        state_from_connection(&connection)
    }

    pub(crate) fn creature_render_state(&self) -> Result<CreatureRenderState, String> {
        let connection = self.open_connection()?;
        creature_render_state_from_connection(&connection)
    }

    pub(crate) fn ensure_no_approved_source(&self) -> Result<(), String> {
        let connection = self.open_connection()?;
        if count(&connection, "source_imports")? == 0 {
            Ok(())
        } else {
            Err("Forget the currently approved source before reading a different one.".to_string())
        }
    }

    pub(crate) fn approve_import(
        &self,
        prepared: &PreparedImport,
        selected_record_ids: &[String],
    ) -> Result<MemoryState, String> {
        if selected_record_ids.is_empty() {
            return Err("Select at least one memory before approval.".to_string());
        }

        let selected = selected_record_ids
            .iter()
            .map(String::as_str)
            .collect::<HashSet<_>>();
        if selected.len() != selected_record_ids.len()
            || selected.iter().any(|record_id| {
                !prepared
                    .events
                    .iter()
                    .any(|event| event.source_record_id == *record_id)
            })
        {
            return Err("The approval contains an unknown memory record.".to_string());
        }
        if prepared.source.adapter_id == CODEX_MEMORY_ADAPTER_ID
            && selected.len() != prepared.events.len()
        {
            return Err(
                "Agent memory consent covers the complete allowlisted source, not individual files."
                    .to_string(),
            );
        }

        let mut connection = self.open_connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| local_store_error("begin approval", error))?;

        let existing_source = transaction
            .query_row(
                "SELECT source_id FROM source_imports ORDER BY source_id LIMIT 1",
                [],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|error| local_store_error("check approved source", error))?;
        if existing_source
            .as_deref()
            .is_some_and(|source_id| source_id != prepared.source.id)
        {
            return Err(
                "Forget the currently approved source before importing a different one."
                    .to_string(),
            );
        }

        clear_derivations(&transaction)?;
        transaction
            .execute(
                "DELETE FROM source_imports WHERE source_id = ?1",
                params![prepared.source.id],
            )
            .map_err(|error| local_store_error("replace approved source", error))?;
        transaction
            .execute(
                "INSERT INTO source_imports
                    (source_id, adapter_id, adapter_version, display_name, source_locator,
                     source_content_hash)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    prepared.source.id,
                    prepared.source.adapter_id,
                    prepared.source.adapter_version,
                    prepared.source.display_name,
                    prepared.source.locator,
                    prepared.source_content_hash
                ],
            )
            .map_err(|error| local_store_error("save approved source", error))?;
        transaction
            .execute(
                "INSERT INTO source_consent_scopes
                    (source_id, schema_version, consent_revision, scope_json, scope_hash)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    prepared.source.id,
                    prepared.consent_scope.schema_version,
                    prepared.consent_scope.revision,
                    prepared.consent_scope_json,
                    prepared.consent_scope_hash
                ],
            )
            .map_err(|error| local_store_error("save approved consent scope", error))?;

        for event in prepared
            .events
            .iter()
            .filter(|event| selected.contains(event.source_record_id.as_str()))
        {
            transaction
                .execute(
                    "INSERT INTO memory_events
                        (id, schema_version, source_id, source_record_id, source_timestamp,
                         kind, normalized_text, content_hash)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    params![
                        event.id,
                        event.schema_version,
                        event.source_id,
                        event.source_record_id,
                        event.source_timestamp,
                        event.kind,
                        event.normalized_text,
                        event.content_hash
                    ],
                )
                .map_err(|error| local_store_error("save normalized memory", error))?;
        }

        if prepared.source.adapter_id == CODEX_MEMORY_ADAPTER_ID {
            transaction
                .execute(
                    "INSERT INTO source_sync_state
                        (source_id, automatic_sync, sync_status, last_attempt_at,
                         last_successful_sync_at, synced_record_count, error_code)
                     VALUES (?1, 1, 'synced', strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                             strftime('%Y-%m-%dT%H:%M:%fZ', 'now'), ?2, NULL)",
                    params![prepared.source.id, prepared.events.len() as i64],
                )
                .map_err(|error| local_store_error("save automatic sync state", error))?;
        }

        rederive(&transaction)?;
        let state = state_from_connection(&transaction)?;
        transaction
            .commit()
            .map_err(|error| local_store_error("commit approval", error))?;
        Ok(state)
    }

    pub(crate) fn forget_source(&self, source_id: &str) -> Result<MemoryState, String> {
        let mut connection = self.open_connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| local_store_error("begin forgetting", error))?;

        clear_derivations(&transaction)?;
        crate::daily_scout::store::invalidate_daily_scout_for_source(&transaction, source_id)?;
        let deleted = transaction
            .execute(
                "DELETE FROM source_imports WHERE source_id = ?1",
                params![source_id],
            )
            .map_err(|error| local_store_error("forget source", error))?;
        if deleted == 0 {
            return Err("The selected source is not currently imported.".to_string());
        }

        rederive(&transaction)?;
        let state = state_from_connection(&transaction)?;
        transaction
            .commit()
            .map_err(|error| local_store_error("commit forgetting", error))?;
        Ok(state)
    }

    pub(crate) fn approved_agent_memory_scope(&self) -> Result<Option<ConsentScopeV1>, String> {
        let connection = self.open_connection()?;
        let scope_json = connection
            .query_row(
                "SELECT scs.scope_json
                 FROM source_consent_scopes scs
                 JOIN source_imports si ON si.source_id = scs.source_id
                 WHERE si.adapter_id = ?1",
                params![CODEX_MEMORY_ADAPTER_ID],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|error| local_store_error("read approved Agent memory scope", error))?;
        scope_json
            .map(|json| {
                serde_json::from_str(&json)
                    .map_err(|_| "The approved Agent memory consent scope is invalid.".to_string())
            })
            .transpose()
    }

    pub(crate) fn sync_agent_memory(
        &self,
        prepared: &PreparedImport,
    ) -> Result<MemoryState, String> {
        if prepared.source.adapter_id != CODEX_MEMORY_ADAPTER_ID
            || !prepared.consent_scope.automatic_sync
        {
            return Err("The prepared source is not an automatic Agent memory source.".to_string());
        }
        let mut connection = self.open_connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| local_store_error("begin Agent memory sync", error))?;
        let stored_scope_hash = transaction
            .query_row(
                "SELECT scs.scope_hash
                 FROM source_consent_scopes scs
                 JOIN source_imports si ON si.source_id = scs.source_id
                 WHERE si.source_id = ?1 AND si.adapter_id = ?2",
                params![prepared.source.id, CODEX_MEMORY_ADAPTER_ID],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|error| local_store_error("verify Agent memory consent", error))?
            .ok_or_else(|| "Codex Agent memory is not approved for automatic sync.".to_string())?;
        if stored_scope_hash != prepared.consent_scope_hash {
            return Err(
                "The Codex Agent memory source location or consent scope changed.".to_string(),
            );
        }

        clear_derivations(&transaction)?;
        transaction
            .execute(
                "DELETE FROM memory_events WHERE source_id = ?1",
                params![prepared.source.id],
            )
            .map_err(|error| local_store_error("replace Agent memory events", error))?;
        transaction
            .execute(
                "UPDATE source_imports SET source_content_hash = ?2 WHERE source_id = ?1",
                params![prepared.source.id, prepared.source_content_hash],
            )
            .map_err(|error| local_store_error("update Agent memory source", error))?;
        insert_all_events(&transaction, prepared)?;
        rederive(&transaction)?;
        transaction
            .execute(
                "UPDATE source_sync_state
                 SET sync_status = 'synced',
                     last_attempt_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                     last_successful_sync_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                     synced_record_count = ?2,
                     error_code = NULL
                 WHERE source_id = ?1",
                params![prepared.source.id, prepared.events.len() as i64],
            )
            .map_err(|error| local_store_error("record Agent memory sync", error))?;
        let state = state_from_connection(&transaction)?;
        transaction
            .commit()
            .map_err(|error| local_store_error("commit Agent memory sync", error))?;
        Ok(state)
    }

    pub(crate) fn record_agent_sync_failure(
        &self,
        status: &str,
        error_code: &str,
        clear_source_events: bool,
    ) -> Result<MemoryState, String> {
        if !matches!(status, "source-missing" | "needs-attention") {
            return Err("The Agent memory sync status is invalid.".to_string());
        }
        let mut connection = self.open_connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| local_store_error("begin Agent memory sync recovery", error))?;
        let source_id = transaction
            .query_row(
                "SELECT source_id FROM source_imports WHERE adapter_id = ?1",
                params![CODEX_MEMORY_ADAPTER_ID],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|error| local_store_error("find approved Agent memory source", error))?
            .ok_or_else(|| "Codex Agent memory is not approved for automatic sync.".to_string())?;
        if clear_source_events {
            clear_derivations(&transaction)?;
            transaction
                .execute(
                    "DELETE FROM memory_events WHERE source_id = ?1",
                    params![source_id],
                )
                .map_err(|error| local_store_error("clear unavailable Agent memories", error))?;
            rederive(&transaction)?;
        }
        transaction
            .execute(
                "UPDATE source_sync_state
                 SET sync_status = ?2,
                     last_attempt_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                     synced_record_count = CASE WHEN ?3 THEN 0 ELSE synced_record_count END,
                     error_code = ?4
                 WHERE source_id = ?1",
                params![source_id, status, clear_source_events, error_code],
            )
            .map_err(|error| local_store_error("record Agent memory sync failure", error))?;
        let state = state_from_connection(&transaction)?;
        transaction
            .commit()
            .map_err(|error| local_store_error("commit Agent memory sync recovery", error))?;
        Ok(state)
    }

    pub(crate) fn apply_agent_operation(
        &self,
        package: &super::agent_operation::AgentOperationPackage,
    ) -> Result<(), String> {
        package.validate()?;
        let mut connection = self.open_connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| local_store_error("begin Agent operation", error))?;
        let existing_digest = transaction
            .query_row(
                "SELECT source_digest FROM agent_operations WHERE operation_id = ?1",
                params![package.operation_id],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|error| local_store_error("check Agent operation identity", error))?;
        if let Some(existing_digest) = existing_digest {
            if existing_digest != package.source_digest {
                return Err(
                    "The Agent operation ID was reused with different evidence.".to_string()
                );
            }
            transaction
                .execute(
                    "UPDATE agent_operation_runtime
                     SET inbox_status = 'applied', inbox_error_code = NULL,
                         last_inbox_checked_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
                     WHERE singleton_id = 1",
                    [],
                )
                .map_err(|error| local_store_error("confirm duplicate Agent operation", error))?;
            transaction
                .commit()
                .map_err(|error| local_store_error("commit duplicate Agent operation", error))?;
            return Ok(());
        }

        transaction
            .execute("DELETE FROM agent_operations", [])
            .map_err(|error| local_store_error("replace prior Agent operation", error))?;

        transaction
            .execute(
                "INSERT INTO agent_operations
                    (operation_id, schema_version, generated_at, agent_family, source_digest,
                     dominant_activity, secondary_activity, journey_state)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    package.operation_id,
                    package.schema_version,
                    package.generated_at,
                    package.agent.family,
                    package.source_digest,
                    activity_to_db(package.profile.dominant_activity),
                    package.profile.secondary_activity.map(activity_to_db),
                    package.profile.journey_state,
                ],
            )
            .map_err(|error| local_store_error("save Agent operation", error))?;
        for evidence in &package.evidence {
            transaction
                .execute(
                    "INSERT INTO agent_operation_evidence
                        (evidence_id, operation_id, evidence_kind, reference_hash, observed_at)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![
                        evidence.id,
                        package.operation_id,
                        evidence.kind,
                        evidence.reference_hash,
                        evidence.observed_at,
                    ],
                )
                .map_err(|error| local_store_error("save Agent operation evidence", error))?;
        }
        for dialogue in &package.dialogues {
            transaction
                .execute(
                    "INSERT INTO agent_dialogue_cards
                        (dialogue_id, operation_id, text_en, text_zh_tw, trigger_kind, priority,
                         not_before, expires_at, cooldown_minutes, max_uses)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                    params![
                        dialogue.id,
                        package.operation_id,
                        dialogue.text.en,
                        dialogue.text.zh_tw,
                        dialogue.trigger,
                        dialogue.priority,
                        dialogue.not_before,
                        dialogue.expires_at,
                        dialogue.cooldown_minutes,
                        dialogue.max_uses,
                    ],
                )
                .map_err(|error| local_store_error("save Agent dialogue", error))?;
            for evidence_id in &dialogue.evidence_ids {
                transaction
                    .execute(
                        "INSERT INTO agent_dialogue_evidence (dialogue_id, evidence_id)
                         VALUES (?1, ?2)",
                        params![dialogue.id, evidence_id],
                    )
                    .map_err(|error| local_store_error("save dialogue lineage", error))?;
            }
        }

        let now = super::agent_operation::local_clock().0;
        let now_value = OffsetDateTime::parse(&now, &time::format_description::well_known::Rfc3339)
            .map_err(|_| "Memoryling could not evaluate the opening dialogue clock.".to_string())?;
        let first_dialogue = package
            .dialogues
            .iter()
            .filter(|dialogue| {
                dialogue.trigger == "on-open"
                    && super::agent_operation::dialogue_is_active_at(dialogue, now_value)
            })
            .min_by(|left, right| {
                right
                    .priority
                    .cmp(&left.priority)
                    .then_with(|| left.id.cmp(&right.id))
            })
            .map(|dialogue| dialogue.id.clone());
        if let Some(dialogue_id) = &first_dialogue {
            transaction
                .execute(
                    "UPDATE agent_dialogue_cards
                     SET use_count = 1, last_used_at = ?2 WHERE dialogue_id = ?1",
                    params![dialogue_id, now],
                )
                .map_err(|error| local_store_error("activate opening Agent dialogue", error))?;
        }
        transaction
            .execute(
                "UPDATE agent_operation_runtime
                 SET current_dialogue_id = ?1, inbox_status = 'applied', inbox_error_code = NULL,
                     last_inbox_checked_at = ?2 WHERE singleton_id = 1",
                params![first_dialogue, now],
            )
            .map_err(|error| local_store_error("activate Agent operation", error))?;
        transaction
            .commit()
            .map_err(|error| local_store_error("commit Agent operation", error))?;
        Ok(())
    }

    pub(crate) fn clear_agent_operations(&self) -> Result<MemoryState, String> {
        let mut connection = self.open_connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| local_store_error("begin clearing Agent operation", error))?;
        transaction
            .execute("DELETE FROM agent_operations", [])
            .map_err(|error| local_store_error("clear Agent operation", error))?;
        transaction
            .execute("DELETE FROM agent_dialogue_daily_usage", [])
            .map_err(|error| local_store_error("clear Agent dialogue budget", error))?;
        transaction
            .execute(
                "UPDATE agent_operation_runtime
                 SET current_dialogue_id = NULL, inbox_status = 'waiting', inbox_error_code = NULL,
                     last_inbox_checked_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
                 WHERE singleton_id = 1",
                [],
            )
            .map_err(|error| local_store_error("reset Agent operation runtime", error))?;
        transaction
            .commit()
            .map_err(|error| local_store_error("commit clearing Agent operation", error))?;
        self.state()
    }

    pub(crate) fn record_agent_operation_error(&self, error_code: &str) -> Result<(), String> {
        let connection = self.open_connection()?;
        connection
            .execute(
                "UPDATE agent_operation_runtime
                 SET inbox_status = 'invalid', inbox_error_code = ?1,
                     last_inbox_checked_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
                 WHERE singleton_id = 1",
                params![error_code],
            )
            .map_err(|error| local_store_error("record Agent operation rejection", error))?;
        Ok(())
    }

    pub(crate) fn advance_agent_dialogue(
        &self,
        trigger: &str,
    ) -> Result<CreatureRenderState, String> {
        let (now_text, local_date, local_hour) = super::agent_operation::local_clock();
        if trigger == "ambient" && !(9..22).contains(&local_hour) {
            return self.creature_render_state();
        }
        let now = OffsetDateTime::parse(&now_text, &time::format_description::well_known::Rfc3339)
            .map_err(|_| "Memoryling could not evaluate the local dialogue clock.".to_string())?;
        let mut connection = self.open_connection()?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| local_store_error("begin Agent dialogue", error))?;
        if trigger == "ambient" {
            let used = transaction
                .query_row(
                    "SELECT ambient_count FROM agent_dialogue_daily_usage WHERE local_date = ?1",
                    params![local_date],
                    |row| row.get::<_, i64>(0),
                )
                .optional()
                .map_err(|error| local_store_error("read ambient dialogue budget", error))?
                .unwrap_or(0);
            if used >= 2 {
                transaction
                    .commit()
                    .map_err(|error| local_store_error("commit quiet dialogue check", error))?;
                return self.creature_render_state();
            }
        }
        let latest_operation = transaction
            .query_row(
                "SELECT operation_id FROM agent_operations ORDER BY applied_at DESC LIMIT 1",
                [],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|error| local_store_error("find current Agent operation", error))?;
        let Some(latest_operation) = latest_operation else {
            transaction
                .commit()
                .map_err(|error| local_store_error("commit empty dialogue check", error))?;
            return self.creature_render_state();
        };
        let current_dialogue = transaction
            .query_row(
                "SELECT current_dialogue_id FROM agent_operation_runtime WHERE singleton_id = 1",
                [],
                |row| row.get::<_, Option<String>>(0),
            )
            .map_err(|error| local_store_error("read current Agent dialogue", error))?;
        let candidates = {
            let mut statement = transaction
                .prepare(
                    "SELECT dialogue_id, not_before, expires_at, cooldown_minutes, last_used_at
                     FROM agent_dialogue_cards
                     WHERE operation_id = ?1 AND trigger_kind = ?2 AND use_count < max_uses
                     ORDER BY priority DESC,
                              CASE WHEN last_used_at IS NULL THEN 0 ELSE 1 END,
                              last_used_at, dialogue_id",
                )
                .map_err(|error| local_store_error("prepare Agent dialogue selection", error))?;
            let rows = statement
                .query_map(params![latest_operation, trigger], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, Option<String>>(1)?,
                        row.get::<_, Option<String>>(2)?,
                        row.get::<_, i64>(3)?,
                        row.get::<_, Option<String>>(4)?,
                    ))
                })
                .map_err(|error| local_store_error("read Agent dialogue candidates", error))?;
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|error| local_store_error("collect Agent dialogue candidates", error))?
        };
        let selected =
            candidates
                .into_iter()
                .find(|(id, not_before, expires_at, cooldown, last)| {
                    if current_dialogue.as_deref() == Some(id.as_str()) {
                        return false;
                    }
                    let not_before_ok = not_before.as_deref().is_none_or(|value| {
                        OffsetDateTime::parse(value, &time::format_description::well_known::Rfc3339)
                            .is_ok_and(|bound| now >= bound)
                    });
                    let expires_ok = expires_at.as_deref().is_none_or(|value| {
                        OffsetDateTime::parse(value, &time::format_description::well_known::Rfc3339)
                            .is_ok_and(|bound| now <= bound)
                    });
                    let cooldown_ok = last.as_deref().is_none_or(|value| {
                        OffsetDateTime::parse(value, &time::format_description::well_known::Rfc3339)
                            .is_ok_and(|used| now - used >= time::Duration::minutes(*cooldown))
                    });
                    not_before_ok && expires_ok && cooldown_ok
                });
        if let Some((dialogue_id, ..)) = selected {
            transaction
                .execute(
                    "UPDATE agent_dialogue_cards
                     SET use_count = use_count + 1, last_used_at = ?2 WHERE dialogue_id = ?1",
                    params![dialogue_id, now_text],
                )
                .map_err(|error| local_store_error("consume Agent dialogue", error))?;
            transaction
                .execute(
                    "UPDATE agent_operation_runtime SET current_dialogue_id = ?1
                     WHERE singleton_id = 1",
                    params![dialogue_id],
                )
                .map_err(|error| local_store_error("show Agent dialogue", error))?;
            if trigger == "ambient" {
                transaction
                    .execute(
                        "INSERT INTO agent_dialogue_daily_usage (local_date, ambient_count)
                         VALUES (?1, 1)
                         ON CONFLICT(local_date) DO UPDATE SET ambient_count = ambient_count + 1",
                        params![local_date],
                    )
                    .map_err(|error| local_store_error("record ambient dialogue budget", error))?;
            }
        }
        transaction
            .commit()
            .map_err(|error| local_store_error("commit Agent dialogue", error))?;
        self.creature_render_state()
    }

    pub(crate) fn open_connection(&self) -> Result<Connection, String> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|_| {
                "Memoryling could not create its local app-data directory.".to_string()
            })?;
        }

        let mut connection = Connection::open(&self.path)
            .map_err(|error| local_store_error("open database", error))?;
        connection
            .busy_timeout(Duration::from_secs(5))
            .map_err(|error| local_store_error("configure database timeout", error))?;
        connection
            .pragma_update(None, "foreign_keys", true)
            .map_err(|error| local_store_error("enable foreign keys", error))?;
        connection
            .pragma_update(None, "secure_delete", true)
            .map_err(|error| local_store_error("enable secure deletion", error))?;

        let migration = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| local_store_error("begin schema migration", error))?;
        let version = migration
            .pragma_query_value(None, "user_version", |row| row.get::<_, i64>(0))
            .map_err(|error| local_store_error("read schema version", error))?;
        match version {
            0 => migration
                .execute_batch(&format!(
                    "{MIGRATION_0001}\n{MIGRATION_0002}\n{MIGRATION_0003}\n{MIGRATION_0004}\n{MIGRATION_0005}"
                ))
                .map_err(|error| local_store_error("apply schema migrations", error))?,
            1 => {
                migrate_v1_to_v2(&migration)?;
                migrate_v2_to_v3(&migration)?;
                migrate_v3_to_v4(&migration)?;
                migrate_v4_to_v5(&migration)?;
            }
            2 => {
                migrate_v2_to_v3(&migration)?;
                migrate_v3_to_v4(&migration)?;
                migrate_v4_to_v5(&migration)?;
            }
            3 => {
                migrate_v3_to_v4(&migration)?;
                migrate_v4_to_v5(&migration)?;
            }
            4 => migrate_v4_to_v5(&migration)?,
            STORE_SCHEMA_VERSION => {}
            _ => return Err("The local Memoryling database schema is not supported.".to_string()),
        }
        migration
            .commit()
            .map_err(|error| local_store_error("commit schema migration", error))?;

        Ok(connection)
    }
}

fn migrate_v4_to_v5(transaction: &Transaction<'_>) -> Result<(), String> {
    transaction
        .execute_batch(MIGRATION_0005)
        .map_err(|error| local_store_error("apply Agent operation migration", error))
}

fn migrate_v3_to_v4(transaction: &Transaction<'_>) -> Result<(), String> {
    transaction
        .execute_batch(MIGRATION_0004)
        .map_err(|error| local_store_error("apply Agent memory sync migration", error))
}

fn insert_all_events(
    transaction: &Transaction<'_>,
    prepared: &PreparedImport,
) -> Result<(), String> {
    for event in &prepared.events {
        transaction
            .execute(
                "INSERT INTO memory_events
                    (id, schema_version, source_id, source_record_id, source_timestamp,
                     kind, normalized_text, content_hash)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    event.id,
                    event.schema_version,
                    event.source_id,
                    event.source_record_id,
                    event.source_timestamp,
                    event.kind,
                    event.normalized_text,
                    event.content_hash
                ],
            )
            .map_err(|error| local_store_error("save normalized Agent memory", error))?;
    }
    Ok(())
}

fn migrate_v2_to_v3(transaction: &Transaction<'_>) -> Result<(), String> {
    transaction
        .execute_batch(MIGRATION_0003)
        .map_err(|error| local_store_error("apply Daily Memory Scout migration", error))
}

fn migrate_v1_to_v2(transaction: &Transaction<'_>) -> Result<(), String> {
    transaction
        .execute_batch(MIGRATION_0002)
        .map_err(|error| local_store_error("apply consent schema migration", error))?;
    let source_count = count(transaction, "source_imports")?;
    if source_count == 0 {
        return Ok(());
    }
    let compatible_count = transaction
        .query_row(
            "SELECT COUNT(*) FROM source_imports
             WHERE source_id = 'codex.synthetic.first-memory'
               AND adapter_id = 'codex-durable-memory'
               AND adapter_version = 1",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| local_store_error("validate legacy source", error))?;
    if source_count != usize::try_from(compatible_count).unwrap_or(0) {
        return Err("The legacy local source requires a fresh explicit consent.".to_string());
    }
    let source = super::adapter::list_sources()?
        .into_iter()
        .next()
        .ok_or_else(|| "The bundled source consent contract is unavailable.".to_string())?;
    let (scope, scope_json, scope_hash) = super::adapter::fixture_consent_contract(&source)?;
    transaction
        .execute(
            "INSERT INTO source_consent_scopes
                (source_id, schema_version, consent_revision, scope_json, scope_hash)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                source.id,
                scope.schema_version,
                scope.revision,
                scope_json,
                scope_hash
            ],
        )
        .map_err(|error| local_store_error("backfill legacy fixture consent", error))?;
    Ok(())
}

fn clear_derivations(transaction: &Transaction<'_>) -> Result<(), String> {
    transaction
        .execute("DELETE FROM world_effects", [])
        .map_err(|error| local_store_error("clear world effects", error))?;
    transaction
        .execute("DELETE FROM derived_signals", [])
        .map_err(|error| local_store_error("clear derived signals", error))?;
    Ok(())
}

fn rederive(transaction: &Transaction<'_>) -> Result<(), String> {
    let inputs = {
        let mut statement = transaction
            .prepare(
                "SELECT id, kind
                 FROM memory_events
                 ORDER BY id",
            )
            .map_err(|error| local_store_error("prepare derivation", error))?;
        let rows = statement
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(|error| local_store_error("read derivation inputs", error))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|error| local_store_error("collect derivation inputs", error))?
    };

    let mut agent_memory_event_ids = Vec::new();
    for (event_id, kind) in inputs {
        if kind == "agent-memory-document" {
            agent_memory_event_ids.push(event_id);
            continue;
        }
        if kind != "completion" {
            continue;
        }

        let version = DERIVATION_VERSION.to_string();
        let signal_id = stable_id("signal", &[&event_id, "completion", &version]);
        let effect_id = stable_id("effect", &[&signal_id, "completion-star", &version]);

        transaction
            .execute(
                "INSERT INTO derived_signals
                    (id, signal_type, confidence, derivation_version)
                 VALUES (?1, 'completion', 1.0, ?2)",
                params![signal_id, DERIVATION_VERSION],
            )
            .map_err(|error| local_store_error("save derived signal", error))?;
        transaction
            .execute(
                "INSERT INTO derived_signal_sources (signal_id, memory_event_id)
                 VALUES (?1, ?2)",
                params![signal_id, event_id],
            )
            .map_err(|error| local_store_error("save signal lineage", error))?;
        transaction
            .execute(
                "INSERT INTO world_effects
                    (id, effect_type, effect_style, state, explanation_key, derivation_version)
                 VALUES (?1, 'visual-mark', 'completion-star', 'active',
                         'approved_completion_created_star', ?2)",
                params![effect_id, DERIVATION_VERSION],
            )
            .map_err(|error| local_store_error("save world effect", error))?;
        transaction
            .execute(
                "INSERT INTO world_effect_signals (effect_id, signal_id)
                 VALUES (?1, ?2)",
                params![effect_id, signal_id],
            )
            .map_err(|error| local_store_error("save effect lineage", error))?;
    }

    if !agent_memory_event_ids.is_empty() {
        let version = DERIVATION_VERSION.to_string();
        let joined_ids = agent_memory_event_ids.join("|");
        let signal_id = stable_id(
            "signal",
            &[&joined_ids, "agent-memory-continuity", &version],
        );
        let effect_id = stable_id("effect", &[&signal_id, "memory-halo", &version]);
        transaction
            .execute(
                "INSERT INTO derived_signals
                    (id, signal_type, confidence, derivation_version)
                 VALUES (?1, 'agent-memory-continuity', 1.0, ?2)",
                params![signal_id, DERIVATION_VERSION],
            )
            .map_err(|error| local_store_error("save Agent memory signal", error))?;
        for event_id in agent_memory_event_ids {
            transaction
                .execute(
                    "INSERT INTO derived_signal_sources (signal_id, memory_event_id)
                     VALUES (?1, ?2)",
                    params![signal_id, event_id],
                )
                .map_err(|error| local_store_error("save Agent memory lineage", error))?;
        }
        transaction
            .execute(
                "INSERT INTO world_effects
                    (id, effect_type, effect_style, state, explanation_key, derivation_version)
                 VALUES (?1, 'visual-mark', 'memory-halo', 'active',
                         'approved_agent_memories_created_halo', ?2)",
                params![effect_id, DERIVATION_VERSION],
            )
            .map_err(|error| local_store_error("save Agent memory world effect", error))?;
        transaction
            .execute(
                "INSERT INTO world_effect_signals (effect_id, signal_id) VALUES (?1, ?2)",
                params![effect_id, signal_id],
            )
            .map_err(|error| local_store_error("save Agent memory effect lineage", error))?;
    }

    Ok(())
}

fn state_from_connection(connection: &Connection) -> Result<MemoryState, String> {
    let source_count = count(connection, "source_imports")?;
    let event_count = count(connection, "memory_events")?;
    let signal_count = count(connection, "derived_signals")?;

    let effect_rows = {
        let mut statement = connection
            .prepare(
                "SELECT we.id, we.effect_style, ds.signal_type, ds.confidence,
                        we.derivation_version, we.explanation_key
                 FROM world_effects we
                 JOIN world_effect_signals wes ON wes.effect_id = we.id
                 JOIN derived_signals ds ON ds.id = wes.signal_id
                 WHERE we.state = 'active'
                 ORDER BY we.id",
            )
            .map_err(|error| local_store_error("prepare memory state", error))?;
        let rows = statement
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, f64>(3)?,
                    row.get::<_, i64>(4)?,
                    row.get::<_, String>(5)?,
                ))
            })
            .map_err(|error| local_store_error("read memory state", error))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|error| local_store_error("collect memory state", error))?
    };

    let mut marks = Vec::with_capacity(effect_rows.len());
    for (effect_id, style, signal_type, confidence, derivation_version, explanation_key) in
        effect_rows
    {
        let lineage = {
            let mut statement = connection
                .prepare(
                    "SELECT me.id, me.schema_version, me.source_id, si.display_name,
                            si.adapter_id, si.adapter_version, me.source_record_id,
                            me.source_timestamp, me.normalized_text, me.content_hash,
                            scs.scope_hash, scs.consent_revision
                     FROM world_effect_signals wes
                     JOIN derived_signal_sources dss ON dss.signal_id = wes.signal_id
                     JOIN memory_events me ON me.id = dss.memory_event_id
                     JOIN source_imports si ON si.source_id = me.source_id
                     JOIN source_consent_scopes scs ON scs.source_id = me.source_id
                     WHERE wes.effect_id = ?1
                     ORDER BY me.id",
                )
                .map_err(|error| local_store_error("prepare effect lineage", error))?;
            let rows = statement
                .query_map(params![effect_id], |row| {
                    let adapter_id = row.get::<_, String>(4)?;
                    let normalized_text = row.get::<_, String>(8)?;
                    let content_redacted = matches!(
                        adapter_id.as_str(),
                        CODEX_THREAD_ADAPTER_ID | CODEX_MEMORY_ADAPTER_ID
                    );
                    Ok(LineageSource {
                        memory_event_id: row.get(0)?,
                        memory_event_schema_version: row.get(1)?,
                        source_id: row.get(2)?,
                        source_label: row.get(3)?,
                        adapter_id,
                        adapter_version: row.get(5)?,
                        source_record_id: row.get(6)?,
                        source_timestamp: row.get(7)?,
                        memory_text: (!content_redacted).then_some(normalized_text.clone()),
                        content_redacted,
                        character_count: normalized_text.chars().count(),
                        content_hash: row.get(9)?,
                        consent_scope_hash: content_redacted.then(|| row.get(10)).transpose()?,
                        consent_revision: content_redacted
                            .then(|| row.get::<_, u8>(11))
                            .transpose()?,
                    })
                })
                .map_err(|error| local_store_error("read effect lineage", error))?;
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|error| local_store_error("collect effect lineage", error))?
        };

        marks.push(CreatureMark {
            id: effect_id,
            style,
            signal_type,
            confidence,
            derivation_version,
            explanation_key,
            lineage,
        });
    }

    let active_source = connection
        .query_row(
            "SELECT si.source_id, si.adapter_id, si.display_name,
                    COALESCE(ss.automatic_sync, 0), COALESCE(ss.sync_status, 'manual'),
                    ss.last_successful_sync_at,
                    COALESCE(ss.synced_record_count,
                        (SELECT COUNT(*) FROM memory_events me WHERE me.source_id = si.source_id))
             FROM source_imports si
             LEFT JOIN source_sync_state ss ON ss.source_id = si.source_id
             ORDER BY si.source_id LIMIT 1",
            [],
            |row| {
                let record_count = row.get::<_, i64>(6)?;
                Ok(ActiveMemorySource {
                    source_id: row.get(0)?,
                    adapter_id: row.get(1)?,
                    display_name: row.get(2)?,
                    automatic_sync: row.get::<_, i64>(3)? == 1,
                    sync_status: row.get(4)?,
                    last_successful_sync_at: row.get(5)?,
                    synced_record_count: usize::try_from(record_count).unwrap_or(0),
                })
            },
        )
        .optional()
        .map_err(|error| local_store_error("read active memory source", error))?;

    let agent_operation = connection
        .query_row(
            "SELECT ao.applied_at, ao.dominant_activity,
                    (SELECT COUNT(*) FROM agent_dialogue_cards adc
                     WHERE adc.operation_id = ao.operation_id)
             FROM agent_operations ao ORDER BY ao.applied_at DESC LIMIT 1",
            [],
            |row| {
                let activity = row.get::<_, String>(1)?;
                let dialogue_count = row.get::<_, i64>(2)?;
                Ok((row.get::<_, String>(0)?, activity, dialogue_count))
            },
        )
        .optional()
        .map_err(|error| local_store_error("read Agent operation summary", error))?
        .map(|(applied_at, activity, dialogue_count)| {
            Ok::<AgentOperationSummary, String>(AgentOperationSummary {
                state: "applied".to_string(),
                applied_at,
                activity: activity_from_db(&activity)?,
                dialogue_count: usize::try_from(dialogue_count).unwrap_or(0),
            })
        })
        .transpose()?;

    Ok(MemoryState {
        store_schema_version: STORE_SCHEMA_VERSION,
        source_count,
        event_count,
        signal_count,
        marks,
        active_source,
        agent_operation,
    })
}

fn creature_render_state_from_connection(
    connection: &Connection,
) -> Result<CreatureRenderState, String> {
    let approved_adapter = connection
        .query_row(
            "SELECT adapter_id FROM source_imports ORDER BY source_id LIMIT 1",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|error| local_store_error("read render-safe import state", error))?;
    let import_state = match approved_adapter.as_deref() {
        None => ImportState::Empty,
        Some(CODEX_MEMORY_ADAPTER_ID) => ImportState::AgentMemoryApproved,
        Some(CODEX_THREAD_ADAPTER_ID) => ImportState::ThreadApproved,
        Some(_) => ImportState::FixtureApproved,
    };
    // Pre-v0.6 effects remain readable for migration and deletion, but they no longer
    // shape the primary pet. Only an Agent Operation Protocol package can do that.
    let mut marks: Vec<CreatureRenderMark> = Vec::new();
    let import_revision_value = match import_state {
        ImportState::Empty => "empty",
        ImportState::FixtureApproved => "fixture-approved",
        ImportState::ThreadApproved => "thread-approved",
        ImportState::AgentMemoryApproved => "agent-memory-approved",
    };
    let operation = connection
        .query_row(
            "SELECT operation_id, dominant_activity, journey_state
             FROM agent_operations ORDER BY applied_at DESC LIMIT 1",
            [],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        )
        .optional()
        .map_err(|error| local_store_error("read render-safe Agent operation", error))?;
    let (operation_revision_value, agent_operation_state, agent_activity) = match &operation {
        Some((operation_id, activity, journey_state)) => {
            if journey_state == "milestone"
                && !marks
                    .iter()
                    .any(|mark| mark.style == CreatureRenderMarkStyle::CompletionStar)
            {
                marks.push(CreatureRenderMark {
                    id: "mark-operation".to_string(),
                    style: CreatureRenderMarkStyle::CompletionStar,
                });
            }
            (
                operation_id.as_str(),
                AgentOperationRenderState::Applied,
                activity_from_db(activity)?,
            )
        }
        None => (
            "empty",
            AgentOperationRenderState::Empty,
            AgentActivity::Off,
        ),
    };
    let dialogue = connection
        .query_row(
            "SELECT adc.dialogue_id, adc.text_en, adc.text_zh_tw, adc.trigger_kind
             FROM agent_operation_runtime aor
             JOIN agent_dialogue_cards adc ON adc.dialogue_id = aor.current_dialogue_id
             WHERE aor.singleton_id = 1",
            [],
            |row| {
                Ok(PetDialogue {
                    id: row.get(0)?,
                    text_en: row.get(1)?,
                    text_zh_tw: row.get(2)?,
                    trigger: row.get(3)?,
                })
            },
        )
        .optional()
        .map_err(|error| local_store_error("read render-safe Agent dialogue", error))?;
    let marks_revision_value = marks
        .iter()
        .map(|mark| match mark.style {
            CreatureRenderMarkStyle::CompletionStar => "completion-star",
        })
        .collect::<Vec<_>>()
        .join("+");
    let activity_revision_value = activity_to_db(agent_activity);
    let dialogue_revision_value = dialogue.as_ref().map_or("none", |item| item.id.as_str());
    let daily_scout_state = DailyScoutRenderState::Off;
    let real_memory_access = RealMemoryAccess::Off;
    let revision = Sha256::digest(
        format!(
            "6|off|{import_revision_value}|{operation_revision_value}|{activity_revision_value}|{dialogue_revision_value}|compact|seed|memory-seed-egg-v1|violet-mint|calm|{marks_revision_value}|scout-off"
        )
        .as_bytes(),
    )
    .iter()
    .map(|byte| format!("{byte:02x}"))
    .collect();

    Ok(CreatureRenderState {
        schema_version: 6,
        revision,
        real_memory_access,
        import_state,
        envelope: CreatureEnvelope::Compact,
        stage: CreatureStage::Seed,
        body_module: BodyModule::MemorySeedEggV1,
        palette: CreaturePalette::VioletMint,
        motion: CreatureMotion::Calm,
        daily_scout_state,
        marks,
        agent_operation_state,
        agent_activity,
        dialogue,
    })
}

fn activity_to_db(activity: AgentActivity) -> &'static str {
    match activity {
        AgentActivity::Off => "off",
        AgentActivity::Building => "building",
        AgentActivity::Research => "research",
        AgentActivity::Design => "design",
        AgentActivity::Planning => "planning",
        AgentActivity::Debugging => "debugging",
        AgentActivity::Writing => "writing",
        AgentActivity::Coordination => "coordination",
        AgentActivity::Shipping => "shipping",
    }
}

fn activity_from_db(activity: &str) -> Result<AgentActivity, String> {
    match activity {
        "building" => Ok(AgentActivity::Building),
        "research" => Ok(AgentActivity::Research),
        "design" => Ok(AgentActivity::Design),
        "planning" => Ok(AgentActivity::Planning),
        "debugging" => Ok(AgentActivity::Debugging),
        "writing" => Ok(AgentActivity::Writing),
        "coordination" => Ok(AgentActivity::Coordination),
        "shipping" => Ok(AgentActivity::Shipping),
        _ => Err("The Agent activity profile is invalid.".to_string()),
    }
}

fn count(connection: &Connection, table: &str) -> Result<usize, String> {
    let query = format!("SELECT COUNT(*) FROM {table}");
    let count = connection
        .query_row(&query, [], |row| row.get::<_, i64>(0))
        .map_err(|error| local_store_error("count local records", error))?;
    usize::try_from(count).map_err(|_| "The local record count is invalid.".to_string())
}

fn local_store_error(context: &str, error: rusqlite::Error) -> String {
    let _ = error;
    format!("Memoryling could not {context} in its local store.")
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        sync::{Arc, Barrier},
        thread,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;
    use crate::memory::{
        adapter::{consent_scope_contract, prepare_import, preview_source},
        agent_operation, codex_memory,
        model::{ConsentScopeV1, CreatureRenderMarkStyle, CODEX_THREAD_ADAPTER_VERSION},
    };

    fn fixture_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/codex-first-memory-v1.json")
    }

    fn temporary_store() -> (MemoryStore, PathBuf) {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();
        let directory =
            std::env::temp_dir().join(format!("memoryling-tests-{}-{nonce}", std::process::id()));
        (
            MemoryStore::new(directory.join("memoryling.sqlite3")),
            directory,
        )
    }

    fn thread_prepared() -> PreparedImport {
        let mut prepared = prepare_import("codex.synthetic.first-memory", &fixture_path())
            .expect("fixture should provide a synthetic normalized event");
        prepared.source.id = "source_thread_opaque".to_string();
        prepared.source.adapter_id = CODEX_THREAD_ADAPTER_ID.to_string();
        prepared.source.adapter_version = CODEX_THREAD_ADAPTER_VERSION;
        prepared.source.display_name = "Codex work record · 2026-08-12".to_string();
        prepared.source.locator = "codex-app-server://source_opaque".to_string();
        prepared.source.fixture_only = false;
        prepared.events[0].source_id = prepared.source.id.clone();
        prepared.events[0].source_record_id = "record_opaque".to_string();
        prepared.events[0].id = "memory_opaque".to_string();
        prepared.consent_scope = ConsentScopeV1 {
            schema_version: 1,
            revision: 1,
            source_id: prepared.source.id.clone(),
            adapter_id: CODEX_THREAD_ADAPTER_ID.to_string(),
            adapter_version: CODEX_THREAD_ADAPTER_VERSION,
            data_categories: vec!["user-confirmed-completion".to_string()],
            purposes: vec!["local-creature-derivation".to_string()],
            read_only: true,
            source_locator_hash: None,
            automatic_sync: false,
        };
        let (json, hash) = consent_scope_contract(&prepared.consent_scope)
            .expect("thread consent scope should serialize");
        prepared.consent_scope_json = json;
        prepared.consent_scope_hash = hash;
        prepared
    }

    #[test]
    fn preview_does_not_create_a_database() {
        let (store, directory) = temporary_store();
        let database_path = store.path.clone();

        let preview = preview_source("codex.synthetic.first-memory", &fixture_path())
            .expect("preview should work")
            .0;
        assert_eq!(preview.record_count, 1);
        assert!(!database_path.exists());

        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn concurrent_first_open_migrates_once_and_preserves_data() {
        let (store, directory) = temporary_store();
        let database_path = store.path.clone();
        let worker_count = 8;
        let barrier = Arc::new(Barrier::new(worker_count));
        let workers = (0..worker_count)
            .map(|_| {
                let path = database_path.clone();
                let barrier = Arc::clone(&barrier);
                thread::spawn(move || {
                    barrier.wait();
                    MemoryStore::new(path).state()
                })
            })
            .collect::<Vec<_>>();

        for worker in workers {
            let state = worker
                .join()
                .expect("concurrent first-open worker should not panic")
                .expect("every concurrent first open should migrate or observe the migration");
            assert_eq!(state.store_schema_version, STORE_SCHEMA_VERSION);
            assert_eq!(state.source_count, 0);
            assert_eq!(state.event_count, 0);
        }

        let prepared = prepare_import("codex.synthetic.first-memory", &fixture_path())
            .expect("fixture should parse after concurrent migration");
        let approved = store
            .approve_import(&prepared, &[prepared.events[0].source_record_id.clone()])
            .expect("data should remain writable after concurrent migration");
        let reopened = MemoryStore::new(database_path)
            .state()
            .expect("migrated database should reopen with approved data");
        assert_eq!(reopened, approved);
        assert_eq!(reopened.source_count, 1);
        assert_eq!(reopened.event_count, 1);
        assert_eq!(reopened.signal_count, 1);
        assert_eq!(reopened.marks.len(), 1);

        fs::remove_dir_all(directory).expect("temporary store should be removable");
    }

    #[test]
    fn approval_persists_lineage_and_forgetting_removes_every_effect() {
        let (store, directory) = temporary_store();
        let prepared = prepare_import("codex.synthetic.first-memory", &fixture_path())
            .expect("fixture should parse");
        let selected = vec![prepared.events[0].source_record_id.clone()];
        let fixture_bytes_before = fs::read(fixture_path()).expect("fixture should be readable");

        let empty_render = store
            .creature_render_state()
            .expect("empty render state should load");
        assert_eq!(empty_render.import_state, ImportState::Empty);
        assert!(empty_render.marks.is_empty());
        assert_eq!(empty_render.revision.len(), 64);
        assert!(empty_render
            .revision
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()));

        let approved = store
            .approve_import(&prepared, &selected)
            .expect("approval should persist");
        assert_eq!(approved.store_schema_version, STORE_SCHEMA_VERSION);
        assert_eq!(approved.source_count, 1);
        assert_eq!(approved.event_count, 1);
        assert_eq!(approved.signal_count, 1);
        assert_eq!(approved.marks.len(), 1);
        assert_eq!(approved.marks[0].style, "completion-star");
        assert_eq!(approved.marks[0].lineage.len(), 1);
        assert_eq!(approved.marks[0].lineage[0].source_record_id, selected[0]);

        let approved_render = store
            .creature_render_state()
            .expect("approved render state should load");
        assert_eq!(approved_render.import_state, ImportState::FixtureApproved);
        assert!(approved_render.marks.is_empty());
        assert_ne!(approved_render.revision, empty_render.revision);
        let pet_json =
            serde_json::to_string(&approved_render).expect("render state should serialize");
        for forbidden in [
            "memoryText",
            "sourceId",
            "sourceRecordId",
            "locator",
            "contentHash",
            "explanationKey",
        ] {
            assert!(!pet_json.contains(forbidden));
        }
        for private_value in [
            prepared.events[0].normalized_text.as_str(),
            prepared.events[0].source_id.as_str(),
            prepared.events[0].source_record_id.as_str(),
            prepared.events[0].content_hash.as_str(),
            prepared.source.display_name.as_str(),
            prepared.source.locator.as_str(),
        ] {
            assert!(!pet_json.contains(private_value));
        }

        let persisted_source = Connection::open(&store.path)
            .expect("store should reopen for schema assertion")
            .query_row(
                "SELECT adapter_id, adapter_version, source_content_hash FROM source_imports",
                [],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, i64>(1)?,
                        row.get::<_, String>(2)?,
                    ))
                },
            )
            .expect("source contract should be stored");
        assert_eq!(persisted_source.0, "codex-durable-memory");
        assert_eq!(persisted_source.1, 1);
        assert_eq!(persisted_source.2.len(), 64);
        let persisted_scope = Connection::open(&store.path)
            .expect("store should reopen for consent assertion")
            .query_row(
                "SELECT scope_json, scope_hash FROM source_consent_scopes",
                [],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
            )
            .expect("consent contract should be stored");
        assert_eq!(persisted_scope.0, prepared.consent_scope_json);
        assert_eq!(persisted_scope.1, prepared.consent_scope_hash);

        let reopened = MemoryStore::new(store.path.clone())
            .state()
            .expect("reopened store should load");
        assert_eq!(reopened, approved);

        let reimported = store
            .approve_import(&prepared, &selected)
            .expect("duplicate approval should replace idempotently");
        assert_eq!(reimported.source_count, 1);
        assert_eq!(reimported.event_count, 1);
        assert_eq!(reimported.marks[0].id, approved.marks[0].id);

        let forgotten = store
            .forget_source("codex.synthetic.first-memory")
            .expect("forgetting should complete");
        assert_eq!(forgotten.source_count, 0);
        assert_eq!(forgotten.event_count, 0);
        assert_eq!(forgotten.signal_count, 0);
        assert!(forgotten.marks.is_empty());
        let forgotten_render = store
            .creature_render_state()
            .expect("forgotten render state should load");
        assert_eq!(forgotten_render.import_state, ImportState::Empty);
        assert_eq!(forgotten_render.revision, empty_render.revision);
        assert!(forgotten_render.marks.is_empty());
        let connection = Connection::open(&store.path).expect("forgotten store should reopen");
        for table in [
            "source_imports",
            "source_consent_scopes",
            "memory_events",
            "derived_signals",
            "derived_signal_sources",
            "world_effects",
            "world_effect_signals",
        ] {
            assert_eq!(
                count(&connection, table).expect("table should be countable"),
                0
            );
        }
        assert_eq!(
            fs::read(fixture_path()).expect("fixture should remain readable"),
            fixture_bytes_before
        );
        drop(connection);

        fs::remove_dir_all(directory).expect("temporary store should be removable");
    }

    #[test]
    fn agent_memory_approval_sync_missing_recovery_and_forgetting_are_atomic() {
        let (store, directory) = temporary_store();
        let memory_root = directory.join("synthetic-codex-home").join("memories");
        fs::create_dir_all(&memory_root).expect("synthetic memory root should exist");
        fs::write(
            memory_root.join("memory_summary.md"),
            "# Synthetic summary\nNo private data.",
        )
        .expect("synthetic summary should be written");
        fs::write(
            memory_root.join("MEMORY.md"),
            "# Synthetic registry\n- initial fixture",
        )
        .expect("synthetic registry should be written");
        let prepared = codex_memory::prepare_import_at(&memory_root)
            .expect("synthetic Agent memories should parse");
        let selected = prepared
            .events
            .iter()
            .map(|event| event.source_record_id.clone())
            .collect::<Vec<_>>();

        let partial_error = store
            .approve_import(&prepared, &selected[..1])
            .expect_err("Agent memory files cannot be approved individually");
        assert!(partial_error.contains("complete allowlisted source"));

        let approved = store
            .approve_import(&prepared, &selected)
            .expect("complete Agent memory scope should be approved");
        assert_eq!(approved.event_count, 2);
        assert_eq!(approved.signal_count, 1);
        assert_eq!(approved.marks[0].style, "memory-halo");
        assert_eq!(approved.marks[0].lineage.len(), 2);
        assert!(approved.marks[0]
            .lineage
            .iter()
            .all(|source| source.content_redacted && source.memory_text.is_none()));
        let active = approved
            .active_source
            .as_ref()
            .expect("active source should be visible");
        assert!(active.automatic_sync);
        assert_eq!(active.sync_status, "synced");
        assert_eq!(active.synced_record_count, 2);
        let render = store
            .creature_render_state()
            .expect("render-safe Agent state should load");
        assert_eq!(render.real_memory_access, RealMemoryAccess::Off);
        assert_eq!(render.import_state, ImportState::AgentMemoryApproved);
        assert!(render.marks.is_empty());

        let old_hash = approved.marks[0].lineage[0].content_hash.clone();
        fs::write(
            memory_root.join("MEMORY.md"),
            "# Synthetic registry\n- updated fixture\n- another safe entry",
        )
        .expect("synthetic registry should update");
        let updated = codex_memory::prepare_import_at(&memory_root)
            .expect("updated Agent memories should parse");
        let synced = store
            .sync_agent_memory(&updated)
            .expect("approved Agent memories should sync");
        assert_eq!(synced.event_count, 2);
        assert!(synced.marks[0]
            .lineage
            .iter()
            .any(|source| source.content_hash != old_hash));

        fs::remove_dir_all(&memory_root).expect("exact synthetic root should be removable");
        let missing = store
            .record_agent_sync_failure("source-missing", "source-missing", true)
            .expect("missing source should withdraw downstream effects");
        assert_eq!(missing.source_count, 1);
        assert_eq!(missing.event_count, 0);
        assert!(missing.marks.is_empty());
        assert_eq!(missing.active_source.unwrap().sync_status, "source-missing");
        assert_eq!(
            store
                .creature_render_state()
                .expect("missing Agent source render state should load")
                .real_memory_access,
            RealMemoryAccess::Off
        );

        fs::create_dir_all(&memory_root).expect("synthetic memory root should recover");
        fs::write(
            memory_root.join("MEMORY.md"),
            "# Recovered synthetic registry",
        )
        .expect("recovered fixture should be written");
        let recovered = codex_memory::prepare_import_at(&memory_root)
            .expect("recovered Agent memory should parse");
        let recovered_state = store
            .sync_agent_memory(&recovered)
            .expect("same approved root should recover automatically");
        assert_eq!(recovered_state.event_count, 1);
        assert_eq!(recovered_state.marks[0].style, "memory-halo");

        let forgotten = store
            .forget_source(&prepared.source.id)
            .expect("disconnect should remove only Memoryling's local source");
        assert_eq!(forgotten.source_count, 0);
        assert!(forgotten.active_source.is_none());
        assert!(memory_root.join("MEMORY.md").is_file());

        fs::remove_dir_all(directory).expect("temporary store should be removable");
    }

    #[test]
    fn agent_operation_persists_bounded_render_state_and_dialogue_lineage() {
        let (store, directory) = temporary_store();
        let package = agent_operation::synthetic_package();

        let empty = store
            .creature_render_state()
            .expect("empty Agent operation state should load");
        assert_eq!(empty.schema_version, 6);
        assert_eq!(
            empty.agent_operation_state,
            AgentOperationRenderState::Empty
        );
        assert_eq!(empty.agent_activity, AgentActivity::Off);
        assert!(empty.dialogue.is_none());

        store
            .apply_agent_operation(&package)
            .expect("synthetic Agent operation should apply");
        let state = store.state().expect("Agent operation summary should load");
        let operation = state
            .agent_operation
            .expect("Agent operation summary should be present");
        assert_eq!(operation.state, "applied");
        assert_eq!(operation.activity, AgentActivity::Building);
        assert_eq!(operation.dialogue_count, 3);

        let render = store
            .creature_render_state()
            .expect("render-safe Agent operation should load");
        assert_eq!(
            render.agent_operation_state,
            AgentOperationRenderState::Applied
        );
        assert_eq!(render.agent_activity, AgentActivity::Building);
        assert_eq!(render.dialogue.as_ref().unwrap().id, "dialogue-1");
        assert!(render
            .marks
            .iter()
            .any(|mark| mark.style == CreatureRenderMarkStyle::CompletionStar));
        let serialized = serde_json::to_string(&render).expect("render state should serialize");
        assert!(!serialized.contains(&package.source_digest));
        assert!(!serialized.contains(&package.evidence[0].reference_hash));
        assert!(!serialized.contains("evidence.repo"));

        let interacted = store
            .advance_agent_dialogue("on-interact")
            .expect("interaction dialogue should advance");
        assert_eq!(interacted.dialogue.as_ref().unwrap().id, "dialogue-2");
        assert_ne!(interacted.revision, render.revision);

        store
            .apply_agent_operation(&package)
            .expect("an identical operation should be idempotent");
        let mut reused_id = package.clone();
        reused_id.source_digest = "c".repeat(64);
        assert!(store.apply_agent_operation(&reused_id).is_err());

        let reopened = MemoryStore::new(store.path.clone())
            .creature_render_state()
            .expect("Agent operation should survive restart");
        assert_eq!(
            reopened.agent_operation_state,
            AgentOperationRenderState::Applied
        );
        assert_eq!(reopened.dialogue.as_ref().unwrap().id, "dialogue-2");

        let mut replacement = package.clone();
        replacement.operation_id = "operation.synthetic-002".to_string();
        replacement.source_digest = "d".repeat(64);
        store
            .apply_agent_operation(&replacement)
            .expect("a new authoritative operation should replace the prior package");
        let connection = Connection::open(&store.path).expect("operation store should reopen");
        assert_eq!(count(&connection, "agent_operations").unwrap(), 1);
        assert_eq!(count(&connection, "agent_dialogue_cards").unwrap(), 3);
        drop(connection);

        let cleared = store
            .clear_agent_operations()
            .expect("derived Agent operation should be clearable");
        assert!(cleared.agent_operation.is_none());
        let cleared_render = store
            .creature_render_state()
            .expect("cleared Agent operation render state should load");
        assert_eq!(
            cleared_render.agent_operation_state,
            AgentOperationRenderState::Empty
        );
        assert!(cleared_render.dialogue.is_none());

        fs::remove_dir_all(directory).expect("temporary store should be removable");
    }

    #[test]
    fn invalid_approval_writes_nothing() {
        let (store, directory) = temporary_store();
        let prepared = prepare_import("codex.synthetic.first-memory", &fixture_path())
            .expect("fixture should parse");

        let render_before = store
            .creature_render_state()
            .expect("empty render state should load");
        assert!(store.approve_import(&prepared, &[]).is_err());
        assert!(store
            .approve_import(&prepared, &["unknown-record".to_string()])
            .is_err());

        let state = store.state().expect("empty state should load");
        assert_eq!(state.source_count, 0);
        assert_eq!(state.event_count, 0);
        assert!(state.marks.is_empty());
        assert_eq!(
            store
                .creature_render_state()
                .expect("failed approval should preserve render state"),
            render_before
        );

        fs::remove_dir_all(directory).expect("temporary store should be removable");
    }

    #[test]
    fn external_thread_lineage_is_redacted_and_a_second_source_fails_closed() {
        let (store, directory) = temporary_store();
        let prepared = thread_prepared();
        let selected = vec![prepared.events[0].source_record_id.clone()];
        let raw_completion = prepared.events[0].normalized_text.clone();

        let state = store
            .approve_import(&prepared, &selected)
            .expect("one thread source should be approved");
        let lineage = &state.marks[0].lineage[0];
        assert!(lineage.memory_text.is_none());
        assert!(lineage.content_redacted);
        assert_eq!(lineage.character_count, raw_completion.chars().count());
        assert_eq!(
            lineage.consent_scope_hash.as_deref(),
            Some(prepared.consent_scope_hash.as_str())
        );
        assert_eq!(lineage.consent_revision, Some(1));
        assert!(!serde_json::to_string(&state)
            .expect("state should serialize")
            .contains(&raw_completion));

        let render = store
            .creature_render_state()
            .expect("thread render state should load");
        assert_eq!(render.import_state, ImportState::ThreadApproved);
        assert!(!serde_json::to_string(&render)
            .expect("render state should serialize")
            .contains(&raw_completion));

        let fixture = prepare_import("codex.synthetic.first-memory", &fixture_path())
            .expect("fixture should parse");
        let error = store
            .approve_import(&fixture, &[fixture.events[0].source_record_id.clone()])
            .expect_err("a different source must require forgetting first");
        assert_eq!(
            error,
            "Forget the currently approved source before importing a different one."
        );
        assert_eq!(store.state().expect("original source should remain"), state);

        fs::remove_dir_all(directory).expect("temporary store should be removable");
    }

    #[test]
    fn migration_is_idempotent_and_future_schemas_fail_closed() {
        let (store, directory) = temporary_store();

        assert_eq!(
            store
                .state()
                .expect("first migration should work")
                .store_schema_version,
            STORE_SCHEMA_VERSION
        );

        let v1_path = directory.join("v1.sqlite3");
        let v1_connection = Connection::open(&v1_path).expect("v1 database should open");
        v1_connection
            .execute_batch(MIGRATION_0001)
            .expect("v1 schema should be created");
        drop(v1_connection);
        assert_eq!(
            MemoryStore::new(v1_path)
                .state()
                .expect("v1 schema should migrate")
                .store_schema_version,
            STORE_SCHEMA_VERSION
        );

        let populated_v1_path = directory.join("populated-v1.sqlite3");
        let populated_v1 =
            Connection::open(&populated_v1_path).expect("populated v1 database should open");
        populated_v1
            .execute_batch(MIGRATION_0001)
            .expect("populated v1 schema should be created");
        populated_v1
            .execute(
                "INSERT INTO source_imports
                    (source_id, adapter_id, adapter_version, display_name, source_locator,
                     source_content_hash)
                 VALUES (?1, ?2, 1, 'Synthetic fixture', 'resource://fixture', ?3)",
                params![
                    "codex.synthetic.first-memory",
                    "codex-durable-memory",
                    "a".repeat(64)
                ],
            )
            .expect("legacy fixture source should be inserted");
        drop(populated_v1);
        let populated_store = MemoryStore::new(populated_v1_path.clone());
        assert_eq!(
            populated_store
                .state()
                .expect("populated fixture v1 should migrate")
                .source_count,
            1
        );
        assert_eq!(
            count(
                &Connection::open(populated_v1_path)
                    .expect("migrated populated database should open"),
                "source_consent_scopes"
            )
            .expect("backfilled scope should be countable"),
            1
        );

        let unsupported_v1_path = directory.join("unsupported-v1.sqlite3");
        let unsupported_v1 =
            Connection::open(&unsupported_v1_path).expect("unsupported v1 database should open");
        unsupported_v1
            .execute_batch(MIGRATION_0001)
            .expect("unsupported v1 schema should be created");
        unsupported_v1
            .execute(
                "INSERT INTO source_imports
                    (source_id, adapter_id, adapter_version, display_name, source_locator,
                     source_content_hash)
                 VALUES ('unexpected', 'unexpected', 1, 'Unexpected', 'opaque', ?1)",
                params!["b".repeat(64)],
            )
            .expect("unsupported legacy source should be inserted");
        drop(unsupported_v1);
        assert_eq!(
            MemoryStore::new(unsupported_v1_path)
                .state()
                .expect_err("unsupported legacy source must require fresh consent"),
            "The legacy local source requires a fresh explicit consent."
        );
        assert_eq!(
            store
                .state()
                .expect("second open should work")
                .store_schema_version,
            STORE_SCHEMA_VERSION
        );

        let future_path = directory.join("future.sqlite3");
        let connection = Connection::open(&future_path).expect("future test database should open");
        connection
            .pragma_update(None, "user_version", 99)
            .expect("future version should be set");
        drop(connection);

        let error = MemoryStore::new(future_path)
            .state()
            .expect_err("future schema should fail closed");
        assert_eq!(
            error,
            "The local Memoryling database schema is not supported."
        );

        fs::remove_dir_all(directory).expect("temporary store should be removable");
    }
}
