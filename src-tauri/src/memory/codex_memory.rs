use std::{
    fs,
    path::{Path, PathBuf},
};

use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use super::{
    adapter::{consent_scope_contract, sha256, stable_hash, stable_id},
    model::{
        AccessScope, ConsentScopeV1, ImportPreview, NormalizedMemoryEvent, PreparedImport,
        PreviewRecord, PreviewTimeRange, SourceOption, CODEX_MEMORY_ADAPTER_ID,
        CODEX_MEMORY_ADAPTER_VERSION, CODEX_MEMORY_SOURCE_ID, MEMORY_EVENT_SCHEMA_VERSION,
    },
};

const SOURCE_LOCATOR: &str = "codex-home://memories";
const MAX_FILE_BYTES: u64 = 2 * 1024 * 1024;
const MAX_TOTAL_BYTES: u64 = 4 * 1024 * 1024;
const ALLOWED_FILES: [(&str, &str); 2] = [
    ("memory_summary.md", "memory-summary"),
    ("MEMORY.md", "durable-memory-registry"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AgentMemoryReadFailure {
    SourceMissing,
    NeedsAttention,
}

impl AgentMemoryReadFailure {
    pub(crate) fn code(self) -> &'static str {
        match self {
            Self::SourceMissing => "source-missing",
            Self::NeedsAttention => "source-invalid",
        }
    }

    pub(crate) fn message(self) -> String {
        match self {
            Self::SourceMissing => {
                "The configured Codex memory store has no supported memory files.".to_string()
            }
            Self::NeedsAttention => {
                "The configured Codex memory store failed its local safety checks.".to_string()
            }
        }
    }
}

pub(crate) fn source_option() -> SourceOption {
    SourceOption {
        id: CODEX_MEMORY_SOURCE_ID.to_string(),
        adapter_id: CODEX_MEMORY_ADAPTER_ID.to_string(),
        adapter_version: CODEX_MEMORY_ADAPTER_VERSION,
        display_name: "Codex · Local Agent memories".to_string(),
        locator: SOURCE_LOCATOR.to_string(),
        fixture_only: false,
    }
}

pub(crate) fn configured_memories_root() -> Result<PathBuf, String> {
    if let Some(codex_home) = std::env::var_os("CODEX_HOME") {
        let home = PathBuf::from(codex_home);
        if home.is_absolute() {
            return Ok(home.join("memories"));
        }
        return Err("The configured Codex home is not an absolute local path.".to_string());
    }
    let user_profile = std::env::var_os("USERPROFILE")
        .ok_or_else(|| "Memoryling could not resolve the local Codex home.".to_string())?;
    let home = PathBuf::from(user_profile);
    if !home.is_absolute() {
        return Err("The local Windows user profile is not an absolute path.".to_string());
    }
    Ok(home.join(".codex").join("memories"))
}

pub(crate) fn root_fingerprint(root: &Path) -> String {
    let normalized = root.to_string_lossy().replace('\\', "/").to_lowercase();
    sha256(normalized.as_bytes())
}

pub(crate) fn preview_source() -> Result<(ImportPreview, PreparedImport), String> {
    let root = configured_memories_root()?;
    preview_source_at(&root).map_err(AgentMemoryReadFailure::message)
}

fn preview_source_at(
    root: &Path,
) -> Result<(ImportPreview, PreparedImport), AgentMemoryReadFailure> {
    let prepared = prepare_import_at(root)?;
    let mut timestamps = prepared
        .events
        .iter()
        .map(|event| event.source_timestamp.as_str())
        .collect::<Vec<_>>();
    timestamps.sort_unstable();
    let start = timestamps
        .first()
        .ok_or(AgentMemoryReadFailure::SourceMissing)?;
    let end = timestamps
        .last()
        .ok_or(AgentMemoryReadFailure::SourceMissing)?;
    let records = prepared
        .events
        .iter()
        .map(|event| PreviewRecord {
            id: event.source_record_id.clone(),
            source_timestamp: event.source_timestamp.clone(),
            kind: event.kind.clone(),
            text_preview: None,
            character_count: event.normalized_text.chars().count(),
            content_hash: event.content_hash.clone(),
        })
        .collect::<Vec<_>>();
    Ok((
        ImportPreview {
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
        },
        prepared,
    ))
}

pub(crate) fn prepare_import() -> Result<PreparedImport, AgentMemoryReadFailure> {
    let root = configured_memories_root().map_err(|_| AgentMemoryReadFailure::NeedsAttention)?;
    prepare_import_at(&root)
}

pub(crate) fn prepare_import_at(root: &Path) -> Result<PreparedImport, AgentMemoryReadFailure> {
    let root_metadata =
        fs::symlink_metadata(root).map_err(|_| AgentMemoryReadFailure::SourceMissing)?;
    if root_metadata.file_type().is_symlink() || !root_metadata.is_dir() {
        return Err(AgentMemoryReadFailure::NeedsAttention);
    }
    let canonical_root =
        fs::canonicalize(root).map_err(|_| AgentMemoryReadFailure::NeedsAttention)?;
    let mut total_bytes = 0_u64;
    let mut events = Vec::new();

    for (file_name, record_id) in ALLOWED_FILES {
        let candidate = root.join(file_name);
        let metadata = match fs::symlink_metadata(&candidate) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(_) => return Err(AgentMemoryReadFailure::NeedsAttention),
        };
        if metadata.file_type().is_symlink()
            || !metadata.is_file()
            || metadata.len() > MAX_FILE_BYTES
        {
            return Err(AgentMemoryReadFailure::NeedsAttention);
        }
        total_bytes = total_bytes
            .checked_add(metadata.len())
            .ok_or(AgentMemoryReadFailure::NeedsAttention)?;
        if total_bytes > MAX_TOTAL_BYTES {
            return Err(AgentMemoryReadFailure::NeedsAttention);
        }
        let canonical_file =
            fs::canonicalize(&candidate).map_err(|_| AgentMemoryReadFailure::NeedsAttention)?;
        if canonical_file.parent() != Some(canonical_root.as_path()) {
            return Err(AgentMemoryReadFailure::NeedsAttention);
        }
        let bytes =
            fs::read(&canonical_file).map_err(|_| AgentMemoryReadFailure::NeedsAttention)?;
        let text = String::from_utf8(bytes).map_err(|_| AgentMemoryReadFailure::NeedsAttention)?;
        let normalized_text = text.trim();
        if normalized_text.is_empty() {
            return Err(AgentMemoryReadFailure::NeedsAttention);
        }
        let modified = metadata
            .modified()
            .map_err(|_| AgentMemoryReadFailure::NeedsAttention)?;
        let source_timestamp = OffsetDateTime::from(modified)
            .format(&Rfc3339)
            .map_err(|_| AgentMemoryReadFailure::NeedsAttention)?;
        let content_hash = stable_hash(&[
            CODEX_MEMORY_SOURCE_ID,
            record_id,
            &source_timestamp,
            "agent-memory-document",
            normalized_text,
        ]);
        events.push(NormalizedMemoryEvent {
            id: stable_id("memory", &[CODEX_MEMORY_SOURCE_ID, record_id]),
            schema_version: MEMORY_EVENT_SCHEMA_VERSION,
            source_id: CODEX_MEMORY_SOURCE_ID.to_string(),
            source_record_id: record_id.to_string(),
            source_timestamp,
            kind: "agent-memory-document".to_string(),
            normalized_text: normalized_text.to_string(),
            content_hash,
        });
    }

    if events.is_empty() {
        return Err(AgentMemoryReadFailure::SourceMissing);
    }
    events.sort_by(|left, right| left.source_record_id.cmp(&right.source_record_id));
    let source = source_option();
    let locator_hash = root_fingerprint(root);
    let consent_scope = ConsentScopeV1 {
        schema_version: 2,
        revision: 1,
        source_id: source.id.clone(),
        adapter_id: source.adapter_id.clone(),
        adapter_version: source.adapter_version,
        data_categories: vec![
            "agent-memory-summary".to_string(),
            "agent-durable-memory-registry".to_string(),
        ],
        purposes: vec![
            "local-creature-derivation".to_string(),
            "automatic-read-only-sync".to_string(),
        ],
        read_only: true,
        source_locator_hash: Some(locator_hash),
        automatic_sync: true,
    };
    let (consent_scope_json, consent_scope_hash) = consent_scope_contract(&consent_scope)
        .map_err(|_| AgentMemoryReadFailure::NeedsAttention)?;
    let hash_parts = events
        .iter()
        .flat_map(|event| [event.source_record_id.as_str(), event.content_hash.as_str()])
        .collect::<Vec<_>>();
    let source_content_hash = stable_hash(&hash_parts);

    Ok(PreparedImport {
        source,
        source_content_hash,
        events,
        consent_scope,
        consent_scope_json,
        consent_scope_hash,
    })
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;

    fn temporary_root() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "memoryling-agent-memory-{}-{nonce}",
            std::process::id()
        ))
    }

    #[test]
    fn allowlisted_files_create_redacted_all_records_preview() {
        let root = temporary_root();
        fs::create_dir_all(&root).unwrap();
        fs::write(
            root.join("memory_summary.md"),
            "# Synthetic summary\nSafe fixture only.",
        )
        .unwrap();
        fs::write(
            root.join("MEMORY.md"),
            "# Synthetic durable entries\n- example",
        )
        .unwrap();
        fs::write(root.join("ignored.jsonl"), "private-shaped but ignored").unwrap();

        let (preview, prepared) =
            preview_source_at(&root).expect("synthetic memory files should parse");
        assert_eq!(preview.record_count, 2);
        assert!(preview
            .records
            .iter()
            .all(|record| record.text_preview.is_none()));
        assert!(prepared.consent_scope.automatic_sync);
        assert_eq!(prepared.consent_scope.schema_version, 2);
        assert!(prepared
            .events
            .iter()
            .all(|event| event.kind == "agent-memory-document"));
        assert!(!prepared
            .events
            .iter()
            .any(|event| event.normalized_text.contains("ignored")));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn missing_empty_and_oversized_sources_fail_closed() {
        let root = temporary_root();
        assert_eq!(
            prepare_import_at(&root),
            Err(AgentMemoryReadFailure::SourceMissing)
        );
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("MEMORY.md"), "   ").unwrap();
        assert_eq!(
            prepare_import_at(&root),
            Err(AgentMemoryReadFailure::NeedsAttention)
        );
        fs::write(
            root.join("MEMORY.md"),
            vec![b'x'; MAX_FILE_BYTES as usize + 1],
        )
        .unwrap();
        assert_eq!(
            prepare_import_at(&root),
            Err(AgentMemoryReadFailure::NeedsAttention)
        );
        fs::remove_dir_all(root).unwrap();
    }
}
