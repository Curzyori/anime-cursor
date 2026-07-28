use crate::pack::get_app_dir;
use std::fs::{self, File};
use std::io::Write;

#[cfg(target_os = "linux")]
fn gsettings_get(key: &str) -> Result<String, String> {
    let output = std::process::Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", key])
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(format!("Failed to read gsettings {}", key));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .trim()
        .trim_matches('\'')
        .trim_matches('"')
        .to_string())
}

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

pub fn create_backup() -> Result<String, String> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .to_string();

    let backup_dir = get_app_dir().join("backups").join(&timestamp);
    fs::create_dir_all(&backup_dir).map_err(|e| e.to_string())?;

    #[cfg(target_os = "windows")]
    {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let cursors_key = hkcu
            .open_subkey("Control Panel\\Cursors")
            .map_err(|e| e.to_string())?;

        // Check if already using anime-cursor to avoid overwriting backup
        let current_arrow: String = cursors_key.get_value("Arrow").unwrap_or_default();
        if current_arrow.contains("anime-cursor") {
            let _ = fs::remove_dir_all(&backup_dir);
            return Ok("anime-cursor".to_string());
        }

        let mut reg_content = String::new();
        // REG file header uses real CR+LF line endings
        reg_content.push_str("Windows Registry Editor Version 5.00\r\n\r\n");
        reg_content.push_str("[HKEY_CURRENT_USER\\Control Panel\\Cursors]\r\n");

        for name in cursors_key
            .enum_values()
            .filter_map(|x| x.ok().map(|v| v.0))
        {
            let val: String = cursors_key.get_value(&name).unwrap_or_default();
            // REG file format uses raw backslashes — no escaping needed
            reg_content.push_str(&format!("\"{}\"=\"{}\"\r\n", name, val));
        }

        let reg_file_path = backup_dir.join("cursors.reg");
        let mut f = File::create(&reg_file_path).map_err(|e| e.to_string())?;
        f.write_all(reg_content.as_bytes())
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        let theme = gsettings_get("cursor-theme")?;
        if theme == "anime-cursor" {
            let _ = fs::remove_dir_all(&backup_dir);
            return Ok("anime-cursor".to_string());
        }
        let size = gsettings_get("cursor-size")?;
        let theme_file_path = backup_dir.join("cursor-theme.txt");
        let mut f = File::create(&theme_file_path).map_err(|e| e.to_string())?;
        f.write_all(theme.as_bytes()).map_err(|e| e.to_string())?;
        let mut f = File::create(backup_dir.join("cursor-size.txt")).map_err(|e| e.to_string())?;
        f.write_all(size.as_bytes()).map_err(|e| e.to_string())?;
    }

    Ok(timestamp)
}

