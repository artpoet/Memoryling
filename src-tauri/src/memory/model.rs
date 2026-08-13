use serde::{Deserialize, Serialize};

fn is_false(value: &bool) -> bool {
    !*value
}

pub const MEMORY_EVENT_SCHEMA_VERSION: i64 = 1;
pub const STORE_SCHEMA_VERSION: i64 = 4;
pub const DERIVATION_VERSION: i64 = 1;
pub const CODEX_ADAPTER_ID: &str = "codex-durable-memory";
pub const CODEX_ADAPTER_VERSION: i64 = 1;
pub const CODEX_THREAD_ADAPTER_ID: &str = "codex-app-server-thread";
pub const CODEX_THREAD_ADAPTER_VERSION: i64 = 1;
pub const CODEX_MEMORY_ADAPTER_ID: &str = "codex-local-memory-store";
pub const CODEX_MEMORY_ADAPTER_VERSION: i64 = 1;
pub const CODEX_MEMORY_SOURCE_ID: &str = "codex.local-memories";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SourceOption {
    pub id: String,
    pub adapter_id: String,
    pub adapter_version: i64,
    pub display_name: String,
    pub locator: String,
    pub fixture_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PreviewRecord {
    pub id: String,
    pub source_timestamp: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_preview: Option<String>,
    pub character_count: usize,
    pub content_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConsentScopeV1 {
    pub schema_version: u8,
    pub revision: u8,
    pub source_id: String,
    pub adapter_id: String,
    pub adapter_version: i64,
    pub data_categories: Vec<String>,
    pub purposes: Vec<String>,
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_locator_hash: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub automatic_sync: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PreviewTimeRange {
    pub start: String,
    pub end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AccessScope {
    pub read_only: bool,
    pub source_write_access: bool,
    pub network_access: bool,
    pub arbitrary_path_access: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImportPreview {
    pub preview_id: String,
    pub source: SourceOption,
    pub record_count: usize,
    pub time_range: PreviewTimeRange,
    pub records: Vec<PreviewRecord>,
    pub access_scope: AccessScope,
    pub consent_scope: ConsentScopeV1,
    pub consent_scope_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApproveImportRequest {
    pub preview_id: String,
    pub source_id: String,
    pub selected_record_ids: Vec<String>,
    #[serde(default)]
    pub consent_scope_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NormalizedMemoryEvent {
    pub id: String,
    pub schema_version: i64,
    pub source_id: String,
    pub source_record_id: String,
    pub source_timestamp: String,
    pub kind: String,
    pub normalized_text: String,
    pub content_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreparedImport {
    pub source: SourceOption,
    pub source_content_hash: String,
    pub events: Vec<NormalizedMemoryEvent>,
    pub consent_scope: ConsentScopeV1,
    pub consent_scope_json: String,
    pub consent_scope_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CodexThreadCandidate {
    pub candidate_id: String,
    pub display_name: String,
    pub updated_at: String,
    pub source_kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CodexThreadCatalog {
    pub catalog_id: String,
    pub candidates: Vec<CodexThreadCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LineageSource {
    pub memory_event_id: String,
    pub memory_event_schema_version: i64,
    pub source_id: String,
    pub source_label: String,
    pub adapter_id: String,
    pub adapter_version: i64,
    pub source_record_id: String,
    pub source_timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_text: Option<String>,
    pub content_redacted: bool,
    pub character_count: usize,
    pub content_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_scope_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_revision: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreatureMark {
    pub id: String,
    pub style: String,
    pub signal_type: String,
    pub confidence: f64,
    pub derivation_version: i64,
    pub explanation_key: String,
    pub lineage: Vec<LineageSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MemoryState {
    pub store_schema_version: i64,
    pub source_count: usize,
    pub event_count: usize,
    pub signal_count: usize,
    pub marks: Vec<CreatureMark>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_source: Option<ActiveMemorySource>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ActiveMemorySource {
    pub source_id: String,
    pub adapter_id: String,
    pub display_name: String,
    pub automatic_sync: bool,
    pub sync_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_sync_at: Option<String>,
    pub synced_record_count: usize,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreatureRenderState {
    pub schema_version: u8,
    pub revision: String,
    pub real_memory_access: RealMemoryAccess,
    pub import_state: ImportState,
    pub envelope: CreatureEnvelope,
    pub stage: CreatureStage,
    pub body_module: BodyModule,
    pub palette: CreaturePalette,
    pub motion: CreatureMotion,
    pub daily_scout_state: DailyScoutRenderState,
    pub marks: Vec<CreatureRenderMark>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum DailyScoutRenderState {
    Off,
    Waiting,
    Ready,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RealMemoryAccess {
    Off,
    CodexLocal,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ImportState {
    Empty,
    FixtureApproved,
    ThreadApproved,
    AgentMemoryApproved,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CreatureEnvelope {
    Compact,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BodyModule {
    MemorySeedEggV1,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CreatureStage {
    Seed,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CreaturePalette {
    VioletMint,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CreatureMotion {
    Calm,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreatureRenderMark {
    pub id: String,
    pub style: CreatureRenderMarkStyle,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CreatureRenderMarkStyle {
    CompletionStar,
    MemoryHalo,
}
