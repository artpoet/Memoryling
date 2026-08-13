CREATE TABLE memory_events_v4 (
    id TEXT PRIMARY KEY NOT NULL,
    schema_version INTEGER NOT NULL CHECK (schema_version > 0),
    source_id TEXT NOT NULL REFERENCES source_imports(source_id) ON DELETE CASCADE,
    source_record_id TEXT NOT NULL,
    source_timestamp TEXT NOT NULL,
    observed_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    kind TEXT NOT NULL CHECK (kind IN ('completion', 'agent-memory-document')),
    normalized_text TEXT NOT NULL CHECK (length(trim(normalized_text)) > 0),
    content_hash TEXT NOT NULL CHECK (length(content_hash) = 64),
    UNIQUE (source_id, source_record_id)
);

CREATE TABLE derived_signals_v4 (
    id TEXT PRIMARY KEY NOT NULL,
    signal_type TEXT NOT NULL CHECK (signal_type IN ('completion', 'agent-memory-continuity')),
    confidence REAL NOT NULL CHECK (confidence >= 0.0 AND confidence <= 1.0),
    derivation_version INTEGER NOT NULL CHECK (derivation_version > 0),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE derived_signal_sources_v4 (
    signal_id TEXT NOT NULL REFERENCES derived_signals_v4(id) ON DELETE CASCADE,
    memory_event_id TEXT NOT NULL REFERENCES memory_events_v4(id) ON DELETE CASCADE,
    PRIMARY KEY (signal_id, memory_event_id)
);

CREATE TABLE world_effects_v4 (
    id TEXT PRIMARY KEY NOT NULL,
    effect_type TEXT NOT NULL CHECK (effect_type IN ('visual-mark')),
    effect_style TEXT NOT NULL CHECK (effect_style IN ('completion-star', 'memory-halo')),
    state TEXT NOT NULL CHECK (state IN ('active')),
    explanation_key TEXT NOT NULL,
    derivation_version INTEGER NOT NULL CHECK (derivation_version > 0),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE world_effect_signals_v4 (
    effect_id TEXT NOT NULL REFERENCES world_effects_v4(id) ON DELETE CASCADE,
    signal_id TEXT NOT NULL REFERENCES derived_signals_v4(id) ON DELETE CASCADE,
    PRIMARY KEY (effect_id, signal_id)
);

CREATE TABLE source_consent_scopes_v4 (
    source_id TEXT PRIMARY KEY NOT NULL REFERENCES source_imports(source_id) ON DELETE CASCADE,
    schema_version INTEGER NOT NULL CHECK (schema_version BETWEEN 1 AND 2),
    consent_revision INTEGER NOT NULL CHECK (consent_revision >= 1),
    scope_json TEXT NOT NULL,
    scope_hash TEXT NOT NULL CHECK (length(scope_hash) = 64),
    consented_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

INSERT INTO memory_events_v4 SELECT * FROM memory_events;
INSERT INTO derived_signals_v4 SELECT * FROM derived_signals;
INSERT INTO derived_signal_sources_v4 SELECT * FROM derived_signal_sources;
INSERT INTO world_effects_v4 SELECT * FROM world_effects;
INSERT INTO world_effect_signals_v4 SELECT * FROM world_effect_signals;
INSERT INTO source_consent_scopes_v4 SELECT * FROM source_consent_scopes;

DROP TABLE world_effect_signals;
DROP TABLE world_effects;
DROP TABLE derived_signal_sources;
DROP TABLE derived_signals;
DROP TABLE memory_events;

ALTER TABLE memory_events_v4 RENAME TO memory_events;
ALTER TABLE derived_signals_v4 RENAME TO derived_signals;
ALTER TABLE derived_signal_sources_v4 RENAME TO derived_signal_sources;
ALTER TABLE world_effects_v4 RENAME TO world_effects;
ALTER TABLE world_effect_signals_v4 RENAME TO world_effect_signals;

DROP TABLE source_consent_scopes;
ALTER TABLE source_consent_scopes_v4 RENAME TO source_consent_scopes;

CREATE TABLE source_sync_state (
    source_id TEXT PRIMARY KEY NOT NULL REFERENCES source_imports(source_id) ON DELETE CASCADE,
    automatic_sync INTEGER NOT NULL CHECK (automatic_sync IN (0, 1)),
    sync_status TEXT NOT NULL CHECK (sync_status IN ('synced', 'source-missing', 'needs-attention')),
    last_attempt_at TEXT NOT NULL,
    last_successful_sync_at TEXT,
    synced_record_count INTEGER NOT NULL CHECK (synced_record_count >= 0),
    error_code TEXT
);

CREATE INDEX memory_events_source_id_idx ON memory_events(source_id);
CREATE INDEX derived_signal_sources_event_idx ON derived_signal_sources(memory_event_id);
CREATE INDEX world_effect_signals_signal_idx ON world_effect_signals(signal_id);

PRAGMA user_version = 4;
