mod context;
mod credential;
mod model;
mod openai;
pub(crate) mod store;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use tauri::{App, AppHandle, Manager, Runtime, State};
use tauri_plugin_opener::OpenerExt;
use time::{format_description::well_known::Rfc3339, OffsetDateTime, UtcOffset};

use crate::{caller::MainCaller, memory};

use context::{compile_recent_work, consent_contract, relevance_reason};
use credential::{CredentialVault, SystemCredentialVault};
use model::LocalNow;
pub(crate) use model::{ConfigureDailyScoutRequest, DailyScoutState, OpenDailyScoutLinkRequest};
use openai::{InsightProvider, OpenAiProvider};

const SCHEDULER_START_DELAY: Duration = Duration::from_secs(5);
const SCHEDULER_INTERVAL: Duration = Duration::from_secs(15 * 60);

#[derive(Default)]
pub(crate) struct DailyScoutService {
    operation: Arc<Mutex<()>>,
}

impl DailyScoutService {
    fn operation(&self) -> Arc<Mutex<()>> {
        Arc::clone(&self.operation)
    }
}

pub(crate) fn setup<R: Runtime>(app: &mut App<R>) {
    let handle = app.handle().clone();
    thread::spawn(move || {
        thread::sleep(SCHEDULER_START_DELAY);
        loop {
            let _ = run_if_due(&handle);
            thread::sleep(SCHEDULER_INTERVAL);
        }
    });
}

#[tauri::command]
pub(crate) fn get_daily_scout_state<R: Runtime>(
    _caller: MainCaller,
    app: AppHandle<R>,
) -> Result<DailyScoutState, String> {
    state_for(&app)
}

#[tauri::command]
pub(crate) fn save_openai_api_key<R: Runtime>(
    _caller: MainCaller,
    app: AppHandle<R>,
    service: State<'_, DailyScoutService>,
    api_key: String,
) -> Result<DailyScoutState, String> {
    let operation = service.operation();
    let _guard = operation
        .lock()
        .map_err(|_| "Memoryling could not reserve Daily Scout settings.".to_string())?;
    SystemCredentialVault.save_key(&api_key)?;
    state_for(&app)
}

#[tauri::command]
pub(crate) async fn test_openai_api_key(
    _caller: MainCaller,
    service: State<'_, DailyScoutService>,
) -> Result<(), String> {
    let operation = service.operation();
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = operation
            .lock()
            .map_err(|_| "connection-test-unavailable".to_string())?;
        let key = SystemCredentialVault
            .load_key()?
            .ok_or_else(|| "missing-key".to_string())?;
        OpenAiProvider::new()?
            .test_connection(&key)
            .map_err(|error| error.code().to_string())
    })
    .await
    .map_err(|_| "connection-test-unavailable".to_string())?
}

#[tauri::command]
pub(crate) fn configure_daily_scout<R: Runtime>(
    _caller: MainCaller,
    app: AppHandle<R>,
    service: State<'_, DailyScoutService>,
    request: ConfigureDailyScoutRequest,
) -> Result<DailyScoutState, String> {
    if !request.consent_accepted {
        return Err("Daily Scout requires explicit consent.".to_string());
    }
    if request.locale != "en" && request.locale != "zh-TW" {
        return Err("Daily Scout received an unsupported language.".to_string());
    }
    let (hour, minute) = parse_delivery_time(&request.delivery_time)?;
    let operation = service.operation();
    let _guard = operation
        .lock()
        .map_err(|_| "Memoryling could not reserve Daily Scout settings.".to_string())?;
    if !SystemCredentialVault.has_key()? {
        return Err("Save an OpenAI API key before enabling Daily Scout.".to_string());
    }
    let store = memory::store_for(&app)?;
    let context = compile_recent_work(&store)?.ok_or_else(|| {
        "Daily Scout needs one approved Codex work record with supported work context.".to_string()
    })?;
    let (scope_hash, scope_json) = consent_contract(&context)?;
    let now = local_now()?;
    store.save_daily_settings(
        &request.locale,
        hour,
        minute,
        &scope_json,
        &scope_hash,
        &now.timestamp,
    )?;
    let state = store.daily_scout_state(true, Some(&context), &now)?;
    drop(_guard);
    let handle = app.clone();
    thread::spawn(move || {
        let _ = run_if_due(&handle);
    });
    Ok(state)
}

