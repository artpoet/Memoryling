use serde::{Deserialize, Serialize};

pub const MEMORY_EVENT_SCHEMA_VERSION: i64 = 1;
pub const STORE_SCHEMA_VERSION: i64 = 1;
pub const DERIVATION_VERSION: i64 = 1;
pub const CODEX_ADAPTER_ID: &str = "codex-durable-memory";
pub const CODEX_ADAPTER_VERSION: i64 = 1;

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
    pub text_preview: String,
    pub content_hash: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApproveImportRequest {
    pub preview_id: String,
    pub source_id: String,
    pub selected_record_ids: Vec<String>,
}

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct PreparedImport {
    pub source: SourceOption,
    pub source_content_hash: String,
    pub events: Vec<NormalizedMemoryEvent>,
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
    pub memory_text: String,
    pub content_hash: String,
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
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreatureRenderState {
    pub schema_version: u8,
    pub revision: String,
    pub real_memory_access: RealMemoryAccess,
    pub fixture_state: FixtureState,
    pub envelope: CreatureEnvelope,
    pub body_module: BodyModule,
    pub palette: CreaturePalette,
    pub motion: CreatureMotion,
    pub marks: Vec<CreatureRenderMark>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RealMemoryAccess {
    Off,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum FixtureState {
    Empty,
    Approved,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CreatureEnvelope {
    Compact,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BodyModule {
    Baseline,
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
}
