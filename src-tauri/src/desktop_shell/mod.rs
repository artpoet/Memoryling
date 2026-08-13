mod lifecycle;
mod model;
mod position;
mod settings;

use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Mutex,
    },
};

use tauri::{
    menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    App, AppHandle, Emitter, EventTarget, LogicalPosition, Manager, State,
};

use crate::{
    caller::{MainCaller, PetCaller},
    memory::{self, CreatureRenderState},
};

pub(crate) use lifecycle::{handle_window_event, open_detail_and_finish_onboarding};
use model::{
    ContextMenuTrigger, CreatureStateChanged, DetailReset, PetShellState, ShellMode, ShellSettings,
};

const MENU_OPEN: &str = "shell.open";
const MENU_SHOW_PET: &str = "shell.show-pet";
const MENU_ALWAYS_ON_TOP: &str = "shell.always-on-top";
const MENU_HIDE_PET: &str = "shell.hide-pet";
const MENU_QUIT: &str = "shell.quit";

const CREATURE_STATE_EVENT: &str = "memoryling://creature-state-changed";
const DETAIL_RESET_EVENT: &str = "memoryling://detail-reset";
const PET_SHELL_STATE_EVENT: &str = "memoryling://pet-shell-state-changed";

struct ShellMenus {
    pet_context: Menu<tauri::Wry>,
    always_on_top_items: Vec<CheckMenuItem<tauri::Wry>>,
}

pub(crate) struct DesktopShellState {
    settings: Mutex<ShellSettings>,
    settings_path: PathBuf,
    menus: ShellMenus,
    mode: Mutex<ShellMode>,
    move_revision: AtomicU64,
    move_save_scheduled: AtomicBool,
    topology_stop: AtomicBool,
}

pub(crate) fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let settings_path = app
        .path()
        .app_local_data_dir()?
        .join("desktop-shell-v1.json");
    let settings = settings::load(&settings_path).unwrap_or_default();
    let (pet_context, pet_always_on_top) = build_shell_menu(app.handle(), settings.always_on_top)?;
    let (tray_menu, tray_always_on_top) = build_shell_menu(app.handle(), settings.always_on_top)?;
    let state = DesktopShellState {
        settings: Mutex::new(settings.clone()),
        settings_path,
        menus: ShellMenus {
            pet_context,
            always_on_top_items: vec![pet_always_on_top, tray_always_on_top],
        },
        mode: Mutex::new(ShellMode::Starting),
        move_revision: AtomicU64::new(0),
        move_save_scheduled: AtomicBool::new(false),
        topology_stop: AtomicBool::new(false),
    };
    if !app.manage(state) {
        return Err("Memoryling could not initialize its desktop shell state.".into());
    }

    let mut tray = TrayIconBuilder::with_id("memoryling-tray")
        .tooltip("Memoryling | 記憶獸")
        .menu(&tray_menu)
        .show_menu_on_left_click(true);
    if let Some(icon) = app.default_window_icon().cloned() {
        tray = tray.icon(icon);
    }
    tray.build(app)?;

    let pet = app
        .get_webview_window("pet")
        .ok_or("Memoryling could not find its pet window.")?;
    pet.set_always_on_top(settings.always_on_top)?;
    position::apply_pet_size(app.handle(), settings.onboarding_dismissed)
        .map_err(std::io::Error::other)?;

    let main_is_visible = app
        .get_webview_window("main")
        .and_then(|main| main.is_visible().ok())
        .unwrap_or(false);
    if main_is_visible {
        set_mode(app.handle(), ShellMode::MainOpen);
    } else {
        position::restore_pet_position(app.handle()).map_err(std::io::Error::other)?;
        pet.show()?;
        set_mode(app.handle(), ShellMode::PetVisible);
    }
    position::spawn_monitor_poll(app.handle());
    Ok(())
}

