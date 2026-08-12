use tauri::{Manager, Window, WindowEvent};

use crate::memory::PendingImports;

use super::{
    emit_detail_reset, mark_onboarding_dismissed,
    model::ShellMode,
    position::{apply_pet_size, restore_pet_position},
    set_mode, shell_settings_snapshot,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SurfaceOperation {
    MainUnminimize,
    MainShow,
    MainFocus,
    MainHide,
    PetShow,
    PetHide,
}

trait SurfaceOperations {
    fn perform(&mut self, operation: SurfaceOperation) -> Result<(), ()>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TransitionOutcome {
    mode: ShellMode,
    failure: Option<SurfaceOperation>,
}

struct TauriSurfaceOperations<'a> {
    app: &'a tauri::AppHandle,
    main: tauri::WebviewWindow,
    pet: tauri::WebviewWindow,
}

impl SurfaceOperations for TauriSurfaceOperations<'_> {
    fn perform(&mut self, operation: SurfaceOperation) -> Result<(), ()> {
        match operation {
            SurfaceOperation::MainUnminimize => self.main.unminimize().map_err(|_| ()),
            SurfaceOperation::MainShow => self.main.show().map_err(|_| ()),
            SurfaceOperation::MainFocus => self.main.set_focus().map_err(|_| ()),
            SurfaceOperation::MainHide => self.main.hide().map_err(|_| ()),
            SurfaceOperation::PetShow => show_pet_surface(self.app).map_err(|_| ()),
            SurfaceOperation::PetHide => self.pet.hide().map_err(|_| ()),
        }
    }
}

fn operation_error(operation: SurfaceOperation) -> String {
    match operation {
        SurfaceOperation::MainUnminimize => {
            "Memoryling could not restore its detail window.".to_string()
        }
        SurfaceOperation::MainShow => "Memoryling could not show its detail window.".to_string(),
        SurfaceOperation::MainFocus => "Memoryling could not focus its detail window.".to_string(),
        SurfaceOperation::MainHide => "Memoryling could not hide its detail window.".to_string(),
        SurfaceOperation::PetShow => "Memoryling could not show its pet window.".to_string(),
        SurfaceOperation::PetHide => "Memoryling could not hide its pet window.".to_string(),
    }
}

fn run_open_detail(operations: &mut impl SurfaceOperations) -> TransitionOutcome {
    if operations
        .perform(SurfaceOperation::MainUnminimize)
        .is_err()
    {
        let _ = operations.perform(SurfaceOperation::PetShow);
        return TransitionOutcome {
            mode: ShellMode::PetVisible,
            failure: Some(SurfaceOperation::MainUnminimize),
        };
    }
    if operations.perform(SurfaceOperation::MainShow).is_err() {
        let mode = if operations.perform(SurfaceOperation::MainHide).is_ok() {
            let _ = operations.perform(SurfaceOperation::PetShow);
            ShellMode::PetVisible
        } else {
            let _ = operations.perform(SurfaceOperation::PetHide);
            ShellMode::MainOpen
        };
        return TransitionOutcome {
            mode,
            failure: Some(SurfaceOperation::MainShow),
        };
    }

    if operations.perform(SurfaceOperation::MainFocus).is_err() {
        let mode = if operations.perform(SurfaceOperation::PetHide).is_ok() {
            ShellMode::MainOpen
        } else if operations.perform(SurfaceOperation::MainHide).is_ok() {
            let _ = operations.perform(SurfaceOperation::PetShow);
            ShellMode::PetVisible
        } else {
            ShellMode::MainOpen
        };
        return TransitionOutcome {
            mode,
            failure: Some(SurfaceOperation::MainFocus),
        };
    }

    if operations.perform(SurfaceOperation::PetHide).is_err() {
        let mode = if operations.perform(SurfaceOperation::MainHide).is_ok() {
            let _ = operations.perform(SurfaceOperation::PetShow);
            ShellMode::PetVisible
        } else {
            let _ = operations.perform(SurfaceOperation::PetHide);
            ShellMode::MainOpen
        };
        return TransitionOutcome {
            mode,
            failure: Some(SurfaceOperation::PetHide),
        };
    }

    TransitionOutcome {
        mode: ShellMode::MainOpen,
        failure: None,
    }
}