#[tauri::command]
pub(crate) fn disable_daily_scout<R: Runtime>(
    _caller: MainCaller,
    app: AppHandle<R>,
    service: State<'_, DailyScoutService>,
) -> Result<DailyScoutState, String> {
    let operation = service.operation();
    let _guard = operation
        .lock()
        .map_err(|_| "Memoryling could not reserve Daily Scout settings.".to_string())?;
    let now = local_now()?;
    memory::store_for(&app)?.disable_daily_scout(&now.timestamp)?;
    state_for(&app)
}

#[tauri::command]
pub(crate) fn delete_openai_api_key<R: Runtime>(
    _caller: MainCaller,
    app: AppHandle<R>,
    service: State<'_, DailyScoutService>,
) -> Result<DailyScoutState, String> {
    let operation = service.operation();
    let _guard = operation
        .lock()
        .map_err(|_| "Memoryling could not reserve Daily Scout settings.".to_string())?;
    let now = local_now()?;
    SystemCredentialVault.delete_key()?;
    memory::store_for(&app)?.disable_daily_scout(&now.timestamp)?;
    state_for(&app)
}

#[tauri::command]
pub(crate) fn clear_daily_scout_history<R: Runtime>(
    _caller: MainCaller,
    app: AppHandle<R>,
    service: State<'_, DailyScoutService>,
) -> Result<DailyScoutState, String> {
    let operation = service.operation();
    let _guard = operation
        .lock()
        .map_err(|_| "Memoryling could not reserve Daily Scout settings.".to_string())?;
    memory::store_for(&app)?.clear_daily_scout_history()?;
    state_for(&app)
}

#[tauri::command]
pub(crate) fn reset_daily_scout<R: Runtime>(
    _caller: MainCaller,
    app: AppHandle<R>,
    service: State<'_, DailyScoutService>,
) -> Result<DailyScoutState, String> {
    let operation = service.operation();
    let _guard = operation
        .lock()
        .map_err(|_| "Memoryling could not reserve Daily Scout settings.".to_string())?;
    let now = local_now()?;
    let store = memory::store_for(&app)?;
    store.disable_daily_scout(&now.timestamp)?;
    store.clear_daily_scout_history()?;
    SystemCredentialVault.delete_key()?;
    state_for(&app)
}

#[tauri::command]
pub(crate) fn mark_daily_insight_read<R: Runtime>(
    _caller: MainCaller,
    app: AppHandle<R>,
    service: State<'_, DailyScoutService>,
) -> Result<DailyScoutState, String> {
    let operation = service.operation();
    let _guard = operation
        .lock()
        .map_err(|_| "Memoryling could not reserve Daily Scout message.".to_string())?;
    let now = local_now()?;
    memory::store_for(&app)?.mark_latest_daily_insight_read(&now.timestamp)?;
    crate::desktop_shell::emit_creature_state_changed(&app);
    state_for(&app)
}

#[tauri::command]
pub(crate) fn open_daily_scout_link<R: Runtime>(
    _caller: MainCaller,
    app: AppHandle<R>,
    request: OpenDailyScoutLinkRequest,
) -> Result<(), String> {
    let store = memory::store_for(&app)?;
    let url = daily_scout_link_for(&store, &request)?;
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|_| "Memoryling could not open the link in your browser.".to_string())
}

fn daily_scout_link_for(
    store: &memory::store::MemoryStore,
    request: &OpenDailyScoutLinkRequest,
) -> Result<String, String> {
    let url = match request.kind.as_str() {
        "api-keys" if request.url.is_none() => "https://platform.openai.com/api-keys".to_string(),
        "quickstart" if request.url.is_none() => {
            "https://developers.openai.com/api/docs/quickstart".to_string()
        }
        "citation" => {
            let url = request
                .url
                .as_deref()
                .ok_or_else(|| "Memoryling received an incomplete source link.".to_string())?;
            if !store.has_daily_citation_url(url)? {
                return Err("Memoryling blocked an unrecognized source link.".to_string());
            }
            url.to_string()
        }
        _ => return Err("Memoryling blocked an unrecognized external link.".to_string()),
    };
    Ok(url)
}

fn run_if_due<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let Some(service) = app.try_state::<DailyScoutService>() else {
        return Ok(());
    };
    let operation = service.operation();
    let _guard = operation
        .lock()
        .map_err(|_| "Memoryling could not reserve the Daily Scout run.".to_string())?;
    let store = memory::store_for(app)?;
    let now = local_now()?;
    let provider = OpenAiProvider::new()?;
    let result = run_if_due_with(&store, &SystemCredentialVault, &provider, &now);
    crate::desktop_shell::emit_creature_state_changed(app);
    result
}

