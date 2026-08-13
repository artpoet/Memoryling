mod caller;
#[path = "../command_manifest.rs"]
mod command_manifest;
mod daily_scout;
mod desktop_shell;
mod memory;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            let _ = desktop_shell::open_detail_and_finish_onboarding(app);
        }))
        .manage(memory::PendingImports::default())
        .manage(daily_scout::DailyScoutService::default())
        .invoke_handler(tauri::generate_handler![
            memory::list_memory_sources,
            memory::preview_memory_source,
            memory::list_codex_threads,
            memory::preview_codex_thread,
            memory::cancel_memory_preview,
            memory::get_memory_state,
            memory::approve_memory_import,
            memory::forget_memory_source,
            memory::get_creature_render_state,
            daily_scout::get_daily_scout_state,
            daily_scout::save_openai_api_key,
            daily_scout::test_openai_api_key,
            daily_scout::configure_daily_scout,
            daily_scout::disable_daily_scout,
            daily_scout::delete_openai_api_key,
            daily_scout::clear_daily_scout_history,
            daily_scout::reset_daily_scout,
            daily_scout::mark_daily_insight_read,
            daily_scout::open_daily_scout_link,
            desktop_shell::show_pet_context_menu,
            desktop_shell::get_pet_shell_state,
            desktop_shell::dismiss_pet_onboarding,
            desktop_shell::start_pet_dragging,
            desktop_shell::reset_pet_onboarding
        ])
        .on_menu_event(desktop_shell::handle_menu_event)
        .on_window_event(desktop_shell::handle_window_event)
        .setup(|app| {
            desktop_shell::setup(app)?;
            daily_scout::setup(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod invoke_security_tests {
    use std::sync::atomic::Ordering;

    use serde_json::{json, Value};
    use tauri::{
        ipc::{CallbackFn, InvokeBody},
        test::{get_ipc_response, mock_builder, mock_context, noop_assets, INVOKE_KEY},
        webview::InvokeRequest,
        WebviewWindowBuilder,
    };

    use super::{
        daily_scout::{self, DailyScoutService},
        memory::{self, PendingImports, SENSITIVE_HANDLER_ENTRIES},
    };

    fn request(command: &str, body: Value) -> InvokeRequest {
        InvokeRequest {
            cmd: command.into(),
            callback: CallbackFn(0),
            error: CallbackFn(1),
            url: "http://tauri.localhost"
                .parse()
                .expect("test invoke URL should parse"),
            body: InvokeBody::Json(body),
            headers: Default::default(),
            invoke_key: INVOKE_KEY.to_string(),
        }
    }

    fn sensitive_probes() -> Vec<(&'static str, Value)> {
        vec![
            ("list_memory_sources", json!({})),
            (
                "preview_memory_source",
                json!({ "sourceId": "codex.synthetic.first-memory" }),
            ),
            ("list_codex_threads", json!({})),
            (
                "preview_codex_thread",
                json!({ "catalogId": "not-a-catalog", "candidateId": "not-a-candidate" }),
            ),
            (
                "cancel_memory_preview",
                json!({ "previewId": "not-a-preview" }),
            ),
            ("get_memory_state", json!({})),
            (
                "approve_memory_import",
                json!({
                    "request": {
                        "previewId": "not-a-preview",
                        "sourceId": "codex.synthetic.first-memory",
                        "selectedRecordIds": ["not-a-record"]
                    }
                }),
            ),
            (
                "forget_memory_source",
                json!({ "sourceId": "codex.synthetic.first-memory" }),
            ),
            ("get_daily_scout_state", json!({})),
            (
                "save_openai_api_key",
                json!({ "apiKey": "synthetic-key-never-reaches-handler" }),
            ),
            ("test_openai_api_key", json!({})),
            (
                "configure_daily_scout",
                json!({
                    "request": {
                        "locale": "en",
                        "deliveryTime": "10:00",
                        "consentAccepted": true
                    }
                }),
            ),
            ("disable_daily_scout", json!({})),
            ("delete_openai_api_key", json!({})),
            ("clear_daily_scout_history", json!({})),
            ("reset_daily_scout", json!({})),
            ("mark_daily_insight_read", json!({})),
            (
                "open_daily_scout_link",
                json!({ "request": { "kind": "api-keys" } }),
            ),
        ]
    }

    #[test]
    fn production_acl_and_caller_guard_each_deny_pet_sensitive_invokes() {
        {
            SENSITIVE_HANDLER_ENTRIES.store(0, Ordering::SeqCst);
            let app = mock_builder()
                .manage(PendingImports::default())
                .manage(DailyScoutService::default())
                .invoke_handler(tauri::generate_handler![
                    memory::list_memory_sources,
                    memory::preview_memory_source,
                    memory::list_codex_threads,
                    memory::preview_codex_thread,
                    memory::cancel_memory_preview,
                    memory::get_memory_state,
                    memory::approve_memory_import,
                    memory::forget_memory_source,
                    daily_scout::get_daily_scout_state,
                    daily_scout::save_openai_api_key,
                    daily_scout::test_openai_api_key,
                    daily_scout::configure_daily_scout,
                    daily_scout::disable_daily_scout,
                    daily_scout::delete_openai_api_key,
                    daily_scout::clear_daily_scout_history,
                    daily_scout::reset_daily_scout,
                    daily_scout::mark_daily_insight_read,
                    daily_scout::open_daily_scout_link,
                ])
                .build(tauri::generate_context!(test = true))
                .expect("production-authority mock app should build");
            let pet = WebviewWindowBuilder::new(&app, "pet", Default::default())
                .build()
                .expect("pet mock webview should build");
            for (command, body) in sensitive_probes() {
                let error = get_ipc_response(&pet, request(command, body))
                    .expect_err("production ACL must deny pet invoke");
                let diagnostic = error
                    .as_str()
                    .expect("ACL denial should be a diagnostic string");
                assert!(diagnostic.contains(&format!("{command} not allowed")));
                assert!(diagnostic.contains("window \"pet\", webview \"pet\""));
                assert_eq!(
                    SENSITIVE_HANDLER_ENTRIES.load(Ordering::SeqCst),
                    0,
                    "ACL allowed {command} to enter its handler body"
                );
            }

            let main = WebviewWindowBuilder::new(&app, "main", Default::default())
                .build()
                .expect("main mock webview should build");
            let response = get_ipc_response(&main, request("list_memory_sources", json!({})))
                .expect("production ACL should allow main list command");
            let sources = response
                .deserialize::<Value>()
                .expect("main list response should deserialize");
            assert!(sources.is_array());
            assert_eq!(SENSITIVE_HANDLER_ENTRIES.load(Ordering::SeqCst), 1);
        }

        SENSITIVE_HANDLER_ENTRIES.store(0, Ordering::SeqCst);
        let app = mock_builder()
            .manage(PendingImports::default())
            .manage(DailyScoutService::default())
            .invoke_handler(tauri::generate_handler![
                memory::list_memory_sources,
                memory::preview_memory_source,
                memory::list_codex_threads,
                memory::preview_codex_thread,
                memory::cancel_memory_preview,
                memory::get_memory_state,
                memory::approve_memory_import,
                memory::forget_memory_source,
                daily_scout::get_daily_scout_state,
                daily_scout::save_openai_api_key,
                daily_scout::test_openai_api_key,
                daily_scout::configure_daily_scout,
                daily_scout::disable_daily_scout,
                daily_scout::delete_openai_api_key,
                daily_scout::clear_daily_scout_history,
                daily_scout::reset_daily_scout,
                daily_scout::mark_daily_insight_read,
                daily_scout::open_daily_scout_link,
            ])
            .build(mock_context(noop_assets()))
            .expect("empty-authority caller-guard mock app should build");
        let pet = WebviewWindowBuilder::new(&app, "pet", Default::default())
            .build()
            .expect("pet caller-guard webview should build");
        let caller_denied =
            json!("This command is not available from the current Memoryling surface.");
        for (command, body) in sensitive_probes() {
            let error = get_ipc_response(&pet, request(command, body))
                .expect_err("MainCaller must deny pet invoke when ACL is bypassed in the harness");
            assert_eq!(error, caller_denied);
            assert_eq!(
                SENSITIVE_HANDLER_ENTRIES.load(Ordering::SeqCst),
                0,
                "caller guard allowed {command} to enter its handler body"
            );
        }
    }
}