fn build_shell_menu(
    app: &AppHandle,
    always_on_top: bool,
) -> tauri::Result<(Menu<tauri::Wry>, CheckMenuItem<tauri::Wry>)> {
    let open = MenuItem::with_id(
        app,
        MENU_OPEN,
        "Open Memoryling / 開啟 Memoryling",
        true,
        None::<&str>,
    )?;
    let status = MenuItem::with_id(
        app,
        "shell.memory-access-off",
        "Memory access off / 記憶存取關閉",
        false,
        None::<&str>,
    )?;
    let show_pet = MenuItem::with_id(
        app,
        MENU_SHOW_PET,
        "Show pet / 顯示寵物",
        true,
        None::<&str>,
    )?;
    let always = CheckMenuItem::with_id(
        app,
        MENU_ALWAYS_ON_TOP,
        "Always on top / 永遠顯示在最上層",
        true,
        always_on_top,
        None::<&str>,
    )?;
    let hide_pet = MenuItem::with_id(
        app,
        MENU_HIDE_PET,
        "Hide pet / 隱藏寵物",
        true,
        None::<&str>,
    )?;
    let quit = MenuItem::with_id(
        app,
        MENU_QUIT,
        "Quit Memoryling / 結束 Memoryling",
        true,
        None::<&str>,
    )?;
    let separator = PredefinedMenuItem::separator(app)?;
    let menu = Menu::with_items(
        app,
        &[
            &open, &status, &show_pet, &always, &hide_pet, &separator, &quit,
        ],
    )?;
    Ok((menu, always))
}

pub(crate) fn handle_menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id().as_ref() {
        MENU_OPEN => {
            let _ = lifecycle::open_detail_and_finish_onboarding(app);
        }
        MENU_SHOW_PET => {
            let _ = lifecycle::return_to_pet(app);
        }
        MENU_ALWAYS_ON_TOP => {
            let _ = toggle_always_on_top(app);
        }
        MENU_HIDE_PET => {
            let _ = lifecycle::hide_pet(app);
        }
        MENU_QUIT => {
            set_mode(app, ShellMode::Quitting);
            if let Some(state) = app.try_state::<DesktopShellState>() {
                state.topology_stop.store(true, Ordering::Release);
            }
            app.exit(0);
        }
        _ => {}
    }
}

#[tauri::command]
pub(crate) fn show_pet_context_menu(
    caller: PetCaller,
    state: State<'_, DesktopShellState>,
    trigger: ContextMenuTrigger,
) -> Result<(), String> {
    match trigger {
        ContextMenuTrigger::Pointer => caller
            .0
            .popup_menu(&state.menus.pet_context)
            .map_err(|_| "Memoryling could not open its pet menu.".to_string()),
        ContextMenuTrigger::Keyboard => {
            let scale = caller
                .0
                .scale_factor()
                .map_err(|_| "Memoryling could not position its pet menu.".to_string())?;
            let size = caller
                .0
                .inner_size()
                .map_err(|_| "Memoryling could not position its pet menu.".to_string())?
                .to_logical::<f64>(scale);
            caller
                .0
                .popup_menu_at(
                    &state.menus.pet_context,
                    LogicalPosition::new(16.0, (size.height - 24.0).max(0.0)),
                )
                .map_err(|_| "Memoryling could not open its pet menu.".to_string())
        }
    }
}

#[tauri::command]
pub(crate) fn get_pet_shell_state(
    _caller: PetCaller,
    state: State<'_, DesktopShellState>,
) -> Result<PetShellState, String> {
    state
        .settings
        .lock()
        .map(|settings| settings.public_state())
        .map_err(|_| "Memoryling could not access its desktop settings.".to_string())
}

#[tauri::command]
pub(crate) fn dismiss_pet_onboarding(
    _caller: PetCaller,
    app: AppHandle,
) -> Result<PetShellState, String> {
    mark_onboarding_dismissed(&app, true)
}

#[tauri::command]
pub(crate) fn start_pet_dragging(caller: PetCaller) -> Result<(), String> {
    caller
        .0
        .start_dragging()
        .map_err(|_| "Memoryling could not start dragging its pet window.".to_string())
}