fn run_if_due_with(
    store: &memory::store::MemoryStore,
    vault: &dyn CredentialVault,
    provider: &dyn InsightProvider,
    now: &LocalNow,
) -> Result<(), String> {
    store.recover_interrupted_daily_attempts(&now.timestamp)?;
    let Some(settings) = store.daily_settings()? else {
        return Ok(());
    };
    if !settings.enabled
        || now.local_minutes
            < u16::from(settings.delivery_hour) * 60 + u16::from(settings.delivery_minute)
    {
        return Ok(());
    }
    let Some(key) = vault.load_key()? else {
        return Ok(());
    };
    let Some(context) = compile_recent_work(store)? else {
        return Ok(());
    };
    let (expected_hash, _) = consent_contract(&context)?;
    if expected_hash != settings.consent_scope_hash {
        store.disable_daily_scout(&now.timestamp)?;
        return Ok(());
    }
    if !store.reserve_daily_attempt(now, &context.context_hash)? {
        return Ok(());
    }
    match provider.search(
        &key,
        &context.outbound,
        &context.context_json,
        &settings.locale,
        &now.local_date,
    ) {
        Ok(insight) => {
            let relevance = relevance_reason(&context.outbound, &settings.locale);
            if store
                .finish_daily_attempt_success(now, &context, &insight, &relevance)
                .is_err()
            {
                store.finish_daily_attempt_failure(
                    &now.local_date,
                    &now.timestamp,
                    "source-changed",
                )?;
            }
        }
        Err(error) => {
            store.finish_daily_attempt_failure(&now.local_date, &now.timestamp, error.code())?
        }
    }
    Ok(())
}

fn state_for<R: Runtime>(app: &AppHandle<R>) -> Result<DailyScoutState, String> {
    let store = memory::store_for(app)?;
    let now = local_now()?;
    let has_key = SystemCredentialVault.has_key()?;
    let context = compile_recent_work(&store)?;
    store.daily_scout_state(has_key, context.as_ref(), &now)
}

fn parse_delivery_time(value: &str) -> Result<(u8, u8), String> {
    let (hour, minute) = value
        .split_once(':')
        .ok_or_else(|| "Choose a valid daytime delivery time.".to_string())?;
    let hour = hour
        .parse::<u8>()
        .map_err(|_| "Choose a valid daytime delivery time.".to_string())?;
    let minute = minute
        .parse::<u8>()
        .map_err(|_| "Choose a valid daytime delivery time.".to_string())?;
    if !(8..=21).contains(&hour) || minute > 59 {
        return Err("Choose a delivery time between 08:00 and 21:59.".to_string());
    }
    Ok((hour, minute))
}

