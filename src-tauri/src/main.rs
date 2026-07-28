#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod applier;
mod backup;
mod pack;
mod parser;

#[cfg(test)]
static GSETTINGS_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

use pack::Pack;
use std::path::Path;
use tauri::Manager;

#[tauri::command]
async fn list_packs() -> Result<Vec<Pack>, String> {
    tauri::async_runtime::spawn_blocking(pack::list_installed_packs)
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn import_pack(zip_path: String) -> Result<Pack, String> {
    let path = Path::new(&zip_path);
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if ext != "zip" && ext != "ani" {
        return Err(
            "Unsupported file extension. Only .zip and .ani files are supported.".to_string(),
        );
    }

    let path_buf = path.to_path_buf();
    tauri::async_runtime::spawn_blocking(move || {
        if path_buf
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase()
            == "zip"
        {
            pack::import_pack_zip(&path_buf)
        } else {
            pack::import_single_ani(&path_buf)
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn apply_cursor(ani_path: String) -> Result<(), String> {
    let path = Path::new(&ani_path);

    let app_dir = pack::get_app_dir();
    let packs_dir = app_dir.join("packs");
    let canonical = path
        .canonicalize()
        .map_err(|_| "Invalid file path".to_string())?;
    let canonical_packs = packs_dir.canonicalize().map_err(|e| e.to_string())?;
    if !canonical.starts_with(&canonical_packs)
        || !canonical.is_file()
        || canonical.extension().and_then(|ext| ext.to_str()) != Some("ani")
    {
        return Err("Access denied: cursor file must be a managed .ani file".to_string());
    }

    let path_buf = ani_path;
    tauri::async_runtime::spawn_blocking(move || {
        backup::create_backup().and_then(|_| applier::apply_cursor_variant(&path_buf))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn restore_default_cursor() -> Result<(), String> {
    let backup_root = pack::get_app_dir().join("backups");
    tauri::async_runtime::spawn_blocking(move || restore_blocking(backup_root))
        .await
        .map_err(|e| e.to_string())?
}

fn restore_blocking(backup_root: std::path::PathBuf) -> Result<(), String> {
    let mut entries: Vec<_> = if backup_root.exists() {
        std::fs::read_dir(backup_root)
            .map_err(|e| e.to_string())?
            .flatten()
            .collect()
    } else {
        Vec::new()
    };

    if entries.is_empty() {
        if let Some(home) = dirs::home_dir() {
            let _ = std::fs::remove_dir_all(home.join(".icons").join("anime-cursor"));
        }

        #[cfg(target_os = "linux")]
        {
            let _ = std::process::Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.interface",
                    "cursor-theme",
                    "Adwaita",
                ])
                .status();
            let _ = std::process::Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.interface",
                    "cursor-theme",
                    "default",
                ])
                .status();
            let _ = std::process::Command::new("gsettings")
                .args(["set", "org.gnome.desktop.interface", "cursor-size", "24"])
                .status();
        }
        #[cfg(target_os = "windows")]
        {
            let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
            if let Ok(mut key) =
                hkcu.open_subkey_with_flags("Control Panel\\Cursors", winreg::enums::KEY_SET_VALUE)
            {
                let _ = key.set_value("Arrow", &"");
                let _ = key.set_value("Help", &"");
                let _ = key.set_value("AppStarting", &"");
                let _ = key.set_value("Wait", &"");
                let _ = key.set_value("Hand", &"");
            }
            unsafe {
                use windows_sys::Win32::UI::WindowsAndMessaging::{
                    SystemParametersInfoW, SPIF_SENDCHANGE, SPI_SETCURSORS,
                };
                SystemParametersInfoW(SPI_SETCURSORS, 0, std::ptr::null_mut(), SPIF_SENDCHANGE);
            }
        }
        return Ok(());
    }

    entries.sort_by_key(|entry| std::cmp::Reverse(entry.file_name()));

    let latest_entry = entries.first().ok_or("No backups found".to_string())?;
    let timestamp = latest_entry.file_name().to_string_lossy().to_string();

    backup::restore_backup(&timestamp)
}

#[tauri::command]
async fn remove_pack(pack_id: String) -> Result<(), String> {
    if pack_id.is_empty() || pack_id.len() > 256 {
        return Err("Invalid pack ID".to_string());
    }
    if !pack_id
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err("Invalid pack ID: only alphanumeric, dash, underscore allowed".to_string());
    }

    let app_dir = pack::get_app_dir();
    let pack_dir = app_dir.join("packs").join(&pack_id);

    if !pack_dir.exists() {
        return Ok(());
    }

    let packs_dir = app_dir.join("packs");
    let canonical = pack_dir.canonicalize().map_err(|e| e.to_string())?;
    let canonical_packs = packs_dir.canonicalize().map_err(|e| e.to_string())?;
    if !canonical.starts_with(&canonical_packs) || !canonical.is_dir() {
        return Err("Invalid pack path".to_string());
    }

    std::fs::remove_dir_all(canonical).map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    #[cfg(target_os = "linux")]
    if let Some(home) = dirs::home_dir() {
        let theme_dir = home.join(".icons").join("anime-cursor");
        let current_theme = std::process::Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "cursor-theme"])
            .output()
            .ok()
            .map(|out| {
                String::from_utf8_lossy(&out.stdout)
                    .trim()
                    .trim_matches('\'')
                    .to_string()
            });
        if current_theme.as_deref() == Some("anime-cursor")
            && !theme_dir.join("index.theme").exists()
        {
            let _ = std::process::Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.interface",
                    "cursor-theme",
                    "default",
                ])
                .status();
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            list_packs,
            import_pack,
            apply_cursor,
            restore_default_cursor,
            remove_pack
        ])
        .setup(|app| {
            if let Ok(resource_dir) = app.path().resource_dir() {
                let packs_source = resource_dir.join("default-packs");
                if let Err(e) = pack::seed_default_packs(&packs_source) {
                    eprintln!("Failed to seed default packs: {}", e);
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
