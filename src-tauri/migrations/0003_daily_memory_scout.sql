CREATE TABLE daily_scout_settings (
    singleton_id INTEGER PRIMARY KEY CHECK (singleton_id = 1),
    enabled INTEGER NOT NULL CHECK (enabled IN (0, 1)),
    provider TEXT NOT NULL CHECK (provider = 'openai'),
    model TEXT NOT NULL,
    locale TEXT NOT NULL CHECK (locale IN ('en', 'zh-TW')),
    delivery_hour INTEGER NOT NULL CHECK (delivery_hour BETWEEN 8 AND 21),
    delivery_minute INTEGER NOT NULL CHECK (delivery_minute BETWEEN 0 AND 59),
    consent_schema_version INTEGER NOT NULL,
    consent_revision INTEGER NOT NULL,
    consent_scope_json TEXT NOT NULL,
    consent_scope_hash TEXT NOT NULL CHECK (length(consent_scope_hash) = 64),
    updated_at TEXT NOT NULL
);

CREATE TABLE daily_search_attempts (
    local_date TEXT PRIMARY KEY,
    timezone TEXT NOT NULL,
    started_at TEXT NOT NULL,
    completed_at TEXT,
    status TEXT NOT NULL CHECK (status IN ('running', 'succeeded', 'failed')),
    context_hash TEXT NOT NULL CHECK (length(context_hash) = 64),
    error_code TEXT
);

CREATE TABLE daily_insights (
    id TEXT PRIMARY KEY,
    local_date TEXT NOT NULL UNIQUE REFERENCES daily_search_attempts(local_date) ON DELETE CASCADE,
    provider TEXT NOT NULL CHECK (provider = 'openai'),
    model TEXT NOT NULL,
    pet_message TEXT NOT NULL,
    strength TEXT NOT NULL CHECK (strength IN ('practical', 'quiet')),
    relevance_reason TEXT NOT NULL,
    searched_at TEXT NOT NULL,
    read_at TEXT
);

CREATE TABLE daily_insight_sources (
    insight_id TEXT NOT NULL REFERENCES daily_insights(id) ON DELETE CASCADE,
    source_id TEXT NOT NULL REFERENCES source_imports(source_id) ON DELETE CASCADE,
    PRIMARY KEY (insight_id, source_id)
);

CREATE TABLE daily_insight_citations (
    insight_id TEXT NOT NULL REFERENCES daily_insights(id) ON DELETE CASCADE,
    position INTEGER NOT NULL CHECK (position BETWEEN 0 AND 2),
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    PRIMARY KEY (insight_id, position)
);

CREATE INDEX idx_daily_insights_unread
    ON daily_insights(read_at, searched_at);

PRAGMA user_version = 3;