fn local_now() -> Result<LocalNow, String> {
    let utc = OffsetDateTime::now_utc();
    let offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    let local = utc.to_offset(offset);
    let timestamp = utc
        .format(&Rfc3339)
        .map_err(|_| "Memoryling could not read the current time.".to_string())?;
    let timezone = iana_time_zone::get_timezone().unwrap_or_else(|_| offset.to_string());
    Ok(LocalNow {
        local_date: local.date().to_string(),
        local_minutes: u16::from(local.hour()) * 60 + u16::from(local.minute()),
        timezone,
        timestamp,
    })
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        sync::Mutex,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;
    use crate::daily_scout::{
        credential::MemoryCredentialVault,
        model::{DailyCitation, DailySearchContext, ProviderError, ProviderInsight},
    };

    struct FakeProvider {
        calls: Mutex<usize>,
        result: Result<ProviderInsight, ProviderError>,
    }

    impl InsightProvider for FakeProvider {
        fn test_connection(&self, _api_key: &str) -> Result<(), ProviderError> {
            Ok(())
        }

        fn search(
            &self,
            _api_key: &str,
            _context: &DailySearchContext,
            _context_json: &str,
            _locale: &str,
            _local_date: &str,
        ) -> Result<ProviderInsight, ProviderError> {
            *self.calls.lock().expect("calls") += 1;
            self.result.clone()
        }
    }

    fn temporary_store() -> (memory::store::MemoryStore, PathBuf) {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "memoryling-daily-scout-tests-{}-{nonce}",
            std::process::id()
        ));
        (
            memory::store::MemoryStore::new(directory.join("memoryling.sqlite3")),
            directory,
        )
    }

    fn seed_supported_work(store: &memory::store::MemoryStore) {
        let connection = store.open_connection().expect("open store");
        connection.execute(
            "INSERT INTO source_imports
                (source_id, adapter_id, adapter_version, display_name, source_locator, source_content_hash)
             VALUES ('thread-source', 'codex-app-server-thread', 1, 'Codex work record',
                     'codex-app-server://thread/redacted', ?1)",
            ["a".repeat(64)],
        ).expect("source");
        connection
            .execute(
                "INSERT INTO source_consent_scopes
                (source_id, schema_version, consent_revision, scope_json, scope_hash)
             VALUES ('thread-source', 1, 1, '{}', ?1)",
                ["b".repeat(64)],
            )
            .expect("scope");
        connection.execute(
            "INSERT INTO memory_events
                (id, schema_version, source_id, source_record_id, source_timestamp,
                 kind, normalized_text, content_hash)
             VALUES ('event-1', 1, 'thread-source', 'record-redacted',
                     '2026-08-13T01:00:00Z', 'completion',
                     'Completed a Codex and Tauri agent coding workflow with verification on Windows.', ?1)",
            ["c".repeat(64)],
        ).expect("event");
    }

    #[test]
    fn one_reserved_local_date_prevents_duplicate_paid_searches() {
        let (store, directory) = temporary_store();
        seed_supported_work(&store);
        let context = compile_recent_work(&store)
            .expect("compile")
            .expect("context");
        assert!(!context.context_json.contains("record-redacted"));
        assert!(!context.context_json.contains("Completed a Codex"));
        let (scope_hash, scope_json) = consent_contract(&context).expect("consent");
        store
            .save_daily_settings(
                "en",
                10,
                0,
                &scope_json,
                &scope_hash,
                "2026-08-13T02:00:00Z",
            )
            .expect("settings");
        let now = LocalNow {
            local_date: "2026-08-13".to_string(),
            local_minutes: 11 * 60,
            timezone: "Asia/Taipei".to_string(),
            timestamp: "2026-08-13T03:00:00Z".to_string(),
        };
        let provider = FakeProvider {
            calls: Mutex::new(0),
            result: Ok(ProviderInsight {
                pet_message: "I found a useful verified workflow tip.".to_string(),
                strength: "practical".to_string(),
                citations: vec![DailyCitation {
                    title: "Official source".to_string(),
                    url: "https://example.com/source".to_string(),
                }],
            }),
        };
        let vault = MemoryCredentialVault::with_key(Some("synthetic-key-material-never-sent"));
        run_if_due_with(&store, &vault, &provider, &now).expect("first run");
        run_if_due_with(&store, &vault, &provider, &now).expect("second check");
        assert_eq!(*provider.calls.lock().expect("calls"), 1);
        let state = store
            .daily_scout_state(true, Some(&context), &now)
            .expect("state");
        assert_eq!(state.status, "ready");
        assert_eq!(state.latest_insight.expect("insight").citations.len(), 1);
        assert_eq!(
            daily_scout_link_for(
                &store,
                &OpenDailyScoutLinkRequest {
                    kind: "citation".to_string(),
                    url: Some("https://example.com/source".to_string()),
                },
            )
            .expect("persisted citation"),
            "https://example.com/source"
        );
        assert!(daily_scout_link_for(
            &store,
            &OpenDailyScoutLinkRequest {
                kind: "citation".to_string(),
                url: Some("https://unrecognized.example/".to_string()),
            },
        )
        .is_err());
        fs::remove_dir_all(directory).expect("cleanup");
    }

    #[test]
    fn failed_attempt_is_not_retried_on_the_same_date() {
        let (store, directory) = temporary_store();
        seed_supported_work(&store);
        let context = compile_recent_work(&store)
            .expect("compile")
            .expect("context");
        let (scope_hash, scope_json) = consent_contract(&context).expect("consent");
        store
            .save_daily_settings(
                "zh-TW",
                10,
                0,
                &scope_json,
                &scope_hash,
                "2026-08-13T02:00:00Z",
            )
            .expect("settings");
        let now = LocalNow {
            local_date: "2026-08-13".to_string(),
            local_minutes: 12 * 60,
            timezone: "Asia/Taipei".to_string(),
            timestamp: "2026-08-13T04:00:00Z".to_string(),
        };
        let provider = FakeProvider {
            calls: Mutex::new(0),
            result: Err(ProviderError::Authentication),
        };
        let vault = MemoryCredentialVault::with_key(Some("synthetic-key-material-never-sent"));
        run_if_due_with(&store, &vault, &provider, &now).expect("first run");
        run_if_due_with(&store, &vault, &provider, &now).expect("second check");
        assert_eq!(*provider.calls.lock().expect("calls"), 1);
        let state = store
            .daily_scout_state(true, Some(&context), &now)
            .expect("state");
        assert_eq!(state.status, "failed");
        assert_eq!(
            state
                .today_attempt
                .and_then(|attempt| attempt.error_code)
                .as_deref(),
            Some("authentication")
        );
        fs::remove_dir_all(directory).expect("cleanup");
    }
}
