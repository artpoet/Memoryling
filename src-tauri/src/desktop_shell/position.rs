use std::{sync::atomic::Ordering, thread, time::Duration};

use tauri::{
    AppHandle, LogicalSize, Manager, Monitor, PhysicalPosition, PhysicalSize, Position, Size,
};

use super::{
    model::{
        SavedPetPosition, PET_DISMISSED_HEIGHT, PET_DISMISSED_WIDTH, PET_ONBOARDING_HEIGHT,
        PET_ONBOARDING_WIDTH,
    },
    settings, update_settings, DesktopShellState,
};

const INITIAL_MARGIN_DIP: f64 = 24.0;
const MOVE_SETTLE_DELAY: Duration = Duration::from_millis(300);
const TOPOLOGY_POLL_INTERVAL: Duration = Duration::from_secs(2);

#[derive(Debug, Clone, PartialEq)]
struct MonitorSignature {
    name: Option<String>,
    position: (i32, i32),
    size: (u32, u32),
    work_position: (i32, i32),
    work_size: (u32, u32),
    scale_factor: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct WorkArea {
    name: Option<String>,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    scale_factor: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AxisAnchor {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ResizeAnchor {
    horizontal: AxisAnchor,
    vertical: AxisAnchor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OnboardingResizeStage {
    Resize,
    Reposition,
    Save,
}

trait OnboardingResizeOperations {
    fn resize(&mut self) -> Result<(), ()>;
    fn reposition(&mut self) -> Result<(), ()>;
    fn save(&mut self) -> Result<(), ()>;
    fn commit(&mut self);
    fn rollback_geometry(&mut self);
}

fn run_onboarding_resize(
    operations: &mut impl OnboardingResizeOperations,
) -> Result<(), OnboardingResizeStage> {
    if operations.resize().is_err() {
        operations.rollback_geometry();
        return Err(OnboardingResizeStage::Resize);
    }
    if operations.reposition().is_err() {
        operations.rollback_geometry();
        return Err(OnboardingResizeStage::Reposition);
    }
    if operations.save().is_err() {
        operations.rollback_geometry();
        return Err(OnboardingResizeStage::Save);
    }
    operations.commit();
    Ok(())
}

pub(crate) fn apply_pet_size(app: &AppHandle, onboarding_dismissed: bool) -> Result<(), String> {
    let pet = app
        .get_webview_window("pet")
        .ok_or_else(|| "Memoryling could not find its pet window.".to_string())?;
    let (width, height) = if onboarding_dismissed {
        (PET_DISMISSED_WIDTH, PET_DISMISSED_HEIGHT)
    } else {
        (PET_ONBOARDING_WIDTH, PET_ONBOARDING_HEIGHT)
    };
    pet.set_size(Size::Logical(LogicalSize::new(width, height)))
        .map_err(|_| "Memoryling could not resize its pet window.".to_string())
}

pub(crate) fn resize_pet_and_commit_onboarding(
    app: &AppHandle,
    onboarding_dismissed: bool,
) -> Result<super::model::PetShellState, String> {
    let pet = app
        .get_webview_window("pet")
        .ok_or_else(|| "Memoryling could not find its pet window.".to_string())?;
    let old_size = pet
        .outer_size()
        .map_err(|_| "Memoryling could not inspect its pet window size.".to_string())?;
    let old_position = pet
        .outer_position()
        .map_err(|_| "Memoryling could not inspect its pet window position.".to_string())?;
    let monitor = pet
        .current_monitor()
        .ok()
        .flatten()
        .or_else(|| pet.primary_monitor().ok().flatten())
        .or_else(|| {
            pet.available_monitors()
                .ok()
                .and_then(|monitors| monitors.into_iter().next())
        })
        .ok_or_else(|| "Memoryling could not find a usable monitor.".to_string())?;
    let area = work_area(&monitor);
    let anchor = resize_anchor(&area, old_size, old_position);
    let state = app
        .try_state::<DesktopShellState>()
        .ok_or_else(|| "Memoryling's desktop settings are not ready.".to_string())?;
    let settings_guard = state
        .settings
        .lock()
        .map_err(|_| "Memoryling could not access its desktop settings.".to_string())?;
    let mut candidate = settings_guard.clone();
    candidate.onboarding_dismissed = onboarding_dismissed;

    struct NativeResize<'a> {
        app: &'a AppHandle,
        pet: tauri::WebviewWindow,
        monitor: Monitor,
        area: WorkArea,
        old_size: PhysicalSize<u32>,
        old_position: PhysicalPosition<i32>,
        anchor: ResizeAnchor,
        settings_path: std::path::PathBuf,
        settings_guard: std::sync::MutexGuard<'a, super::model::ShellSettings>,
        candidate: super::model::ShellSettings,
    }

    impl OnboardingResizeOperations for NativeResize<'_> {
        fn resize(&mut self) -> Result<(), ()> {
            apply_pet_size(self.app, self.candidate.onboarding_dismissed).map_err(|_| ())
        }

        fn reposition(&mut self) -> Result<(), ()> {
            let new_size = self.pet.outer_size().map_err(|_| ())?;
            let desired =
                anchored_resize_position(self.old_position, self.old_size, new_size, self.anchor);
            let position = clamp_position(&self.area, new_size, desired);
            self.pet
                .set_position(Position::Physical(position))
                .map_err(|_| ())?;
            self.candidate.pet_position = Some(saved_position(&self.monitor, new_size, position));
            Ok(())
        }

        fn save(&mut self) -> Result<(), ()> {
            settings::save(&self.settings_path, &self.candidate).map_err(|_| ())
        }

        fn commit(&mut self) {
            *self.settings_guard = self.candidate.clone();
        }

        fn rollback_geometry(&mut self) {
            let _ = self.pet.set_size(Size::Physical(self.old_size));
            let _ = self.pet.set_position(Position::Physical(self.old_position));
        }
    }

    let settings_path = state.settings_path.clone();
    let mut operations = NativeResize {
        app,
        pet,
        monitor,
        area,
        old_size,
        old_position,
        anchor,
        settings_path,
        settings_guard,
        candidate,
    };
    run_onboarding_resize(&mut operations).map_err(|stage| match stage {
        OnboardingResizeStage::Resize => "Memoryling could not resize its pet window.".to_string(),
        OnboardingResizeStage::Reposition => {
            "Memoryling could not preserve its pet window position.".to_string()
        }
        OnboardingResizeStage::Save => {
            "Memoryling could not save its desktop settings.".to_string()
        }
    })?;
    Ok(operations.settings_guard.public_state())
}

pub(crate) fn restore_pet_position(app: &AppHandle) -> Result<(), String> {
    let pet = app
        .get_webview_window("pet")
        .ok_or_else(|| "Memoryling could not find its pet window.".to_string())?;
    let monitors = pet
        .available_monitors()
        .map_err(|_| "Memoryling could not inspect available monitors.".to_string())?;
    let saved = app
        .try_state::<DesktopShellState>()
        .and_then(|state| state.settings.lock().ok()?.pet_position.clone());

    let primary = pet.primary_monitor().ok().flatten();
    let primary_name = primary.as_ref().and_then(|monitor| monitor.name().cloned());
    let monitor_index = select_monitor_index(
        saved
            .as_ref()
            .and_then(|position| position.monitor_name.as_deref()),
        &monitors.iter().map(work_area).collect::<Vec<_>>(),
        primary_name.as_deref(),
    );
    let monitor = monitor_index
        .and_then(|index| monitors.get(index).cloned())
        .or(primary)
        .ok_or_else(|| "Memoryling could not find a usable monitor.".to_string())?;
    let window_size = pet
        .outer_size()
        .map_err(|_| "Memoryling could not inspect its pet window size.".to_string())?;

    let area = work_area(&monitor);
    let desired = if let Some(position) = saved.filter(SavedPetPosition::is_valid) {
        if monitor.name() == position.monitor_name.as_ref() {
            restored_from_normalized(&area, window_size, &position)
        } else {
            default_bottom_right(&area, window_size)
        }
    } else {
        default_bottom_right(&area, window_size)
    };
    let clamped = clamp_position(&area, window_size, desired);
    pet.set_position(Position::Physical(clamped))
        .map_err(|_| "Memoryling could not restore its pet window position.".to_string())?;
    let _ = persist_position(app, &monitor, window_size, clamped);
    Ok(())
}

pub(crate) fn clamp_and_persist_pet(app: &AppHandle) -> Result<(), String> {
    let pet = app
        .get_webview_window("pet")
        .ok_or_else(|| "Memoryling could not find its pet window.".to_string())?;
    let window_size = pet
        .outer_size()
        .map_err(|_| "Memoryling could not inspect its pet window size.".to_string())?;
    let current = pet
        .outer_position()
        .map_err(|_| "Memoryling could not inspect its pet window position.".to_string())?;
    let monitor = pet
        .current_monitor()
        .ok()
        .flatten()
        .or_else(|| pet.primary_monitor().ok().flatten())
        .or_else(|| {
            pet.available_monitors()
                .ok()
                .and_then(|monitors| monitors.into_iter().next())
        })
        .ok_or_else(|| "Memoryling could not find a usable monitor.".to_string())?;
    let clamped = clamp_position(&work_area(&monitor), window_size, current);
    if clamped != current {
        pet.set_position(Position::Physical(clamped))
            .map_err(|_| "Memoryling could not recover its pet window position.".to_string())?;
    }
    persist_position(app, &monitor, window_size, clamped)
}

pub(crate) fn schedule_pet_position_settle(app: &AppHandle) {
    let Some(state) = app.try_state::<DesktopShellState>() else {
        return;
    };
    let first_revision = state.move_revision.fetch_add(1, Ordering::Relaxed) + 1;
    if state.move_save_scheduled.swap(true, Ordering::AcqRel) {
        return;
    }

    let app = app.clone();
    thread::spawn(move || {
        let mut observed_revision = first_revision;
        loop {
            thread::sleep(MOVE_SETTLE_DELAY);
            let Some(state) = app.try_state::<DesktopShellState>() else {
                return;
            };
            let current_revision = state.move_revision.load(Ordering::Acquire);
            if current_revision == observed_revision {
                break;
            }
            observed_revision = current_revision;
        }

        let _ = clamp_and_persist_pet(&app);
        let Some(state) = app.try_state::<DesktopShellState>() else {
            return;
        };
        state.move_save_scheduled.store(false, Ordering::Release);
        if state.move_revision.load(Ordering::Acquire) != observed_revision {
            schedule_pet_position_settle(&app);
        }
    });
}

pub(crate) fn spawn_monitor_poll(app: &AppHandle) {
    let app = app.clone();
    thread::spawn(move || {
        let mut previous = monitor_signatures(&app).unwrap_or_default();
        loop {
            thread::sleep(TOPOLOGY_POLL_INTERVAL);
            let Some(state) = app.try_state::<DesktopShellState>() else {
                return;
            };
            if state.topology_stop.load(Ordering::Acquire) {
                return;
            }
            let Ok(current) = monitor_signatures(&app) else {
                continue;
            };
            if current == previous {
                continue;
            }
            previous = current;
            let pet_visible = app
                .get_webview_window("pet")
                .and_then(|pet| pet.is_visible().ok())
                .unwrap_or(false);
            if pet_visible {
                let _ = clamp_and_persist_pet(&app);
            }
        }
    });
}

fn monitor_signatures(app: &AppHandle) -> Result<Vec<MonitorSignature>, String> {
    app.available_monitors()
        .map_err(|_| "Memoryling could not inspect available monitors.".to_string())
        .map(|monitors| monitors.iter().map(monitor_signature).collect())
}

fn monitor_signature(monitor: &Monitor) -> MonitorSignature {
    let position = monitor.position();
    let size = monitor.size();
    let work = monitor.work_area();
    MonitorSignature {
        name: monitor.name().cloned(),
        position: (position.x, position.y),
        size: (size.width, size.height),
        work_position: (work.position.x, work.position.y),
        work_size: (work.size.width, work.size.height),
        scale_factor: monitor.scale_factor(),
    }
}

fn work_area(monitor: &Monitor) -> WorkArea {
    let work = monitor.work_area();
    WorkArea {
        name: monitor.name().cloned(),
        x: work.position.x,
        y: work.position.y,
        width: work.size.width,
        height: work.size.height,
        scale_factor: monitor.scale_factor(),
    }
}

fn select_monitor_index(
    saved_name: Option<&str>,
    monitors: &[WorkArea],
    primary_name: Option<&str>,
) -> Option<usize> {
    saved_name
        .and_then(|name| {
            monitors
                .iter()
                .position(|monitor| monitor.name.as_deref() == Some(name))
        })
        .or_else(|| {
            primary_name.and_then(|name| {
                monitors
                    .iter()
                    .position(|monitor| monitor.name.as_deref() == Some(name))
            })
        })
        .or_else(|| (!monitors.is_empty()).then_some(0))
}

fn default_bottom_right(area: &WorkArea, window_size: PhysicalSize<u32>) -> PhysicalPosition<i32> {
    let margin = (INITIAL_MARGIN_DIP * area.scale_factor).round() as i64;
    let x = i64::from(area.x) + i64::from(area.width) - i64::from(window_size.width) - margin;
    let y = i64::from(area.y) + i64::from(area.height) - i64::from(window_size.height) - margin;
    PhysicalPosition::new(saturating_i32(x), saturating_i32(y))
}

fn restored_from_normalized(
    area: &WorkArea,
    window_size: PhysicalSize<u32>,
    saved: &SavedPetPosition,
) -> PhysicalPosition<i32> {
    let travel_x = i64::from(area.width.saturating_sub(window_size.width));
    let travel_y = i64::from(area.height.saturating_sub(window_size.height));
    let x = i64::from(area.x) + (saved.normalized_x * travel_x as f64).round() as i64;
    let y = i64::from(area.y) + (saved.normalized_y * travel_y as f64).round() as i64;
    PhysicalPosition::new(saturating_i32(x), saturating_i32(y))
}

fn clamp_position(
    area: &WorkArea,
    window_size: PhysicalSize<u32>,
    desired: PhysicalPosition<i32>,
) -> PhysicalPosition<i32> {
    let min_x = i64::from(area.x);
    let min_y = i64::from(area.y);
    let max_x = (min_x + i64::from(area.width) - i64::from(window_size.width)).max(min_x);
    let max_y = (min_y + i64::from(area.height) - i64::from(window_size.height)).max(min_y);
    PhysicalPosition::new(
        saturating_i32(i64::from(desired.x).clamp(min_x, max_x)),
        saturating_i32(i64::from(desired.y).clamp(min_y, max_y)),
    )
}

fn resize_anchor(
    area: &WorkArea,
    size: PhysicalSize<u32>,
    position: PhysicalPosition<i32>,
) -> ResizeAnchor {
    let travel_x = area.width.saturating_sub(size.width);
    let travel_y = area.height.saturating_sub(size.height);
    let offset_x = i64::from(position.x) - i64::from(area.x);
    let offset_y = i64::from(position.y) - i64::from(area.y);
    ResizeAnchor {
        horizontal: axis_anchor(offset_x, travel_x),
        vertical: axis_anchor(offset_y, travel_y),
    }
}

fn axis_anchor(offset: i64, travel: u32) -> AxisAnchor {
    if travel == 0 {
        return AxisAnchor::Start;
    }
    let normalized = (offset as f64 / f64::from(travel)).clamp(0.0, 1.0);
    if normalized <= 0.25 {
        AxisAnchor::Start
    } else if normalized >= 0.75 {
        AxisAnchor::End
    } else {
        AxisAnchor::Center
    }
}

fn anchored_resize_position(
    old_position: PhysicalPosition<i32>,
    old_size: PhysicalSize<u32>,
    new_size: PhysicalSize<u32>,
    anchor: ResizeAnchor,
) -> PhysicalPosition<i32> {
    fn anchored_axis(position: i32, old_extent: u32, new_extent: u32, anchor: AxisAnchor) -> i32 {
        let delta = i64::from(old_extent) - i64::from(new_extent);
        let shift = match anchor {
            AxisAnchor::Start => 0,
            AxisAnchor::Center => delta / 2,
            AxisAnchor::End => delta,
        };
        saturating_i32(i64::from(position) + shift)
    }

    PhysicalPosition::new(
        anchored_axis(
            old_position.x,
            old_size.width,
            new_size.width,
            anchor.horizontal,
        ),
        anchored_axis(
            old_position.y,
            old_size.height,
            new_size.height,
            anchor.vertical,
        ),
    )
}

fn persist_position(
    app: &AppHandle,
    monitor: &Monitor,
    window_size: PhysicalSize<u32>,
    position: PhysicalPosition<i32>,
) -> Result<(), String> {
    let saved = saved_position(monitor, window_size, position);
    update_settings(app, |settings| settings.pet_position = Some(saved)).map(|_| ())
}

fn saved_position(
    monitor: &Monitor,
    window_size: PhysicalSize<u32>,
    position: PhysicalPosition<i32>,
) -> SavedPetPosition {
    let work = monitor.work_area();
    let travel_x = work.size.width.saturating_sub(window_size.width);
    let travel_y = work.size.height.saturating_sub(window_size.height);
    let offset_x = i64::from(position.x) - i64::from(work.position.x);
    let offset_y = i64::from(position.y) - i64::from(work.position.y);
    let normalized_x = if travel_x == 0 {
        0.0
    } else {
        (offset_x as f64 / f64::from(travel_x)).clamp(0.0, 1.0)
    };
    let normalized_y = if travel_y == 0 {
        0.0
    } else {
        (offset_y as f64 / f64::from(travel_y)).clamp(0.0, 1.0)
    };
    let logical = position.to_logical::<f64>(monitor.scale_factor());
    SavedPetPosition {
        monitor_name: monitor.name().cloned(),
        monitor_work_width_px: work.size.width,
        monitor_work_height_px: work.size.height,
        logical_x: logical.x,
        logical_y: logical.y,
        normalized_x,
        normalized_y,
        scale_factor: monitor.scale_factor(),
    }
}

fn saturating_i32(value: i64) -> i32 {
    value.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct FakeOnboardingResize {
        dismissed: bool,
        geometry_changed: bool,
        fail_at: Option<OnboardingResizeStage>,
        steps: Vec<&'static str>,
    }

    impl OnboardingResizeOperations for FakeOnboardingResize {
        fn resize(&mut self) -> Result<(), ()> {
            self.steps.push("resize");
            self.geometry_changed = true;
            (self.fail_at != Some(OnboardingResizeStage::Resize))
                .then_some(())
                .ok_or(())
        }

        fn reposition(&mut self) -> Result<(), ()> {
            self.steps.push("reposition");
            (self.fail_at != Some(OnboardingResizeStage::Reposition))
                .then_some(())
                .ok_or(())
        }

        fn save(&mut self) -> Result<(), ()> {
            self.steps.push("save");
            (self.fail_at != Some(OnboardingResizeStage::Save))
                .then_some(())
                .ok_or(())
        }

        fn commit(&mut self) {
            self.steps.push("commit");
            self.dismissed = true;
        }

        fn rollback_geometry(&mut self) {
            self.steps.push("rollback-geometry");
            self.geometry_changed = false;
        }
    }

    fn area(name: &str, x: i32, y: i32, width: u32, height: u32, scale_factor: f64) -> WorkArea {
        WorkArea {
            name: Some(name.to_string()),
            x,
            y,
            width,
            height,
            scale_factor,
        }
    }

    #[test]
    fn clamp_handles_negative_origins_and_work_area_offsets() {
        let monitor = area("left", -1920, 40, 1920, 1000, 1.0);
        let size = PhysicalSize::new(320, 320);
        assert_eq!(
            clamp_position(&monitor, size, PhysicalPosition::new(-3000, -20)),
            PhysicalPosition::new(-1920, 40)
        );
        assert_eq!(
            clamp_position(&monitor, size, PhysicalPosition::new(100, 2000)),
            PhysicalPosition::new(-320, 720)
        );
    }

    #[test]
    fn oversized_window_is_anchored_to_work_area_origin() {
        let monitor = area("small", 100, 80, 500, 400, 1.0);
        assert_eq!(
            clamp_position(
                &monitor,
                PhysicalSize::new(700, 600),
                PhysicalPosition::new(900, 900)
            ),
            PhysicalPosition::new(100, 80)
        );
    }

    #[test]
    fn normalized_restore_scales_logical_pet_at_100_to_200_percent() {
        let saved = SavedPetPosition {
            monitor_name: Some("test-monitor".to_string()),
            monitor_work_width_px: 1920,
            monitor_work_height_px: 1040,
            logical_x: 100.0,
            logical_y: 100.0,
            normalized_x: 0.5,
            normalized_y: 1.0,
            scale_factor: 1.0,
        };
        for scale in [1.0, 1.25, 1.5, 2.0] {
            let monitor = area("test-monitor", 0, 40, 2560, 1360, scale);
            let physical = (320.0 * scale).round() as u32;
            let size = PhysicalSize::new(physical, physical);
            let position = restored_from_normalized(&monitor, size, &saved);
            assert_eq!(position.x, ((2560 - physical) / 2) as i32);
            assert_eq!(position.y, 40 + (1360 - physical) as i32);
            assert_eq!(clamp_position(&monitor, size, position), position);
        }
    }

    #[test]
    fn removed_monitor_falls_back_to_primary_then_first() {
        let monitors = vec![
            area("primary", 0, 0, 1920, 1040, 1.0),
            area("secondary", 1920, 0, 2560, 1400, 1.5),
        ];
        assert_eq!(
            select_monitor_index(Some("removed"), &monitors, Some("primary")),
            Some(0)
        );
        assert_eq!(
            select_monitor_index(Some("removed"), &monitors, Some("missing")),
            Some(0)
        );
        assert_eq!(select_monitor_index(None, &[], None), None);
    }

    #[test]
    fn onboarding_resize_preserves_bottom_right_or_center_anchor() {
        let monitor = area("primary", 0, 40, 1920, 1040, 1.0);
        let onboarding = PhysicalSize::new(360, 430);
        let compact = PhysicalSize::new(320, 320);

        let bottom_right = PhysicalPosition::new(1536, 626);
        let bottom_right_anchor = resize_anchor(&monitor, onboarding, bottom_right);
        assert_eq!(
            bottom_right_anchor,
            ResizeAnchor {
                horizontal: AxisAnchor::End,
                vertical: AxisAnchor::End,
            }
        );
        assert_eq!(
            anchored_resize_position(bottom_right, onboarding, compact, bottom_right_anchor),
            PhysicalPosition::new(1576, 736)
        );

        let centered = PhysicalPosition::new(780, 345);
        let center_anchor = resize_anchor(&monitor, onboarding, centered);
        assert_eq!(
            center_anchor,
            ResizeAnchor {
                horizontal: AxisAnchor::Center,
                vertical: AxisAnchor::Center,
            }
        );
        assert_eq!(
            anchored_resize_position(centered, onboarding, compact, center_anchor),
            PhysicalPosition::new(800, 400)
        );
    }

    #[test]
    fn onboarding_setting_commits_only_after_geometry_and_save_succeed() {
        for (failure, expected_steps) in [
            (
                OnboardingResizeStage::Resize,
                vec!["resize", "rollback-geometry"],
            ),
            (
                OnboardingResizeStage::Reposition,
                vec!["resize", "reposition", "rollback-geometry"],
            ),
            (
                OnboardingResizeStage::Save,
                vec!["resize", "reposition", "save", "rollback-geometry"],
            ),
        ] {
            let mut operations = FakeOnboardingResize {
                dismissed: false,
                geometry_changed: false,
                fail_at: Some(failure),
                steps: Vec::new(),
            };
            assert_eq!(run_onboarding_resize(&mut operations), Err(failure));
            assert_eq!(operations.steps, expected_steps);
            assert!(!operations.dismissed);
            assert!(!operations.geometry_changed);
        }

        let mut success = FakeOnboardingResize {
            dismissed: false,
            geometry_changed: false,
            fail_at: None,
            steps: Vec::new(),
        };
        assert_eq!(run_onboarding_resize(&mut success), Ok(()));
        assert_eq!(success.steps, ["resize", "reposition", "save", "commit"]);
        assert!(success.dismissed);
        assert!(success.geometry_changed);
    }
}