fn run_hide_pet(
    operations: &mut impl SurfaceOperations,
    main_visible: bool,
    main_minimized: bool,
) -> TransitionOutcome {
    let initial_mode = if main_visible {
        if main_minimized {
            ShellMode::MainMinimizedPetVisible
        } else {
            ShellMode::MainOpen
        }
    } else {
        ShellMode::PetVisible
    };
    if operations.perform(SurfaceOperation::PetHide).is_err() {
        return TransitionOutcome {
            mode: initial_mode,
            failure: Some(SurfaceOperation::PetHide),
        };
    }
    if !main_visible {
        return TransitionOutcome {
            mode: ShellMode::TrayOnly,
            failure: None,
        };
    }
    if !main_minimized {
        return TransitionOutcome {
            mode: ShellMode::MainOpen,
            failure: None,
        };
    }
    if operations.perform(SurfaceOperation::MainHide).is_ok() {
        return TransitionOutcome {
            mode: ShellMode::TrayOnly,
            failure: None,
        };
    }
    let mode = if operations.perform(SurfaceOperation::PetShow).is_ok() {
        ShellMode::MainMinimizedPetVisible
    } else {
        ShellMode::MainOpen
    };
    TransitionOutcome {
        mode,
        failure: Some(SurfaceOperation::MainHide),
    }
}

fn run_return_to_pet(operations: &mut impl SurfaceOperations) -> TransitionOutcome {
    if operations.perform(SurfaceOperation::PetShow).is_err() {
        return TransitionOutcome {
            mode: ShellMode::MainOpen,
            failure: Some(SurfaceOperation::PetShow),
        };
    }
    if operations.perform(SurfaceOperation::MainHide).is_err() {
        let mode = if operations.perform(SurfaceOperation::PetHide).is_ok() {
            let _ = operations.perform(SurfaceOperation::MainFocus);
            ShellMode::MainOpen
        } else if operations.perform(SurfaceOperation::MainHide).is_ok() {
            ShellMode::PetVisible
        } else {
            ShellMode::MainOpen
        };
        return TransitionOutcome {
            mode,
            failure: Some(SurfaceOperation::MainHide),
        };
    }
    TransitionOutcome {
        mode: ShellMode::PetVisible,
        failure: None,
    }
}

pub(crate) fn open_detail(app: &tauri::AppHandle) -> Result<(), String> {
    let main = app
        .get_webview_window("main")
        .ok_or_else(|| "Memoryling could not find its detail window.".to_string())?;
    let pet = app
        .get_webview_window("pet")
        .ok_or_else(|| "Memoryling could not find its pet window.".to_string())?;
    set_mode(app, ShellMode::OpeningMain);
    let mut operations = TauriSurfaceOperations { app, main, pet };
    let outcome = run_open_detail(&mut operations);
    set_mode(app, outcome.mode);
    outcome
        .failure
        .map_or(Ok(()), |operation| Err(operation_error(operation)))
}

pub(crate) fn open_detail_and_finish_onboarding(app: &tauri::AppHandle) -> Result<(), String> {
    open_detail(app)?;
    let _ = mark_onboarding_dismissed(app, true);
    Ok(())
}

pub(crate) fn return_to_pet(app: &tauri::AppHandle) -> Result<(), String> {
    let main = app
        .get_webview_window("main")
        .ok_or_else(|| "Memoryling could not find its detail window.".to_string())?;
    let pet = app
        .get_webview_window("pet")
        .ok_or_else(|| "Memoryling could not find its pet window.".to_string())?;
    set_mode(app, ShellMode::ReturningToPet);
    let mut operations = TauriSurfaceOperations { app, main, pet };
    let outcome = run_return_to_pet(&mut operations);
    set_mode(app, outcome.mode);
    outcome
        .failure
        .map_or(Ok(()), |operation| Err(operation_error(operation)))
}

