use std::{fs, path::Path};

use super::model::ShellSettings;

const MAX_SETTINGS_BYTES: u64 = 16 * 1024;

pub(crate) fn load(path: &Path) -> Result<ShellSettings, String> {
    if let Some(settings) = read_valid(path) {
        return Ok(settings);
    }
    let backup = backup_path(path);
    if let Some(settings) = read_valid(&backup) {
        return Ok(settings);
    }
    Ok(ShellSettings::default())
}

fn read_valid(path: &Path) -> Option<ShellSettings> {
    if !path.exists() {
        return None;
    }
    let metadata = fs::metadata(path).ok()?;
    if metadata.len() > MAX_SETTINGS_BYTES {
        return None;
    }
    let bytes = fs::read(path).ok()?;
    let value = serde_json::from_slice::<serde_json::Value>(&bytes).ok()?;
    let legacy_without_setup = value.get("setupComplete").is_none();
    let mut settings = serde_json::from_value::<ShellSettings>(value).ok()?;
    if legacy_without_setup {
        // Any existing settings file proves the app already ran before the
        // first-run creation flow existed. Do not interrupt upgraded users.
        settings.setup_complete = true;
    }
    if !settings.is_valid() {
        return None;
    }
    Some(settings)
}

pub(crate) fn save(path: &Path, settings: &ShellSettings) -> Result<(), String> {
    if !settings.is_valid() {
        return Err("Memoryling refused invalid desktop settings.".to_string());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|_| "Memoryling could not create its desktop settings folder.".to_string())?;
    }
    let bytes = serde_json::to_vec(settings)
        .map_err(|_| "Memoryling could not encode its desktop settings.".to_string())?;
    let temporary = temporary_path(path);
    let backup = backup_path(path);
    let _ = fs::remove_file(&temporary);
    fs::write(&temporary, bytes)
        .map_err(|_| "Memoryling could not save its desktop settings.".to_string())?;

    if path.exists() {
        if read_valid(path).is_some() {
            let _ = fs::remove_file(&backup);
            if fs::rename(path, &backup).is_err() {
                let _ = fs::remove_file(&temporary);
                return Err("Memoryling could not rotate its desktop settings.".to_string());
            }
        } else if fs::remove_file(path).is_err() {
            let _ = fs::remove_file(&temporary);
            return Err("Memoryling could not replace invalid desktop settings.".to_string());
        }
    }
    if fs::rename(&temporary, path).is_err() {
        if backup.exists() && !path.exists() {
            let _ = fs::rename(&backup, path);
        }
        let _ = fs::remove_file(&temporary);
        return Err("Memoryling could not replace its desktop settings.".to_string());
    }
    Ok(())
}

fn temporary_path(path: &Path) -> std::path::PathBuf {
    path.with_extension("json.tmp")
}

fn backup_path(path: &Path) -> std::path::PathBuf {
    path.with_extension("json.bak")
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    #[test]
    fn settings_round_trip_contains_only_shell_state() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be valid")
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "memoryling-shell-settings-{}-{nonce}",
            std::process::id()
        ));
        let path = directory.join("desktop-shell-v1.json");
        let expected = ShellSettings {
            onboarding_dismissed: true,
            ..Default::default()
        };
        save(&path, &expected).expect("settings should save");
        assert_eq!(load(&path).expect("settings should reload"), expected);

        let encoded = fs::read_to_string(&path).expect("settings should be readable");
        for forbidden in ["memoryText", "sourceId", "locator", "contentHash", "apiKey"] {
            assert!(!encoded.contains(forbidden));
        }
        fs::remove_dir_all(directory).expect("temporary settings should be removable");
    }

    #[test]
    fn truncated_primary_recovers_from_backup_or_defaults() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be valid")
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "memoryling-shell-recovery-{}-{nonce}",
            std::process::id()
        ));
        let path = directory.join("desktop-shell-v1.json");
        let first = ShellSettings {
            onboarding_dismissed: true,
            ..Default::default()
        };
        save(&path, &first).expect("first settings should save");

        let mut second = first.clone();
        second.always_on_top = false;
        save(&path, &second).expect("second settings should rotate a backup");
        fs::write(&path, b"{truncated").expect("test should truncate primary");
        assert_eq!(load(&path).expect("backup should recover"), first);

        fs::remove_file(backup_path(&path)).expect("backup should exist");
        assert_eq!(
            load(&path).expect("invalid files should recover safely"),
            ShellSettings::default()
        );
        fs::remove_dir_all(directory).expect("temporary settings should be removable");
    }

    #[test]
    fn legacy_settings_skip_new_first_run_without_losing_preferences() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be valid")
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "memoryling-shell-legacy-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&directory).expect("test directory should exist");
        let path = directory.join("desktop-shell-v1.json");
        fs::write(
            &path,
            br#"{"schemaVersion":1,"onboardingDismissed":true,"alwaysOnTop":false,"petPosition":null}"#,
        )
        .expect("legacy settings should write");

        let settings = load(&path).expect("legacy settings should migrate in memory");
        assert!(settings.setup_complete);
        assert!(settings.onboarding_dismissed);
        assert!(!settings.always_on_top);
        fs::remove_dir_all(directory).expect("temporary settings should be removable");
    }
}
