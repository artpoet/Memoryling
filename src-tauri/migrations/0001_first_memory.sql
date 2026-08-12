CREATE TABLE source_imports (
    source_id TEXT PRIMARY KEY NOT NULL,
    adapter_id TEXT NOT NULL,
    adapter_version INTEGER NOT NULL CHECK (adapter_version > 0),
    display_name TEXT NOT NULL,
    source_locator TEXT NOT NULL,
    source_content_hash TEXT NOT NULL CHECK (length(source_content_hash) = 64),
    approved_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE memory_events (
    id TEXT PRIMARY KEY NOT NULL,
    schema_version INTEGER NOT NULL CHECK (schema_version > 0),
    source_id TEXT NOT NULL REFERENCES source_imports(source_id) ON DELETE CASCADE,
    source_record_id TEXT NOT NULL,
    source_timestamp TEXT NOT NULL,
    observed_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    kind TEXT NOT NULL CHECK (kind IN ('completion')),
    normalized_text TEXT NOT NULL CHECK (length(trim(normalized_text)) > 0),
    content_hash TEXT NOT NULL CHECK (length(content_hash) = 64),
    UNIQUE (source_id, source_record_id)
);

CREATE TABLE derived_signals (
    id TEXT PRIMARY KEY NOT NULL,
    signal_type TEXT NOT NULL CHECK (signal_type IN ('completion')),
    confidence REAL NOT NULL CHECK (confidence >= 0.0 AND confidence <= 1.0),
    derivation_version INTEGER NOT NULL CHECK (derivation_version > 0),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE derived_signal_sources (
    signal_id TEXT NOT NULL REFERENCES derived_signals(id) ON DELETE CASCADE,
    memory_event_id TEXT NOT NULL REFERENCES memory_events(id) ON DELETE CASCADE,
    PRIMARY KEY (signal_id, memory_event_id)
);

CREATE TABLE world_effects (
    id TEXT PRIMARY KEY NOT NULL,
    effect_type TEXT NOT NULL CHECK (effect_type IN ('visual-mark')),
    effect_style TEXT NOT NULL CHECK (effect_style IN ('completion-star')),
    state TEXT NOT NULL CHECK (state IN ('active')),
    explanation_key TEXT NOT NULL,
    derivation_version INTEGER NOT NULL CHECK (derivation_version > 0),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE world_effect_signals (
    effect_id TEXT NOT NULL REFERENCES world_effects(id) ON DELETE CASCADE,
    signal_id TEXT NOT NULL REFERENCES derived_signals(id) ON DELETE CASCADE,
    PRIMARY KEY (effect_id, signal_id)
);

CREATE INDEX memory_events_source_id_idx ON memory_events(source_id);
CREATE INDEX derived_signal_sources_event_idx ON derived_signal_sources(memory_event_id);
CREATE INDEX world_effect_signals_signal_idx ON world_effect_signals(signal_id);

PRAGMA user_version = 1;