pub(crate) fn hide_pet(app: &tauri::AppHandle) -> Result<(), String> {
    let main = app
        .get_webview_window("main")
        .ok_or_else(|| "Memoryling could not find its detail window.".to_string())?;
    let pet = app
        .get_webview_window("pet")
        .ok_or_else(|| "Memoryling could not find its pet window.".to_string())?;
    let main_visible = main
        .is_visible()
        .map_err(|_| "Memoryling could not inspect its detail window.".to_string())?;
    let main_minimized = main
        .is_minimized()
        .map_err(|_| "Memoryling could not inspect its detail window.".to_string())?;
    let mut operations = TauriSurfaceOperations { app, main, pet };
    let outcome = run_hide_pet(&mut operations, main_visible, main_minimized);
    set_mode(app, outcome.mode);
    outcome
        .failure
        .map_or(Ok(()), |operation| Err(operation_error(operation)))
}

pub(crate) fn handle_window_event(window: &Window, event: &WindowEvent) {
    match window.label() {
        "main" => handle_main_window_event(window, event),
        "pet" => match event {
            WindowEvent::Moved(_) | WindowEvent::ScaleFactorChanged { .. } => {
                super::position::schedule_pet_position_settle(window.app_handle());
            }
            _ => {}
        },
        _ => {}
    }
}

fn handle_main_window_event(window: &Window, event: &WindowEvent) {
    let app = window.app_handle();
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            let cleared = app
                .try_state::<PendingImports>()
                .map(|pending| pending.clear_all())
                .unwrap_or(Ok(()));
            if cleared.is_err() {
                let _ = window.show();
                let _ = window.set_focus();
                return;
            }
            emit_detail_reset(app);
            let _ = return_to_pet(app);
        }
        WindowEvent::Resized(_) | WindowEvent::Focused(true) => {
            synchronize_minimize_restore(window);
        }
        _ => {}
    }
}

