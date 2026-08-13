use std::collections::BTreeSet;

use rusqlite::OptionalExtension;
use sha2::{Digest, Sha256};

use crate::memory::{model::CODEX_THREAD_ADAPTER_ID, store::MemoryStore};

use super::model::{
    CompiledContext, DailyInsightConsentScopeV1, DailySearchContext, EvidenceWindow,
    CONSENT_REVISION, CONSENT_SCHEMA_VERSION, CONTEXT_COMPILER_VERSION, MAX_CONTEXT_CHARS,
    MODEL_ID, PROVIDER_ID,
};

pub(crate) fn compile_recent_work(store: &MemoryStore) -> Result<Option<CompiledContext>, String> {
    let connection = store.open_connection()?;
    let source_id = connection
        .query_row(
            "SELECT source_id FROM source_imports
             WHERE adapter_id = ?1
             ORDER BY source_id LIMIT 1",
            [CODEX_THREAD_ADAPTER_ID],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_| "Memoryling could not inspect its approved work context.".to_string())?;
    let Some(source_id) = source_id else {
        return Ok(None);
    };

    let records = {
        let mut statement = connection
            .prepare(
                "SELECT normalized_text, source_timestamp
                 FROM memory_events
                 WHERE source_id = ?1
                 ORDER BY source_timestamp DESC, id DESC
                 LIMIT 12",
            )
            .map_err(|_| "Memoryling could not prepare its approved work context.".to_string())?;
        let rows = statement
            .query_map([&source_id], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(|_| "Memoryling could not read its approved work context.".to_string())?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Memoryling could not collect its approved work context.".to_string())?
    };
    if records.is_empty() {
        return Ok(None);
    }

    let joined = records
        .iter()
        .map(|(text, _)| text.to_lowercase())
        .collect::<Vec<_>>()
        .join("\n");

    let mut domains = BTreeSet::new();
    let mut tools = BTreeSet::new();
    let mut goals = BTreeSet::new();
    let mut constraints = BTreeSet::new();

    let agent_coding = contains_any(
        &joined,
        &[
            "agent",
            "codex",
            "code",
            "coding",
            "programming",
            "typescript",
            "react",
            "rust",
            "tauri",
            "debug",
            "test",
            "github",
            "pull request",
            " ci ",
        ],
    );
    let ai_video = contains_any(
        &joined,
        &[
            "minimax",
            "h3",
            "video",
            "kling",
            "runway",
            "veo",
            "sora",
            "storyboard",
            "shot",
            "camera",
            "character consistency",
        ],
    );
    if agent_coding {
        domains.insert("agent-assisted software development".to_string());
        goals.insert("effective agent-assisted coding workflows".to_string());
    }
    if ai_video {
        domains.insert("AI video production".to_string());
        goals.insert("efficient AI video production workflows".to_string());
    }
    if domains.is_empty() {
        return Ok(None);
    }

    for (needle, label) in [
        ("codex", "Codex"),
        ("claude code", "Claude Code"),
        ("cursor", "Cursor"),
        ("github copilot", "GitHub Copilot"),
        ("tauri", "Tauri"),
        ("react", "React"),
        ("typescript", "TypeScript"),
        ("rust", "Rust"),
        ("vite", "Vite"),
        ("playwright", "Playwright"),
        ("github actions", "GitHub Actions"),
        ("minimax h3", "MiniMax H3"),
        ("kling", "Kling"),
        ("runway", "Runway"),
        ("veo", "Veo"),
        ("sora", "Sora"),
    ] {
        if joined.contains(needle) {
            tools.insert(label.to_string());
        }
    }

    if contains_any(
        &joined,
        &["verify", "verification", "test", "quality", "uat", "ci"],
    ) {
        goals.insert("verification and reliable delivery".to_string());
    }
    if contains_any(
        &joined,
        &["long task", "long-running", "handoff", "plan", "workflow"],
    ) {
        goals.insert("reliable long-running task structure".to_string());
    }
    if contains_any(&joined, &["multi-agent", "multi agent", "subagent"]) {
        goals.insert("multi-agent coordination".to_string());
    }
    if contains_any(&joined, &["character consistency", "consistent character"]) {
        goals.insert("character consistency".to_string());
    }
    if contains_any(
        &joined,
        &["shot transition", "shot continuity", "camera transition"],
    ) {
        goals.insert("shot continuity and transitions".to_string());
    }
    if joined.contains("windows") {
        constraints.insert("Windows".to_string());
    }
    if contains_any(&joined, &["local-first", "local first", "offline"]) {
        constraints.insert("local-first workflow".to_string());
    }
    if contains_any(&joined, &["cost", "budget", "quota"]) {
        constraints.insert("cost awareness".to_string());
    }

    let mut dates = records
        .iter()
        .map(|(_, timestamp)| coarse_date(timestamp))
        .collect::<Vec<_>>();
    dates.sort();
    let context = DailySearchContext {
        schema_version: CONTEXT_COMPILER_VERSION,
        work_domains: domains.into_iter().take(3).collect(),
        public_tools_and_models: tools.into_iter().take(8).collect(),
        current_goals: goals.into_iter().take(5).collect(),
        non_sensitive_constraints: constraints.into_iter().take(4).collect(),
        evidence_window: EvidenceWindow {
            start_date: dates.first().cloned().unwrap_or_default(),
            end_date: dates.last().cloned().unwrap_or_default(),
        },
        preferred_insight_categories: vec![
            "official product or model updates".to_string(),
            "reproducible workflow improvements".to_string(),
            "practical techniques".to_string(),
        ],
    };
    let context_json = serde_json::to_string(&context)
        .map_err(|_| "Memoryling could not prepare the outbound work context.".to_string())?;
    if context_json.chars().count() > MAX_CONTEXT_CHARS {
        return Err("The minimized work context exceeded its safety limit.".to_string());
    }
    let context_hash = hex_hash(&format!("{source_id}|{context_json}"));
    Ok(Some(CompiledContext {
        outbound: context,
        source_ids: vec![source_id],
        context_json,
        context_hash,
    }))
}

pub(crate) fn consent_contract(context: &CompiledContext) -> Result<(String, String), String> {
    let scope = DailyInsightConsentScopeV1 {
        schema_version: CONSENT_SCHEMA_VERSION,
        revision: CONSENT_REVISION,
        provider: PROVIDER_ID.to_string(),
        model: MODEL_ID.to_string(),
        source_ids: context.source_ids.clone(),
        data_categories: vec![
            "coarse-work-domains".to_string(),
            "public-tools-and-models".to_string(),
            "generic-work-goals".to_string(),
            "non-sensitive-constraints".to_string(),
            "coarse-evidence-window".to_string(),
        ],
        purposes: vec!["daily-source-linked-work-insight".to_string()],
        context_compiler_version: CONTEXT_COMPILER_VERSION,
        max_context_characters: MAX_CONTEXT_CHARS,
        automatic_daily_send: true,
    };
    let json = serde_json::to_string(&scope)
        .map_err(|_| "Memoryling could not prepare the Daily Scout consent.".to_string())?;
    Ok((hex_hash(&json), json))
}

pub(crate) fn relevance_reason(context: &DailySearchContext, locale: &str) -> String {
    let domains = context.work_domains.join(" + ");
    if locale == "zh-TW" {
        format!("因為你近期核准的工作包含：{domains}")
    } else {
        format!("Because your recently approved work includes: {domains}")
    }
}

fn contains_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}

fn coarse_date(timestamp: &str) -> String {
    timestamp.chars().take(10).collect()
}

fn hex_hash(value: &str) -> String {
    Sha256::digest(value.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coarse_compiler_helpers_never_copy_arbitrary_phrases() {
        let private_text = "Client Acme / secret-roadmap is using Codex, Tauri and React on Windows. Verify the long-running workflow.";
        let lower = private_text.to_lowercase();
        assert!(contains_any(&lower, &["codex"]));
        assert!(!coarse_date("2026-08-13T09:12:00Z").contains("T"));
        assert!(!relevance_reason(
            &DailySearchContext {
                schema_version: 1,
                work_domains: vec!["agent-assisted software development".to_string()],
                public_tools_and_models: vec!["Codex".to_string()],
                current_goals: vec![],
                non_sensitive_constraints: vec![],
                evidence_window: EvidenceWindow {
                    start_date: "2026-08-13".to_string(),
                    end_date: "2026-08-13".to_string(),
                },
                preferred_insight_categories: vec![],
            },
            "en"
        )
        .contains("Acme"));
    }
}