#[tauri::command]
pub(crate) fn reset_pet_onboarding(
    _caller: MainCaller,
    app: AppHandle,
) -> Result<PetShellState, String> {
    mark_onboarding_dismissed(&app, false)
}

pub(crate) fn mark_onboarding_dismissed(
    app: &AppHandle,
    dismissed: bool,
) -> Result<PetShellState, String> {
    let public = position::resize_pet_and_commit_onboarding(app, dismissed)?;
    emit_pet_shell_state(app, public.clone());
    Ok(public)
}

fn toggle_always_on_top(app: &AppHandle) -> Result<PetShellState, String> {
    let current = shell_settings_snapshot(app).always_on_top;
    let next = !current;
    let pet = app
        .get_webview_window("pet")
        .ok_or_else(|| "Memoryling could not find its pet window.".to_string())?;
    pet.set_always_on_top(next)
        .map_err(|_| "Memoryling could not update its pet window preference.".to_string())?;
    let public = match update_settings(app, |settings| settings.always_on_top = next) {
        Ok(public) => public,
        Err(error) => {
            let _ = pet.set_always_on_top(current);
            return Err(error);
        }
    };
    if let Some(state) = app.try_state::<DesktopShellState>() {
        for item in &state.menus.always_on_top_items {
            let _ = item.set_checked(next);
        }
    }
    emit_pet_shell_state(app, public.clone());
    Ok(public)
}

const CREATURE_STATE_TARGETS: [&str; 2] = ["pet", "main"];

pub(crate) fn emit_creature_state_changed<R: tauri::Runtime>(app: &AppHandle<R>) {
    let Ok(store) = memory::store_for(app) else {
        return;
    };
    let Ok(CreatureRenderState { revision, .. }) = store.creature_render_state() else {
        return;
    };
    for label in CREATURE_STATE_TARGETS {
        let _ = app.emit_to(
            EventTarget::webview_window(label),
            CREATURE_STATE_EVENT,
            CreatureStateChanged {
                revision: revision.clone(),
            },
        );
    }
}

pub(crate) fn emit_detail_reset(app: &AppHandle) {
    let _ = app.emit_to(
        EventTarget::webview_window("main"),
        DETAIL_RESET_EVENT,
        DetailReset::default(),
    );
}

fn emit_pet_shell_state(app: &AppHandle, state: PetShellState) {
    let _ = app.emit_to(
        EventTarget::webview_window("pet"),
        PET_SHELL_STATE_EVENT,
        state,
    );
}

pub(crate) fn update_settings(
    app: &AppHandle,
    update: impl FnOnce(&mut ShellSettings),
) -> Result<PetShellState, String> {
    let state = app
        .try_state::<DesktopShellState>()
        .ok_or_else(|| "Memoryling's desktop settings are not ready.".to_string())?;
    let mut settings = state
        .settings
        .lock()
        .map_err(|_| "Memoryling could not access its desktop settings.".to_string())?;
    let mut candidate = settings.clone();
    update(&mut candidate);
    settings::save(&state.settings_path, &candidate)?;
    *settings = candidate;
    Ok(settings.public_state())
}

pub(crate) fn shell_settings_snapshot(app: &AppHandle) -> ShellSettings {
    app.try_state::<DesktopShellState>()
        .and_then(|state| state.settings.lock().ok().map(|settings| settings.clone()))
        .unwrap_or_default()
}

pub(crate) fn set_mode(app: &AppHandle, mode: ShellMode) {
    if let Some(state) = app.try_state::<DesktopShellState>() {
        if let Ok(mut current) = state.mode.lock() {
            *current = mode;
        }
    }
}