fn synchronize_minimize_restore(main: &Window) {
    let app = main.app_handle();
    let mode = super::shell_mode(app);
    let visible = main.is_visible().unwrap_or(false);
    let minimized = main.is_minimized().unwrap_or(false);
    match minimize_sync_action(mode, visible, minimized) {
        MinimizeSyncAction::ShowPet => {
            let _ = show_pet(app, ShellMode::MainMinimizedPetVisible);
        }
        MinimizeSyncAction::HidePet => {
            if let Some(pet) = app.get_webview_window("pet") {
                if pet.hide().is_ok() {
                    set_mode(app, ShellMode::MainOpen);
                }
            }
        }
        MinimizeSyncAction::None => {}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MinimizeSyncAction {
    None,
    ShowPet,
    HidePet,
}

fn minimize_sync_action(
    mode: ShellMode,
    main_visible: bool,
    main_minimized: bool,
) -> MinimizeSyncAction {
    if !main_visible
        || matches!(
            mode,
            ShellMode::Starting
                | ShellMode::OpeningMain
                | ShellMode::ReturningToPet
                | ShellMode::TrayOnly
                | ShellMode::Quitting
        )
    {
        return MinimizeSyncAction::None;
    }
    if main_minimized {
        MinimizeSyncAction::ShowPet
    } else if mode == ShellMode::MainMinimizedPetVisible {
        MinimizeSyncAction::HidePet
    } else {
        MinimizeSyncAction::None
    }
}

fn show_pet(app: &tauri::AppHandle, resulting_mode: ShellMode) -> Result<(), String> {
    show_pet_surface(app)?;
    set_mode(app, resulting_mode);
    Ok(())
}

fn show_pet_surface(app: &tauri::AppHandle) -> Result<(), String> {
    let pet = app
        .get_webview_window("pet")
        .ok_or_else(|| "Memoryling could not find its pet window.".to_string())?;
    let settings = shell_settings_snapshot(app);
    apply_pet_size(app, settings.onboarding_dismissed)?;
    restore_pet_position(app)?;
    pet.set_always_on_top(settings.always_on_top)
        .map_err(|_| "Memoryling could not apply its pet window preference.".to_string())?;
    pet.show()
        .map_err(|_| "Memoryling could not show its pet window.".to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct FakeSurfaces {
        main_visible: bool,
        main_minimized: bool,
        pet_visible: bool,
        fail_once: Option<SurfaceOperation>,
        operations: Vec<SurfaceOperation>,
    }

    impl FakeSurfaces {
        fn pet_first(fail_once: Option<SurfaceOperation>) -> Self {
            Self {
                main_visible: false,
                main_minimized: false,
                pet_visible: true,
                fail_once,
                operations: Vec::new(),
            }
        }

        fn main_first(fail_once: Option<SurfaceOperation>) -> Self {
            Self {
                main_visible: true,
                main_minimized: false,
                pet_visible: false,
                fail_once,
                operations: Vec::new(),
            }
        }

        fn minimized_with_pet(fail_once: Option<SurfaceOperation>) -> Self {
            Self {
                main_visible: true,
                main_minimized: true,
                pet_visible: true,
                fail_once,
                operations: Vec::new(),
            }
        }

        fn assert_one_recoverable_surface(&self) {
            let interactive_main = self.main_visible && !self.main_minimized;
            assert_ne!(interactive_main, self.pet_visible);
        }
    }

    impl SurfaceOperations for FakeSurfaces {
        fn perform(&mut self, operation: SurfaceOperation) -> Result<(), ()> {
            self.operations.push(operation);
            if self.fail_once == Some(operation) {
                self.fail_once = None;
                return Err(());
            }
            match operation {
                SurfaceOperation::MainUnminimize => {
                    if self.main_minimized {
                        self.main_minimized = false;
                        self.main_visible = true;
                    }
                }
                SurfaceOperation::MainShow => self.main_visible = true,
                SurfaceOperation::MainHide => self.main_visible = false,
                SurfaceOperation::PetShow => self.pet_visible = true,
                SurfaceOperation::PetHide => self.pet_visible = false,
                SurfaceOperation::MainFocus => {}
            }
            Ok(())
        }
    }

    #[test]
    fn lifecycle_modes_keep_open_and_return_transitions_explicit() {
        assert_ne!(ShellMode::OpeningMain, ShellMode::MainOpen);
        assert_ne!(ShellMode::ReturningToPet, ShellMode::PetVisible);
        assert_ne!(ShellMode::TrayOnly, ShellMode::Quitting);
    }

    #[test]
    fn open_detail_pet_hide_failure_rolls_main_back_before_restoring_pet() {
        let mut surfaces = FakeSurfaces::pet_first(Some(SurfaceOperation::PetHide));
        let outcome = run_open_detail(&mut surfaces);
        assert_eq!(outcome.mode, ShellMode::PetVisible);
        assert_eq!(outcome.failure, Some(SurfaceOperation::PetHide));
        assert_eq!(
            surfaces.operations,
            [
                SurfaceOperation::MainUnminimize,
                SurfaceOperation::MainShow,
                SurfaceOperation::MainFocus,
                SurfaceOperation::PetHide,
                SurfaceOperation::MainHide,
                SurfaceOperation::PetShow,
            ]
        );
        surfaces.assert_one_recoverable_surface();
        assert!(!surfaces.main_visible);
        assert!(surfaces.pet_visible);
    }

    #[test]
    fn return_to_pet_main_hide_failure_hides_pet_and_refocuses_main() {
        let mut surfaces = FakeSurfaces::main_first(Some(SurfaceOperation::MainHide));
        let outcome = run_return_to_pet(&mut surfaces);
        assert_eq!(outcome.mode, ShellMode::MainOpen);
        assert_eq!(outcome.failure, Some(SurfaceOperation::MainHide));
        assert_eq!(
            surfaces.operations,
            [
                SurfaceOperation::PetShow,
                SurfaceOperation::MainHide,
                SurfaceOperation::PetHide,
                SurfaceOperation::MainFocus,
            ]
        );
        surfaces.assert_one_recoverable_surface();
        assert!(surfaces.main_visible);
        assert!(!surfaces.pet_visible);
    }

    #[test]
    fn every_single_transition_failure_keeps_one_recoverable_surface() {
        for failure in [
            SurfaceOperation::MainUnminimize,
            SurfaceOperation::MainShow,
            SurfaceOperation::MainFocus,
            SurfaceOperation::PetHide,
        ] {
            let mut surfaces = FakeSurfaces::pet_first(Some(failure));
            let outcome = run_open_detail(&mut surfaces);
            assert_eq!(outcome.failure, Some(failure));
            surfaces.assert_one_recoverable_surface();
            assert_eq!(
                outcome.mode,
                if surfaces.main_visible {
                    ShellMode::MainOpen
                } else {
                    ShellMode::PetVisible
                }
            );
        }

        for failure in [SurfaceOperation::PetShow, SurfaceOperation::MainHide] {
            let mut surfaces = FakeSurfaces::main_first(Some(failure));
            let outcome = run_return_to_pet(&mut surfaces);
            assert_eq!(outcome.failure, Some(failure));
            surfaces.assert_one_recoverable_surface();
            assert_eq!(
                outcome.mode,
                if surfaces.main_visible {
                    ShellMode::MainOpen
                } else {
                    ShellMode::PetVisible
                }
            );
        }
    }

    #[test]
    fn tray_only_ignores_stale_minimize_and_focus_events() {
        assert_eq!(
            minimize_sync_action(ShellMode::TrayOnly, true, true),
            MinimizeSyncAction::None
        );
        assert_eq!(
            minimize_sync_action(ShellMode::MainOpen, true, true),
            MinimizeSyncAction::ShowPet
        );
        assert_eq!(
            minimize_sync_action(ShellMode::MainMinimizedPetVisible, true, false),
            MinimizeSyncAction::HidePet
        );
    }

    #[test]
    fn open_from_minimized_pet_rolls_back_if_show_fails_after_unminimize() {
        let mut surfaces = FakeSurfaces::minimized_with_pet(Some(SurfaceOperation::MainShow));
        let outcome = run_open_detail(&mut surfaces);
        assert_eq!(outcome.mode, ShellMode::PetVisible);
        assert_eq!(outcome.failure, Some(SurfaceOperation::MainShow));
        assert_eq!(
            surfaces.operations,
            [
                SurfaceOperation::MainUnminimize,
                SurfaceOperation::MainShow,
                SurfaceOperation::MainHide,
                SurfaceOperation::PetShow,
            ]
        );
        surfaces.assert_one_recoverable_surface();
        assert!(!surfaces.main_visible);
        assert!(surfaces.pet_visible);
    }

    #[test]
    fn hiding_pet_from_minimized_mode_removes_taskbar_restore_path() {
        let mut surfaces = FakeSurfaces::minimized_with_pet(None);
        let hidden = run_hide_pet(&mut surfaces, true, true);
        assert_eq!(hidden.mode, ShellMode::TrayOnly);
        assert_eq!(hidden.failure, None);
        assert_eq!(
            surfaces.operations,
            [SurfaceOperation::PetHide, SurfaceOperation::MainHide]
        );
        assert!(!surfaces.main_visible);
        assert!(!surfaces.pet_visible);
        assert_eq!(
            minimize_sync_action(ShellMode::TrayOnly, false, true),
            MinimizeSyncAction::None
        );

        surfaces.operations.clear();
        let reopened = run_open_detail(&mut surfaces);
        assert_eq!(reopened.mode, ShellMode::MainOpen);
        assert!(surfaces.main_visible);
        assert!(!surfaces.main_minimized);
        assert!(!surfaces.pet_visible);
        assert_eq!(
            minimize_sync_action(ShellMode::MainOpen, true, true),
            MinimizeSyncAction::ShowPet
        );
    }
}
