use std::{
    collections::HashSet,
    fs,
    path::Path,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use serde::Deserialize;
use tauri::{AppHandle, Manager};
use time::{format_description::well_known::Rfc3339, OffsetDateTime, UtcOffset};

use super::{
    model::{AgentActivity, CreatureRenderState},
    store::MemoryStore,
};

const INBOX_RELATIVE_PATH: [&str; 2] = ["agent-inbox", "operation-v2.json"];
const MAX_OPERATION_BYTES: u64 = 64 * 1024;
const POLL_INTERVAL: Duration = Duration::from_secs(5);
const AMBIENT_MINUTES_MIN: u64 = 35;
const AMBIENT_MINUTES_SPAN: u64 = 36;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct AgentOperationPackage {
    pub schema_version: u8,
    pub operation_id: String,
    pub generated_at: String,
    pub agent: AgentDescriptor,
    pub source_digest: String,
    pub profile: AgentProfile,
    pub appearance_plan: AppearancePlan,
    pub evidence: Vec<OperationEvidence>,
    pub dialogues: Vec<DialogueCard>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct AgentDescriptor {
    pub family: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct AgentProfile {
    pub dominant_activity: AgentActivity,
    pub secondary_activity: Option<AgentActivity>,
    pub journey_state: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct OperationEvidence {
    pub id: String,
    pub kind: String,
    pub reference_hash: String,
    pub observed_at: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct AppearancePlan {
    pub decision: String,
    pub qualification: String,
    pub target_activity: Option<AgentActivity>,
    pub target_journey_state: Option<String>,
    pub evidence_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct DialogueCard {
    pub id: String,
    pub theme_id: String,
    pub semantic_group: String,
    pub category: String,
    pub text: LocalizedDialogue,
    pub trigger: String,
    pub priority: u8,
    pub not_before: Option<String>,
    pub expires_at: Option<String>,
    pub cooldown_minutes: u16,
    pub max_uses: u8,
    pub evidence_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct LocalizedDialogue {
    pub en: String,
    pub zh_tw: String,
}

impl AgentOperationPackage {
    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.schema_version != 2
            || !valid_id(&self.operation_id)
            || !valid_hash(&self.source_digest)
            || OffsetDateTime::parse(&self.generated_at, &Rfc3339).is_err()
            || !matches!(self.agent.family.as_str(), "codex" | "claude" | "other")
            || self.profile.dominant_activity == AgentActivity::Off
            || self.profile.secondary_activity == Some(AgentActivity::Off)
            || !matches!(
                self.profile.journey_state.as_str(),
                "steady" | "exploring" | "milestone" | "recovering"
            )
            || !(1..=12).contains(&self.evidence.len())
            || self.dialogues.len() != 48
        {
            return Err("The Agent operation package does not match protocol v2.".to_string());
        }

        let mut evidence_ids = HashSet::new();
        for evidence in &self.evidence {
            if !valid_id(&evidence.id)
                || !evidence_ids.insert(evidence.id.as_str())
                || !matches!(
                    evidence.kind.as_str(),
                    "durable-memory" | "recent-work" | "repo-ssot" | "current-thread"
                )
                || !valid_hash(&evidence.reference_hash)
                || OffsetDateTime::parse(&evidence.observed_at, &Rfc3339).is_err()
            {
                return Err("The Agent operation evidence is invalid.".to_string());
            }
        }

        let mut appearance_evidence_ids = HashSet::new();
        if self.appearance_plan.evidence_ids.iter().any(|id| {
            !evidence_ids.contains(id.as_str()) || !appearance_evidence_ids.insert(id.as_str())
        }) {
            return Err("The Agent appearance plan evidence is invalid.".to_string());
        }
        let valid_target_journey = self
            .appearance_plan
            .target_journey_state
            .as_deref()
            .is_some_and(|value| {
                matches!(value, "steady" | "exploring" | "milestone" | "recovering")
            });
        let appearance_valid = match self.appearance_plan.decision.as_str() {
            "hold" => {
                self.appearance_plan.qualification == "insufficient-evidence"
                    && self.appearance_plan.target_activity.is_none()
                    && self.appearance_plan.target_journey_state.is_none()
            }
            "reset" => {
                self.appearance_plan.qualification == "source-removed"
                    && self.appearance_plan.target_activity.is_none()
                    && self.appearance_plan.target_journey_state.is_none()
            }
            "change" => {
                self.appearance_plan
                    .target_activity
                    .is_some_and(|activity| activity != AgentActivity::Off)
                    && valid_target_journey
                    && match self.appearance_plan.qualification.as_str() {
                        "explicit-milestone" => !appearance_evidence_ids.is_empty(),
                        "consistent-signals" => appearance_evidence_ids.len() >= 2,
                        _ => false,
                    }
            }
            _ => false,
        };
        if !appearance_valid {
            return Err("The Agent appearance plan is invalid.".to_string());
        }

        let mut dialogue_ids = HashSet::new();
        let mut opening_count = 0;
        let mut interaction_count = 0;
        let mut ambient_count = 0;
        let mut appearance_count = 0;
        for dialogue in &self.dialogues {
            let mut dialogue_evidence_ids = HashSet::new();
            if !valid_id(&dialogue.id)
                || !dialogue_ids.insert(dialogue.id.as_str())
                || !valid_id(&dialogue.theme_id)
                || !valid_id(&dialogue.semantic_group)
                || !valid_dialogue(&dialogue.text.en)
                || !valid_dialogue(&dialogue.text.zh_tw)
                || !matches!(
                    dialogue.trigger.as_str(),
                    "on-open" | "on-interact" | "ambient"
                )
                || dialogue.priority > 3
                || dialogue.cooldown_minutes > 10_080
                || !(1..=20).contains(&dialogue.max_uses)
                || dialogue.evidence_ids.is_empty()
                || dialogue.evidence_ids.iter().any(|id| {
                    !evidence_ids.contains(id.as_str())
                        || !dialogue_evidence_ids.insert(id.as_str())
                })
                || !valid_optional_timestamp(dialogue.not_before.as_deref())
                || !valid_optional_timestamp(dialogue.expires_at.as_deref())
            {
                return Err("The Agent operation dialogue deck is invalid.".to_string());
            }
            match dialogue.category.as_str() {
                "opening" if dialogue.trigger == "on-open" => opening_count += 1,
                "interaction" if dialogue.trigger == "on-interact" => interaction_count += 1,
                "ambient" if dialogue.trigger == "ambient" => ambient_count += 1,
                "appearance" if dialogue.trigger == "on-open" => appearance_count += 1,
                _ => {
                    return Err(
                        "The Agent dialogue category does not match its trigger.".to_string()
                    )
                }
            }
        }
        if (
            opening_count,
            interaction_count,
            ambient_count,
            appearance_count,
        ) != (8, 20, 16, 4)
        {
            return Err("The Agent dialogue deck must contain 8 opening, 20 interaction, 16 ambient, and 4 appearance cards.".to_string());
        }
        Ok(())
    }
}

pub(crate) fn dialogue_is_active_at(dialogue: &DialogueCard, now: OffsetDateTime) -> bool {
    let not_before_ok = dialogue
        .not_before
        .as_deref()
        .is_none_or(|value| OffsetDateTime::parse(value, &Rfc3339).is_ok_and(|bound| now >= bound));
    let expires_ok = dialogue
        .expires_at
        .as_deref()
        .is_none_or(|value| OffsetDateTime::parse(value, &Rfc3339).is_ok_and(|bound| now <= bound));
    not_before_ok && expires_ok
}

fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value.bytes().enumerate().all(|(index, byte)| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || (index > 0 && matches!(byte, b'.' | b'_' | b'-'))
        })
}

fn valid_hash(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn valid_dialogue(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty()
        && trimmed.chars().count() <= 160
        && !trimmed.contains('\r')
        && !trimmed.contains('\n')
}

fn valid_optional_timestamp(value: Option<&str>) -> bool {
    value.is_none_or(|timestamp| OffsetDateTime::parse(timestamp, &Rfc3339).is_ok())
}

fn inbox_path<R: tauri::Runtime>(app: &AppHandle<R>) -> Result<std::path::PathBuf, String> {
    let root = app
        .path()
        .app_local_data_dir()
        .map_err(|_| "Memoryling could not resolve its Agent operation inbox.".to_string())?;
    Ok(root
        .join(INBOX_RELATIVE_PATH[0])
        .join(INBOX_RELATIVE_PATH[1]))
}

fn read_package(path: &Path) -> Result<Option<AgentOperationPackage>, String> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err("Memoryling could not inspect the Agent operation inbox.".to_string()),
    };
    if metadata.file_type().is_symlink()
        || !metadata.is_file()
        || metadata.len() == 0
        || metadata.len() > MAX_OPERATION_BYTES
    {
        return Err("The Agent operation inbox item failed local safety checks.".to_string());
    }
    let bytes = fs::read(path)
        .map_err(|_| "Memoryling could not read the Agent operation inbox item.".to_string())?;
    let package: AgentOperationPackage = serde_json::from_slice(&bytes)
        .map_err(|_| "The Agent operation inbox item is not valid protocol-v2 JSON.".to_string())?;
    package.validate()?;
    Ok(Some(package))
}

pub(crate) fn process_inbox<R: tauri::Runtime>(app: &AppHandle<R>) -> Result<bool, String> {
    let path = inbox_path(app)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|_| "Memoryling could not create its Agent operation inbox.".to_string())?;
    }
    let package = match read_package(&path) {
        Ok(Some(package)) => package,
        Ok(None) => return Ok(false),
        Err(error) => {
            let _ = super::store_for(app)?.record_agent_operation_error("invalid-package");
            let _ = fs::remove_file(&path);
            return Err(error);
        }
    };
    super::store_for(app)?.apply_agent_operation(&package)?;
    fs::remove_file(&path).map_err(|_| {
        "Memoryling applied the operation but could not clear its inbox item.".to_string()
    })?;
    Ok(true)
}

pub(crate) fn setup<R: tauri::Runtime>(app: &tauri::App<R>) {
    let handle = app.handle().clone();
    std::thread::spawn(move || {
        let mut next_ambient = Instant::now() + next_ambient_delay();
        loop {
            if process_inbox(&handle).unwrap_or(false) {
                crate::desktop_shell::emit_creature_state_changed(&handle);
            }
            if Instant::now() >= next_ambient {
                if let Ok(store) = super::store_for(&handle) {
                    let before = store
                        .creature_render_state()
                        .ok()
                        .map(|state| state.revision);
                    let after = store
                        .advance_agent_dialogue("ambient")
                        .ok()
                        .map(|state| state.revision);
                    if before.is_some() && after.is_some() && before != after {
                        crate::desktop_shell::emit_creature_state_changed(&handle);
                    }
                }
                next_ambient = Instant::now() + next_ambient_delay();
            }
            if let Ok(store) = super::store_for(&handle) {
                let before = store
                    .creature_render_state()
                    .ok()
                    .map(|state| state.revision);
                let after = store
                    .apply_pending_appearance_if_due()
                    .ok()
                    .map(|state| state.revision);
                if before.is_some() && after.is_some() && before != after {
                    crate::desktop_shell::emit_creature_state_changed(&handle);
                }
            }
            std::thread::sleep(POLL_INTERVAL);
        }
    });
}

fn next_ambient_delay() -> Duration {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_nanos() as u64)
        .unwrap_or(0);
    Duration::from_secs((AMBIENT_MINUTES_MIN + seed % AMBIENT_MINUTES_SPAN) * 60)
}

