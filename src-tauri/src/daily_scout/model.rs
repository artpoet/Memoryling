use serde::{Deserialize, Serialize};

pub(crate) const PROVIDER_ID: &str = "openai";
pub(crate) const MODEL_ID: &str = "gpt-5.6-terra";
pub(crate) const CONTEXT_COMPILER_VERSION: u8 = 1;
pub(crate) const CONSENT_SCHEMA_VERSION: u8 = 1;
pub(crate) const CONSENT_REVISION: u8 = 1;
pub(crate) const MAX_CONTEXT_CHARS: usize = 3_000;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DailySearchContext {
    pub schema_version: u8,
    pub work_domains: Vec<String>,
    pub public_tools_and_models: Vec<String>,
    pub current_goals: Vec<String>,
    pub non_sensitive_constraints: Vec<String>,
    pub evidence_window: EvidenceWindow,
    pub preferred_insight_categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceWindow {
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DailyInsightConsentScopeV1 {
    pub schema_version: u8,
    pub revision: u8,
    pub provider: String,
    pub model: String,
    pub source_ids: Vec<String>,
    pub data_categories: Vec<String>,
    pub purposes: Vec<String>,
    pub context_compiler_version: u8,
    pub max_context_characters: usize,
    pub automatic_daily_send: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CompiledContext {
    pub outbound: DailySearchContext,
    pub source_ids: Vec<String>,
    pub context_json: String,
    pub context_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DailyCitation {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DailyInsight {
    pub id: String,
    pub local_date: String,
    pub provider: String,
    pub model: String,
    pub pet_message: String,
    pub strength: String,
    pub relevance_reason: String,
    pub searched_at: String,
    pub read: bool,
    pub citations: Vec<DailyCitation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DailyAttemptSummary {
    pub local_date: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DailyScoutState {
    pub enabled: bool,
    pub has_api_key: bool,
    pub can_enable: bool,
    pub provider: String,
    pub model: String,
    pub delivery_time: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_preview: Option<DailySearchContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_insight: Option<DailyInsight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub today_attempt: Option<DailyAttemptSummary>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConfigureDailyScoutRequest {
    pub locale: String,
    pub delivery_time: String,
    pub consent_accepted: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OpenDailyScoutLinkRequest {
    pub kind: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DailyScoutSettings {
    pub enabled: bool,
    pub locale: String,
    pub delivery_hour: u8,
    pub delivery_minute: u8,
    pub consent_scope_json: String,
    pub consent_scope_hash: String,
}

impl DailyScoutSettings {
    pub(crate) fn delivery_time(&self) -> String {
        format!("{:02}:{:02}", self.delivery_hour, self.delivery_minute)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocalNow {
    pub local_date: String,
    pub local_minutes: u16,
    pub timezone: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProviderInsight {
    pub pet_message: String,
    pub strength: String,
    pub citations: Vec<DailyCitation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProviderError {
    Authentication,
    QuotaOrRateLimit,
    InvalidRequest,
    Timeout,
    Offline,
    Service,
    InvalidResponse,
}

impl ProviderError {
    pub(crate) fn code(self) -> &'static str {
        match self {
            Self::Authentication => "authentication",
            Self::QuotaOrRateLimit => "quota-or-rate-limit",
            Self::InvalidRequest => "invalid-request",
            Self::Timeout => "timeout",
            Self::Offline => "offline",
            Self::Service => "service",
            Self::InvalidResponse => "invalid-response",
        }
    }
}
