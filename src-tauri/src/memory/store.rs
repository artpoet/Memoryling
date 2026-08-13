use std::{collections::HashSet, fs, path::PathBuf, time::Duration};

use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};
use sha2::{Digest, Sha256};

use super::{
    adapter::stable_id,
    model::{
        BodyModule, CreatureEnvelope, CreatureMark, CreatureMotion, CreaturePalette,
        CreatureRenderMark, CreatureRenderMarkStyle, CreatureRenderState, DailyScoutRenderState,
        ImportState, LineageSource, MemoryState, PreparedImport, RealMemoryAccess,
        CODEX_THREAD_ADAPTER_ID, DERIVATION_VERSION, STORE_SCHEMA_VERSION,
    },
};

const MIGRATION_0001: &str = include_str!("../../migrations/0001_first_memory.sql");
const MIGRATION_0002: &str = include_str!("../../migrations/0002_source_consent_scope.sql");
const MIGRATION_0003: &str = include_str!("../../migrations/0003_daily_memory_scout.sql");

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
                    "{MIGRATION_0001}\n{MIGRATION_0002}\n{MIGRATION_0003}"
                ))
                .map_err(|error| local_store_error("apply schema migrations", error))?,
            1 => {
                migrate_v1_to_v2(&migration)?;
                migrate_v2_to_v3(&migration)?;
            }
            2 => migrate_v2_to_v3(&migration)?,
            STORE_SCHEMA_VERSION => {}
            _ => return Err("The local Memoryling database schema is not supported.".to_string()),
        }
        migration
            .commit()
            .map_err(|error| local_store_error("commit schema migration", error))?;

        Ok(connection)
    }
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

    for (event_id, kind) in inputs {
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
                    let content_redacted = adapter_id == CODEX_THREAD_ADAPTER_ID;
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

    Ok(MemoryState {
        store_schema_version: STORE_SCHEMA_VERSION,
        source_count,
        event_count,
        signal_count,
        marks,
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
        Some(CODEX_THREAD_ADAPTER_ID) => ImportState::ThreadApproved,
        Some(_) => ImportState::FixtureApproved,
    };
    let completion_star_count = connection
        .query_row(
            "SELECT COUNT(*)
             FROM world_effects
             WHERE state = 'active' AND effect_style = 'completion-star'",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| local_store_error("read render-safe creature state", error))?;

    let marks = if completion_star_count > 0 {
        vec![CreatureRenderMark {
            id: "mark-1".to_string(),
            style: CreatureRenderMarkStyle::CompletionStar,
        }]
    } else {
        Vec::new()
    };
    let import_revision_value = match import_state {
        ImportState::Empty => "empty",
        ImportState::FixtureApproved => "fixture-approved",
        ImportState::ThreadApproved => "thread-approved",
    };
    let marks_revision_value = if marks.is_empty() {
        "none"
    } else {
        "completion-star"
    };
    let daily_scout_enabled = connection
        .query_row(
            "SELECT enabled FROM daily_scout_settings WHERE singleton_id = 1",
            [],
            |row| row.get::<_, i64>(0),
        )
        .optional()
        .map_err(|error| local_store_error("read render-safe Daily Scout state", error))?
        == Some(1);
    let unread_insight_count = connection
        .query_row(
            "SELECT COUNT(*) FROM daily_insights WHERE read_at IS NULL",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|error| local_store_error("read render-safe Daily Scout message", error))?;
    let daily_scout_state = if unread_insight_count > 0 {
        DailyScoutRenderState::Ready
    } else if daily_scout_enabled {
        DailyScoutRenderState::Waiting
    } else {
        DailyScoutRenderState::Off
    };
    let daily_scout_revision_value = match daily_scout_state {
        DailyScoutRenderState::Off => "scout-off",
        DailyScoutRenderState::Waiting => "scout-waiting",
        DailyScoutRenderState::Ready => "scout-ready",
    };
    let revision = Sha256::digest(
        format!(
            "3|off|{import_revision_value}|compact|baseline|violet-mint|calm|{marks_revision_value}|{daily_scout_revision_value}"
        )
        .as_bytes(),
    )
    .iter()
    .map(|byte| format!("{byte:02x}"))
    .collect();

    Ok(CreatureRenderState {
        schema_version: 3,
        revision,
        real_memory_access: RealMemoryAccess::Off,
        import_state,
        envelope: CreatureEnvelope::Compact,
        body_module: BodyModule::Baseline,
        palette: CreaturePalette::VioletMint,
        motion: CreatureMotion::Calm,
        daily_scout_state,
        marks,
    })
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
        model::{ConsentScopeV1, CODEX_THREAD_ADAPTER_VERSION},
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
        assert_eq!(approved_render.marks.len(), 1);
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
