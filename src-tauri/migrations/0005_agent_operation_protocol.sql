CREATE TABLE agent_operations (
    operation_id TEXT PRIMARY KEY NOT NULL,
    schema_version INTEGER NOT NULL CHECK (schema_version = 1),
    generated_at TEXT NOT NULL,
    agent_family TEXT NOT NULL CHECK (agent_family IN ('codex', 'claude', 'other')),
    source_digest TEXT NOT NULL CHECK (length(source_digest) = 64),
    dominant_activity TEXT NOT NULL CHECK (
        dominant_activity IN (
            'building', 'research', 'design', 'planning',
            'debugging', 'writing', 'coordination', 'shipping'
        )
    ),
    secondary_activity TEXT CHECK (
        secondary_activity IS NULL OR secondary_activity IN (
            'building', 'research', 'design', 'planning',
            'debugging', 'writing', 'coordination', 'shipping'
        )
    ),
    journey_state TEXT NOT NULL CHECK (
        journey_state IN ('steady', 'exploring', 'milestone', 'recovering')
    ),
    applied_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE agent_operation_evidence (
    evidence_id TEXT PRIMARY KEY NOT NULL,
    operation_id TEXT NOT NULL REFERENCES agent_operations(operation_id) ON DELETE CASCADE,
    evidence_kind TEXT NOT NULL CHECK (
        evidence_kind IN ('durable-memory', 'recent-work', 'repo-ssot', 'current-thread')
    ),
    reference_hash TEXT NOT NULL CHECK (length(reference_hash) = 64),
    observed_at TEXT NOT NULL
);

CREATE TABLE agent_dialogue_cards (
    dialogue_id TEXT PRIMARY KEY NOT NULL,
    operation_id TEXT NOT NULL REFERENCES agent_operations(operation_id) ON DELETE CASCADE,
    text_en TEXT NOT NULL CHECK (length(trim(text_en)) BETWEEN 1 AND 240),
    text_zh_tw TEXT NOT NULL CHECK (length(trim(text_zh_tw)) BETWEEN 1 AND 240),
    trigger_kind TEXT NOT NULL CHECK (trigger_kind IN ('on-open', 'on-interact', 'ambient')),
    priority INTEGER NOT NULL CHECK (priority BETWEEN 0 AND 3),
    not_before TEXT,
    expires_at TEXT,
    cooldown_minutes INTEGER NOT NULL CHECK (cooldown_minutes BETWEEN 0 AND 10080),
    max_uses INTEGER NOT NULL CHECK (max_uses BETWEEN 1 AND 20),
    use_count INTEGER NOT NULL DEFAULT 0 CHECK (use_count >= 0),
    last_used_at TEXT
);

CREATE TABLE agent_dialogue_evidence (
    dialogue_id TEXT NOT NULL REFERENCES agent_dialogue_cards(dialogue_id) ON DELETE CASCADE,
    evidence_id TEXT NOT NULL REFERENCES agent_operation_evidence(evidence_id) ON DELETE CASCADE,
    PRIMARY KEY (dialogue_id, evidence_id)
);

CREATE TABLE agent_operation_runtime (
    singleton_id INTEGER PRIMARY KEY NOT NULL CHECK (singleton_id = 1),
    current_dialogue_id TEXT REFERENCES agent_dialogue_cards(dialogue_id) ON DELETE SET NULL,
    inbox_status TEXT NOT NULL CHECK (inbox_status IN ('waiting', 'applied', 'invalid')),
    inbox_error_code TEXT,
    last_inbox_checked_at TEXT
);

INSERT INTO agent_operation_runtime
    (singleton_id, current_dialogue_id, inbox_status, inbox_error_code, last_inbox_checked_at)
VALUES (1, NULL, 'waiting', NULL, NULL);

CREATE TABLE agent_dialogue_daily_usage (
    local_date TEXT PRIMARY KEY NOT NULL,
    ambient_count INTEGER NOT NULL CHECK (ambient_count BETWEEN 0 AND 2)
);

CREATE INDEX agent_operation_evidence_operation_idx
    ON agent_operation_evidence(operation_id);
CREATE INDEX agent_dialogue_cards_operation_idx
    ON agent_dialogue_cards(operation_id);
CREATE INDEX agent_dialogue_evidence_evidence_idx
    ON agent_dialogue_evidence(evidence_id);

PRAGMA user_version = 5;
