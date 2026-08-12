use std::{collections::HashSet, fs, path::PathBuf, time::Duration};

use rusqlite::{params, Connection, Transaction, TransactionBehavior};
use sha2::{Digest, Sha256};

use super::{
    adapter::stable_id,
    model::{
        BodyModule, CreatureEnvelope, CreatureMark, CreatureMotion, CreaturePalette,
        CreatureRenderMark, CreatureRenderMarkStyle, CreatureRenderState, FixtureState,
        LineageSource, MemoryState, PreparedImport, RealMemoryAccess, DERIVATION_VERSION,
        STORE_SCHEMA_VERSION,
    },
};

const MIGRATION_0001: &str = include_str!("../../migrations/0001_first_memory.sql");

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

    fn open_connection(&self) -> Result<Connection, String> {
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
                .execute_batch(MIGRATION_0001)
                .map_err(|error| local_store_error("apply schema migration", error))?,
            STORE_SCHEMA_VERSION => {}
            _ => return Err("The local Memoryling database schema is not supported.".to_string()),
        }
        migration
            .commit()
            .map_err(|error| local_store_error("commit schema migration", error))?;

        Ok(connection)
    }
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
                            me.source_timestamp, me.normalized_text, me.content_hash
                     FROM world_effect_signals wes
                     JOIN derived_signal_sources dss ON dss.signal_id = wes.signal_id
                     JOIN memory_events me ON me.id = dss.memory_event_id
                     JOIN source_imports si ON si.source_id = me.source_id
                     WHERE wes.effect_id = ?1
                     ORDER BY me.id",
                )
                .map_err(|error| local_store_error("prepare effect lineage", error))?;
            let rows = statement
                .query_map(params![effect_id], |row| {
                    Ok(LineageSource {
                        memory_event_id: row.get(0)?,
                        memory_event_schema_version: row.get(1)?,
                        source_id: row.get(2)?,
                        source_label: row.get(3)?,
                        adapter_id: row.get(4)?,
                        adapter_version: row.get(5)?,
                        source_record_id: row.get(6)?,
                        source_timestamp: row.get(7)?,
                        memory_text: row.get(8)?,
                        content_hash: row.get(9)?,
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
    let fixture_state = if count(connection, "source_imports")? == 0 {
        FixtureState::Empty
    } else {
        FixtureState::Approved
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
    let fixture_revision_value = match fixture_state {
        FixtureState::Empty => "empty",
        FixtureState::Approved => "approved",
    };
    let marks_revision_value = if marks.is_empty() {
        "none"
    } else {
        "completion-star"
    };
    let revision = Sha256::digest(
        format!(
            "1|off|{fixture_revision_value}|compact|baseline|violet-mint|calm|{marks_revision_value}"
        )
        .as_bytes(),
    )
    .iter()
    .map(|byte| format!("{byte:02x}"))
    .collect();

    Ok(CreatureRenderState {
        schema_version: 1,
        revision,
        real_memory_access: RealMemoryAccess::Off,
        fixture_state,
        envelope: CreatureEnvelope::Compact,
        body_module: BodyModule::Baseline,
        palette: CreaturePalette::VioletMint,
        motion: CreatureMotion::Calm,
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
    use crate::memory::adapter::{prepare_import, preview_source};

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
        assert_eq!(empty_render.fixture_state, FixtureState::Empty);
        assert!(empty_render.marks.is_empty());
        assert_eq!(empty_render.revision.len(), 64);
        assert!(empty_render
            .revision
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()));

        let approved = store
            .approve_import(&prepared, &selected)
            .expect("approval should persist");
        assert_eq!(approved.store_schema_version, 1);
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
        assert_eq!(approved_render.fixture_state, FixtureState::Approved);
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
        assert_eq!(forgotten_render.fixture_state, FixtureState::Empty);
        assert_eq!(forgotten_render.revision, empty_render.revision);
        assert!(forgotten_render.marks.is_empty());
        let connection = Connection::open(&store.path).expect("forgotten store should reopen");
        for table in [
            "source_imports",
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
    fn migration_is_idempotent_and_future_schemas_fail_closed() {
        let (store, directory) = temporary_store();

        assert_eq!(
            store
                .state()
                .expect("first migration should work")
                .store_schema_version,
            1
        );
        assert_eq!(
            store
                .state()
                .expect("second open should work")
                .store_schema_version,
            1
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
