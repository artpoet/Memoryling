CREATE TABLE source_consent_scopes (
    source_id TEXT PRIMARY KEY NOT NULL REFERENCES source_imports(source_id) ON DELETE CASCADE,
    schema_version INTEGER NOT NULL CHECK (schema_version = 1),
    consent_revision INTEGER NOT NULL CHECK (consent_revision = 1),
    scope_json TEXT NOT NULL,
    scope_hash TEXT NOT NULL CHECK (length(scope_hash) = 64),
    consented_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

PRAGMA user_version = 2;