pub(crate) fn shell_mode(app: &AppHandle) -> ShellMode {
    app.try_state::<DesktopShellState>()
        .and_then(|state| state.mode.lock().ok().map(|mode| *mode))
        .unwrap_or(ShellMode::Starting)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use serde_json::Value;

    use super::CREATURE_STATE_TARGETS;
    use crate::command_manifest::{MAIN_COMMANDS, PET_COMMANDS, SENSITIVE_MEMORY_COMMANDS};

    fn capability(path: &str) -> Value {
        let source = match path {
            "main" => include_str!("../../capabilities/main.json"),
            "pet" => include_str!("../../capabilities/pet.json"),
            _ => unreachable!(),
        };
        serde_json::from_str(source).expect("capability should be valid JSON")
    }

    fn app_permissions(capability: &Value) -> BTreeSet<String> {
        capability["permissions"]
            .as_array()
            .expect("permissions should be an array")
            .iter()
            .filter_map(Value::as_str)
            .filter(|permission| permission.starts_with("allow-"))
            .map(ToOwned::to_owned)
            .collect()
    }

    fn expected_permissions(commands: &[&str]) -> BTreeSet<String> {
        commands
            .iter()
            .map(|command| format!("allow-{}", command.replace('_', "-")))
            .collect()
    }

    #[test]
    fn capabilities_are_local_exact_and_content_separated() {
        let main = capability("main");
        let pet = capability("pet");
        assert_eq!(main["local"], true);
        assert_eq!(pet["local"], true);
        assert!(main.get("remote").is_none());
        assert!(pet.get("remote").is_none());
        assert_eq!(main["webviews"], serde_json::json!(["main"]));
        assert_eq!(pet["webviews"], serde_json::json!(["pet"]));
        assert_eq!(app_permissions(&main), expected_permissions(MAIN_COMMANDS));
        assert_eq!(app_permissions(&pet), expected_permissions(PET_COMMANDS));
        assert!(!main["permissions"]
            .as_array()
            .unwrap()
            .contains(&Value::String("core:default".to_string())));
        assert!(!pet["permissions"]
            .as_array()
            .unwrap()
            .contains(&Value::String("core:default".to_string())));

        for capability in [&main, &pet] {
            let permissions = capability["permissions"].as_array().unwrap();
            assert!(permissions.iter().all(|permission| {
                permission
                    .as_str()
                    .is_some_and(|value| !value.contains('*') && !value.starts_with("deny-"))
            }));
        }

        let pet_core = pet["permissions"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(Value::as_str)
            .filter(|permission| permission.starts_with("core:"))
            .collect::<BTreeSet<_>>();
        assert_eq!(
            pet_core,
            BTreeSet::from(["core:event:allow-listen", "core:event:allow-unlisten",])
        );
        assert_eq!(CREATURE_STATE_TARGETS, ["pet", "main"]);

        assert_eq!(
            SENSITIVE_MEMORY_COMMANDS,
            [
                "list_memory_sources",
                "preview_memory_source",
                "list_codex_threads",
                "preview_codex_thread",
                "cancel_memory_preview",
                "get_memory_state",
                "approve_memory_import",
                "forget_memory_source",
                "get_daily_scout_state",
                "save_openai_api_key",
                "test_openai_api_key",
                "configure_daily_scout",
                "disable_daily_scout",
                "delete_openai_api_key",
                "clear_daily_scout_history",
                "reset_daily_scout",
                "mark_daily_insight_read",
                "open_daily_scout_link",
            ]
        );
        let sensitive = expected_permissions(SENSITIVE_MEMORY_COMMANDS);
        assert!(sensitive.is_subset(&app_permissions(&main)));
        assert!(app_permissions(&pet).is_disjoint(&sensitive));
    }

    #[test]
    fn configured_windows_are_precreated_hidden_and_unique() {
        let config: Value = serde_json::from_str(include_str!("../../tauri.conf.json"))
            .expect("tauri config should be valid JSON");
        let windows = config["app"]["windows"].as_array().unwrap();
        assert_eq!(windows.len(), 2);
        assert_eq!(windows[0]["label"], "main");
        assert_eq!(windows[1]["label"], "pet");
        assert_eq!(windows[0]["visible"], false);
        assert_eq!(windows[1]["visible"], false);
        assert_eq!(windows[1]["transparent"], true);
        assert_eq!(windows[1]["skipTaskbar"], true);
        assert_eq!(windows[1]["closable"], false);
        assert_ne!(windows[0].get("closable"), Some(&Value::Bool(false)));
    }
}
