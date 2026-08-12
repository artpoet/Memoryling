use serde::{Deserialize, Serialize};

pub(crate) const PET_DISMISSED_WIDTH: f64 = 320.0;
pub(crate) const PET_DISMISSED_HEIGHT: f64 = 320.0;
pub(crate) const PET_ONBOARDING_WIDTH: f64 = 360.0;
pub(crate) const PET_ONBOARDING_HEIGHT: f64 = 430.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShellMode {
    Starting,
    OpeningMain,
    ReturningToPet,
    PetVisible,
    MainOpen,
    MainMinimizedPetVisible,
    TrayOnly,
    Quitting,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PetShellState {
    pub schema_version: u8,
    pub onboarding_dismissed: bool,
    pub always_on_top: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreatureStateChanged {
    pub revision: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub(crate) struct DetailReset {}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ContextMenuTrigger {
    Pointer,
    Keyboard,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SavedPetPosition {
    pub monitor_name: Option<String>,
    pub monitor_work_width_px: u32,
    pub monitor_work_height_px: u32,
    pub logical_x: f64,
    pub logical_y: f64,
    pub normalized_x: f64,
    pub normalized_y: f64,
    pub scale_factor: f64,
}

impl SavedPetPosition {
    pub(crate) fn is_valid(&self) -> bool {
        self.logical_x.is_finite()
            && self.logical_y.is_finite()
            && self.normalized_x.is_finite()
            && self.normalized_y.is_finite()
            && (0.0..=1.0).contains(&self.normalized_x)
            && (0.0..=1.0).contains(&self.normalized_y)
            && self.scale_factor.is_finite()
            && self.scale_factor > 0.0
            && self.monitor_work_width_px > 0
            && self.monitor_work_height_px > 0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ShellSettings {
    pub schema_version: u8,
    pub onboarding_dismissed: bool,
    pub always_on_top: bool,
    pub pet_position: Option<SavedPetPosition>,
}

impl Default for ShellSettings {
    fn default() -> Self {
        Self {
            schema_version: 1,
            onboarding_dismissed: false,
            always_on_top: true,
            pet_position: None,
        }
    }
}

impl ShellSettings {
    pub(crate) fn is_valid(&self) -> bool {
        self.schema_version == 1
            && self
                .pet_position
                .as_ref()
                .map(SavedPetPosition::is_valid)
                .unwrap_or(true)
    }

    pub(crate) fn public_state(&self) -> PetShellState {
        PetShellState {
            schema_version: 1,
            onboarding_dismissed: self.onboarding_dismissed,
            always_on_top: self.always_on_top,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_or_future_position_state_fails_validation() {
        let mut settings = ShellSettings {
            schema_version: 2,
            ..Default::default()
        };
        assert!(!settings.is_valid());

        settings.schema_version = 1;
        settings.pet_position = Some(SavedPetPosition {
            monitor_name: None,
            monitor_work_width_px: 1920,
            monitor_work_height_px: 1040,
            logical_x: 10.0,
            logical_y: 10.0,
            normalized_x: f64::NAN,
            normalized_y: 0.5,
            scale_factor: 1.0,
        });
        assert!(!settings.is_valid());
    }

    #[test]
    fn public_shell_state_is_content_free() {
        let json = serde_json::to_string(&ShellSettings::default().public_state())
            .expect("shell state should serialize");
        assert_eq!(
            json,
            r#"{"schemaVersion":1,"onboardingDismissed":false,"alwaysOnTop":true}"#
        );
    }
}
