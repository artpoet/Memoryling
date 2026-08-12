use std::{fs, path::Path};

use serde::Deserialize;
use sha2::{Digest, Sha256};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use super::model::{
    AccessScope, ConsentScopeV1, ImportPreview, NormalizedMemoryEvent, PreparedImport,
    PreviewRecord, PreviewTimeRange, SourceOption, CODEX_ADAPTER_ID, CODEX_ADAPTER_VERSION,
    MEMORY_EVENT_SCHEMA_VERSION,
};

const SUPPORTED_SOURCE_ID: &str = "codex.synthetic.first-memory";
const SOURCE_LOCATOR: &str = "resource://fixtures/codex-first-memory-v1.json";
const MAX_FIXTURE_BYTES: u64 = 64 * 1024;

#[derive(Debug, Deserialize)]
struct CodexFixture {
    format: String,
    version: u32,
    source: FixtureSource,
    memories: Vec<FixtureMemory>,
}

#[derive(Debug, Deserialize)]
struct FixtureSource {
    id: String,
    label: String,
    locator: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureMemory {
    id: String,
    created_at: String,
    kind: String,
    text: String,
}

pub(crate) fn list_sources() -> Result<Vec<SourceOption>, String> {
    Ok(vec![source_option()])
}

pub(crate) fn ensure_supported_source(source_id: &str) -> Result<(), String> {
    if source_id == SUPPORTED_SOURCE_ID {
        Ok(())
    } else {
        Err("Unknown or unapproved memory source.".to_string())
    }
}

pub(crate) fn preview_source(
    source_id: &str,
    source_path: &Path,
) -> Result<(ImportPreview, PreparedImport), String> {
    let prepared = prepare_import(source_id, source_path)?;
    let mut timestamps = prepared
        .events
        .iter()
        .map(|event| event.source_timestamp.as_str())
        .collect::<Vec<_>>();
    timestamps.sort_unstable();

    let start = timestamps
        .first()
        .ok_or_else(|| "The selected source contains no importable records.".to_string())?;
    let end = timestamps
        .last()
        .ok_or_else(|| "The selected source contains no importable records.".to_string())?;

    let records = prepared
        .events
        .iter()
        .map(|event| PreviewRecord {
            id: event.source_record_id.clone(),
            source_timestamp: event.source_timestamp.clone(),
            kind: event.kind.clone(),
            text_preview: Some(event.normalized_text.clone()),
            character_count: event.normalized_text.chars().count(),
            content_hash: event.content_hash.clone(),
        })
        .collect::<Vec<_>>();

    let preview = ImportPreview {
        preview_id: String::new(),
        source: prepared.source.clone(),
        record_count: records.len(),
        time_range: PreviewTimeRange {
            start: (*start).to_string(),
            end: (*end).to_string(),
        },
        records,
        access_scope: AccessScope {
            read_only: true,
            source_write_access: false,
            network_access: false,
            arbitrary_path_access: false,
        },
        consent_scope: prepared.consent_scope.clone(),
        consent_scope_hash: prepared.consent_scope_hash.clone(),
    };

    Ok((preview, prepared))
}

pub(crate) fn prepare_import(
    source_id: &str,
    source_path: &Path,
) -> Result<PreparedImport, String> {
    let metadata = fs::metadata(source_path)
        .map_err(|_| "The bundled synthetic Codex source is unavailable.".to_string())?;
    if !metadata.is_file() || metadata.len() > MAX_FIXTURE_BYTES {
        return Err("The bundled synthetic Codex source is not an approved file.".to_string());
    }
    let raw = fs::read_to_string(source_path)
        .map_err(|_| "The bundled synthetic Codex source is not valid UTF-8.".to_string())?;
    parse_fixture(&raw, source_id)
}

fn parse_fixture(raw: &str, requested_source_id: &str) -> Result<PreparedImport, String> {
    ensure_supported_source(requested_source_id)?;

    let fixture: CodexFixture = serde_json::from_str(raw)
        .map_err(|_| "The selected Codex memory format is invalid.".to_string())?;

    if fixture.format != "codex-durable-memory" || fixture.version != 1 {
        return Err("Unsupported Codex memory format or version.".to_string());
    }
    if fixture.source.id != requested_source_id || fixture.source.locator != SOURCE_LOCATOR {
        return Err(
            "The selected source identity does not match the approved fixture.".to_string(),
        );
    }
    if fixture.memories.is_empty() {
        return Err("The selected source contains no importable records.".to_string());
    }

    let mut source = source_option();
    source.display_name = fixture.source.label;

    let mut events = Vec::with_capacity(fixture.memories.len());
    for memory in fixture.memories {
        let text = memory.text.trim();
        if memory.id.trim().is_empty()
            || memory.created_at.trim().is_empty()
            || text.is_empty()
            || memory.kind != "completion"
        {
            return Err("The selected Codex memory contains an unsupported record.".to_string());
        }
        OffsetDateTime::parse(&memory.created_at, &Rfc3339).map_err(|_| {
            "The selected Codex memory contains an invalid source timestamp.".to_string()
        })?;

        let content_hash = stable_hash(&[
            &fixture.source.id,
            &memory.id,
            &memory.created_at,
            &memory.kind,
            text,
        ]);
        events.push(NormalizedMemoryEvent {
            id: stable_id("memory", &[&fixture.source.id, &memory.id]),
            schema_version: MEMORY_EVENT_SCHEMA_VERSION,
            source_id: fixture.source.id.clone(),
            source_record_id: memory.id,
            source_timestamp: memory.created_at,
            kind: memory.kind,
            normalized_text: text.to_string(),
            content_hash,
        });
    }

    let (consent_scope, consent_scope_json, consent_scope_hash) =
        fixture_consent_contract(&source)?;

    Ok(PreparedImport {
        source,
        source_content_hash: sha256(raw.as_bytes()),
        events,
        consent_scope,
        consent_scope_json,
        consent_scope_hash,
    })
}

pub(crate) fn fixture_consent_contract(
    source: &SourceOption,
) -> Result<(ConsentScopeV1, String, String), String> {
    let consent_scope = ConsentScopeV1 {
        schema_version: 1,
        revision: 1,
        source_id: source.id.clone(),
        adapter_id: source.adapter_id.clone(),
        adapter_version: source.adapter_version,
        data_categories: vec!["synthetic-completion".to_string()],
        purposes: vec!["local-creature-derivation".to_string()],
        read_only: true,
    };
    let (consent_scope_json, consent_scope_hash) = consent_scope_contract(&consent_scope)?;
    Ok((consent_scope, consent_scope_json, consent_scope_hash))
}

fn source_option() -> SourceOption {
    SourceOption {
        id: SUPPORTED_SOURCE_ID.to_string(),
        adapter_id: CODEX_ADAPTER_ID.to_string(),
        adapter_version: CODEX_ADAPTER_VERSION,
        display_name: "Codex · First memory fixture".to_string(),
        locator: SOURCE_LOCATOR.to_string(),
        fixture_only: true,
    }
}

pub(crate) fn stable_id(prefix: &str, values: &[&str]) -> String {
    let hash = stable_hash(values);
    format!("{prefix}_{}", &hash[..24])
}

pub(crate) fn stable_hash(values: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for value in values {
        hasher.update(value.as_bytes());
        hasher.update([0]);
    }
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

pub(crate) fn sha256(value: &[u8]) -> String {
    Sha256::digest(value)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

pub(crate) fn consent_scope_contract(scope: &ConsentScopeV1) -> Result<(String, String), String> {
    let json = serde_json::to_string(scope)
        .map_err(|_| "Memoryling could not prepare the consent scope.".to_string())?;
    let hash = sha256(json.as_bytes());
    Ok((json, hash))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn fixture_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/codex-first-memory-v1.json")
    }

    #[test]
    fn synthetic_preview_is_narrow_and_deterministic() {
        let first = preview_source(SUPPORTED_SOURCE_ID, &fixture_path())
            .expect("preview should parse")
            .0;
        let second = preview_source(SUPPORTED_SOURCE_ID, &fixture_path())
            .expect("preview should be repeatable")
            .0;

        assert_eq!(first, second);
        assert_eq!(first.record_count, 1);
        assert!(first.source.fixture_only);
        assert!(first.access_scope.read_only);
        assert!(!first.access_scope.source_write_access);
        assert!(!first.access_scope.network_access);
        assert!(!first.access_scope.arbitrary_path_access);
        assert_eq!(
            first.records[0].text_preview.as_deref(),
            Some("Shipped a local-first creature whose changes can always explain their source.")
        );
        assert_eq!(
            first.records[0].character_count,
            first.records[0]
                .text_preview
                .as_ref()
                .expect("fixture preview remains visible")
                .chars()
                .count()
        );
        assert_eq!(first.records[0].content_hash.len(), 64);
        assert_eq!(first.consent_scope_hash.len(), 64);
        assert_eq!(
            sha256(
                serde_json::to_string(&first.consent_scope)
                    .expect("scope should serialize")
                    .as_bytes()
            ),
            first.consent_scope_hash
        );
    }

    #[test]
    fn unknown_source_and_version_fail_closed() {
        assert!(prepare_import("codex.other", &fixture_path()).is_err());

        let raw = fs::read_to_string(fixture_path()).expect("fixture should be readable");
        let unsupported = raw.replace("\"version\": 1", "\"version\": 99");
        let error = match parse_fixture(&unsupported, SUPPORTED_SOURCE_ID) {
            Ok(_) => panic!("unsupported version should fail"),
            Err(error) => error,
        };
        assert_eq!(error, "Unsupported Codex memory format or version.");

        let invalid_time = raw.replace("2026-08-10T08:15:00Z", "not-a-date");
        let error = match parse_fixture(&invalid_time, SUPPORTED_SOURCE_ID) {
            Ok(_) => panic!("invalid timestamp should fail"),
            Err(error) => error,
        };
        assert_eq!(
            error,
            "The selected Codex memory contains an invalid source timestamp."
        );
    }
}