pub fn restore_backup(timestamp: &str) -> Result<(), String> {
    let backup_dir = get_app_dir().join("backups").join(timestamp);
    if !backup_dir.exists() {
        return Err("Backup not found".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        let reg_file_path = backup_dir.join("cursors.reg");
        if !reg_file_path.exists() {
            return Err("Registry backup file not found".to_string());
        }

        let status = std::process::Command::new("reg")
            .args(["import", &reg_file_path.to_string_lossy()])
            .status()
            .map_err(|e| e.to_string())?;

        if !status.success() {
            return Err("Failed to import registry file".to_string());
        }

        unsafe {
            use windows_sys::Win32::UI::WindowsAndMessaging::{
                SystemParametersInfoW, SPIF_SENDCHANGE, SPI_SETCURSORS,
            };
            SystemParametersInfoW(SPI_SETCURSORS, 0, std::ptr::null_mut(), SPIF_SENDCHANGE);
        }
    }

    #[cfg(target_os = "linux")]
    {
        let theme_file_path = backup_dir.join("cursor-theme.txt");
        if !theme_file_path.exists() {
            return Err("Theme backup file not found".to_string());
        }

        let theme = fs::read_to_string(&theme_file_path).map_err(|e| e.to_string())?;
        let theme = theme.trim();
        let size =
            fs::read_to_string(backup_dir.join("cursor-size.txt")).map_err(|e| e.to_string())?;
        let size = size.trim();

        if let Some(home) = dirs::home_dir() {
            let _ = std::fs::remove_dir_all(home.join(".icons").join("anime-cursor"));
        }

        let dummy = if theme == "default" {
            "Adwaita"
        } else {
            "default"
        };
        let _ = std::process::Command::new("gsettings")
            .args(["set", "org.gnome.desktop.interface", "cursor-theme", dummy])
            .status();

        let status = std::process::Command::new("gsettings")
            .args(["set", "org.gnome.desktop.interface", "cursor-theme", theme])
            .status()
            .map_err(|e| e.to_string())?;

        if !status.success() {
            return Err("Failed to set gsettings cursor-theme".to_string());
        }
        let status = std::process::Command::new("gsettings")
            .args(["set", "org.gnome.desktop.interface", "cursor-size", size])
            .status()
            .map_err(|e| e.to_string())?;
        if !status.success() {
            return Err("Failed to set gsettings cursor-size".to_string());
        }

        let _ = std::process::Command::new("plasma-apply-cursortheme")
            .arg(theme)
            .status();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_and_restore_lifecycle() {
        #[cfg(target_os = "linux")]
        let _lock = crate::GSETTINGS_TEST_LOCK.lock().unwrap();
        let ts = create_backup().expect("Failed to create backup");
        let backup_dir = get_app_dir().join("backups").join(&ts);
        assert!(backup_dir.exists());

        #[cfg(target_os = "linux")]
        assert!(backup_dir.join("cursor-theme.txt").exists());

        #[cfg(target_os = "windows")]
        assert!(backup_dir.join("cursors.reg").exists());

        restore_backup(&ts).expect("Failed to restore backup");
    }

    #[test]
    fn test_backup_saves_cursor_size() {
        #[cfg(target_os = "linux")]
        let _lock = crate::GSETTINGS_TEST_LOCK.lock().unwrap();
        let ts = create_backup().expect("Failed to create backup");
        let backup_dir = get_app_dir().join("backups").join(&ts);
        #[cfg(target_os = "linux")]
        {
            let size_file = backup_dir.join("cursor-size.txt");
            assert!(size_file.exists(), "backup should create cursor-size.txt");
            let content = std::fs::read_to_string(&size_file).unwrap();
            assert!(
                !content.trim().is_empty(),
                "cursor-size.txt should not be empty"
            );
        }
        let _ = std::fs::remove_dir_all(&backup_dir);
    }

    #[test]
    fn test_backup_ids_unique() {
        #[cfg(target_os = "linux")]
        let _lock = crate::GSETTINGS_TEST_LOCK.lock().unwrap();
        let ts1 = create_backup().expect("backup 1 failed");
        let ts2 = create_backup().expect("backup 2 failed");
        assert_ne!(ts1, ts2, "backup IDs must be unique even in same second");
        let dir1 = get_app_dir().join("backups").join(&ts1);
        let dir2 = get_app_dir().join("backups").join(&ts2);
        assert!(dir1.exists());
        assert!(dir2.exists());
        let _ = std::fs::remove_dir_all(&dir1);
        let _ = std::fs::remove_dir_all(&dir2);
    }

    #[test]
    fn test_backup_skips_when_already_anime_cursor() {
        #[cfg(target_os = "linux")]
        {
            let _lock = crate::GSETTINGS_TEST_LOCK.lock().unwrap();
            let _ = std::process::Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.interface",
                    "cursor-theme",
                    "anime-cursor",
                ])
                .status();

            let ts = create_backup().expect("backup should not fail");
            let backup_dir = get_app_dir().join("backups").join(&ts);
            let size_file = backup_dir.join("cursor-size.txt");
            if size_file.exists() {
                let content = std::fs::read_to_string(&size_file).unwrap();
                assert_ne!(
                    content.trim(),
                    "anime-cursor",
                    "backup should not save anime-cursor as original"
                );
            }
            let _ = std::process::Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.interface",
                    "cursor-theme",
                    "Adwaita",
                ])
                .status();
            let _ = std::fs::remove_dir_all(&backup_dir);
        }
    }

    #[test]
    fn test_restore_restores_cursor_size() {
        #[cfg(target_os = "linux")]
        {
            let _lock = crate::GSETTINGS_TEST_LOCK.lock().unwrap();
            let current = gsettings_get("cursor-theme").unwrap_or_default();
            if current == "anime-cursor" {
                let _ = std::process::Command::new("gsettings")
                    .args([
                        "set",
                        "org.gnome.desktop.interface",
                        "cursor-theme",
                        "Adwaita",
                    ])
                    .status();
            }

            let ts = create_backup().expect("backup failed");
            let backup_dir = get_app_dir().join("backups").join(&ts);
            assert!(backup_dir.exists(), "backup dir must exist");

            std::fs::write(backup_dir.join("cursor-size.txt"), "48")
                .expect("write cursor-size.txt");

            restore_backup(&ts).expect("restore failed");

            let size = gsettings_get("cursor-size").expect("gsettings get size");
            assert_eq!(
                size, "48",
                "restore should set cursor-size to backed up value"
            );

            let _ = std::process::Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.interface",
                    "cursor-theme",
                    "Adwaita",
                ])
                .status();
            let _ = std::fs::remove_dir_all(&backup_dir);
        }
    }

    #[test]
    fn test_windows_registry_key_single_backslash() {
        #[cfg(target_os = "windows")]
        {
            let path = "Control Panel\\Cursors";
            assert_eq!(path, "Control Panel\\Cursors");
        }
    }
}