pub(crate) fn local_clock() -> (String, String, u8) {
    let utc = OffsetDateTime::now_utc();
    let offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    let local = utc.to_offset(offset);
    (
        utc.format(&Rfc3339)
            .unwrap_or_else(|_| utc.unix_timestamp().to_string()),
        local.date().to_string(),
        local.hour(),
    )
}

pub(crate) fn advance_dialogue(
    store: &MemoryStore,
    trigger: &str,
) -> Result<CreatureRenderState, String> {
    if !matches!(trigger, "on-interact" | "ambient") {
        return Err("The pet dialogue trigger is not supported.".to_string());
    }
    store.advance_agent_dialogue(trigger)
}

#[cfg(test)]
pub(crate) fn synthetic_package() -> AgentOperationPackage {
    AgentOperationPackage {
        schema_version: 2,
        operation_id: "operation.synthetic-001".to_string(),
        generated_at: "2026-08-13T10:00:00Z".to_string(),
        agent: AgentDescriptor {
            family: "codex".to_string(),
        },
        source_digest: "a".repeat(64),
        profile: AgentProfile {
            dominant_activity: AgentActivity::Building,
            secondary_activity: Some(AgentActivity::Design),
            journey_state: "milestone".to_string(),
        },
        appearance_plan: AppearancePlan {
            decision: "change".to_string(),
            qualification: "consistent-signals".to_string(),
            target_activity: Some(AgentActivity::Building),
            target_journey_state: Some("milestone".to_string()),
            evidence_ids: vec!["evidence.repo".to_string(), "evidence.thread".to_string()],
        },
        evidence: vec![
            OperationEvidence {
                id: "evidence.repo".to_string(),
                kind: "repo-ssot".to_string(),
                reference_hash: "b".repeat(64),
                observed_at: "2026-08-13T09:00:00Z".to_string(),
            },
            OperationEvidence {
                id: "evidence.thread".to_string(),
                kind: "current-thread".to_string(),
                reference_hash: "c".repeat(64),
                observed_at: "2026-08-13T09:30:00Z".to_string(),
            },
        ],
        dialogues: (1..=48)
            .map(|index| DialogueCard {
                id: format!("dialogue-{index}"),
                theme_id: format!("theme-{}", (index - 1) % 6 + 1),
                semantic_group: format!("meaning-{index}"),
                category: match index {
                    1..=8 => "opening",
                    9..=28 => "interaction",
                    29..=44 => "ambient",
                    _ => "appearance",
                }
                .to_string(),
                text: LocalizedDialogue {
                    en: format!("Synthetic line {index}."),
                    zh_tw: format!("合成對話 {index}。"),
                },
                trigger: match index {
                    1..=8 | 45..=48 => "on-open",
                    9..=28 => "on-interact",
                    _ => "ambient",
                }
                .to_string(),
                priority: 1,
                not_before: None,
                expires_at: None,
                cooldown_minutes: 0,
                max_uses: 2,
                evidence_ids: vec!["evidence.repo".to_string()],
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_rejects_raw_shaped_or_unbounded_packages() {
        assert!(synthetic_package().validate().is_ok());
        let mut invalid = synthetic_package();
        invalid.dialogues[0].text.en = "line one\nline two".to_string();
        assert!(invalid.validate().is_err());
        let mut invalid_hash = synthetic_package();
        invalid_hash.evidence[0].reference_hash = "NOT-A-HASH".to_string();
        assert!(invalid_hash.validate().is_err());

        let mut missing_interaction = synthetic_package();
        for dialogue in &mut missing_interaction.dialogues {
            if dialogue.category == "interaction" {
                dialogue.trigger = "on-open".to_string();
            }
        }
        assert!(missing_interaction.validate().is_err());

        let mut duplicate_evidence = synthetic_package();
        duplicate_evidence.dialogues[0]
            .evidence_ids
            .push("evidence.repo".to_string());
        assert!(duplicate_evidence.validate().is_err());

        let mut expired = synthetic_package();
        expired.dialogues[0].expires_at = Some("2026-08-13T09:59:59Z".to_string());
        let now = OffsetDateTime::parse("2026-08-13T10:00:00Z", &Rfc3339).unwrap();
        assert!(!dialogue_is_active_at(&expired.dialogues[0], now));

        let mut weak_change = synthetic_package();
        weak_change.appearance_plan.evidence_ids.truncate(1);
        assert!(weak_change.validate().is_err());

        let delay = next_ambient_delay();
        assert!(delay >= Duration::from_secs(35 * 60));
        assert!(delay <= Duration::from_secs(70 * 60));
    }
}
