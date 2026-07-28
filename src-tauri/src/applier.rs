use crate::parser;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

/// (width, height, delay_ms, rgba, hot_x, hot_y)
type DecodedFrame = (u32, u32, u32, Vec<u8>, u32, u32);

static CURSOR_NAMES: &[(&str, bool)] = &[
    ("left_ptr", true),       // animated arrow
    ("left_ptr_watch", true), // animated busy
    ("watch", true),          // animated watch
    ("hand2", false),         // static hand
    ("pointer", false),       // static pointer — symlink target for crosshair etc.
];

/// Standard X11 cursor alias → our cursor file name.
/// Maps common GTK/X11 cursor names to our 5 cursor types via symlink.
/// ponytail: no ("pointer","pointer") self-reference — "pointer" is covered by left_ptr.
static SYMLINK_MAP: &[(&str, &str)] = &[
    ("default", "left_ptr"),
    ("xterm", "left_ptr"),
    ("ibeam", "left_ptr"),
    ("text", "left_ptr"),
    ("crosshair", "pointer"),
    ("cross", "pointer"),
    ("tcross", "pointer"),
    ("move", "hand2"),
    ("grabbing", "hand2"),
    ("grab", "hand2"),
    ("all-scroll", "hand2"),
    ("hand", "hand2"),
    ("progress", "left_ptr_watch"),
    ("wait", "watch"),
    ("pirates", "pointer"),
    ("right_ptr", "left_ptr"),
    ("top_left_arrow", "left_ptr"),
    ("w-resize", "hand2"),
    ("e-resize", "hand2"),
    ("n-resize", "hand2"),
    ("s-resize", "hand2"),
    ("ne-resize", "hand2"),
    ("nw-resize", "hand2"),
    ("se-resize", "hand2"),
    ("sw-resize", "hand2"),
];

/// Windows registry key path for cursor settings — single backslash.
#[cfg(target_os = "windows")]
const REGISTRY_CURSORS_KEY: &str = "Control Panel\\Cursors";

pub fn apply_cursor_variant(ani_path_str: &str) -> Result<(), String> {
    let ani_path = Path::new(ani_path_str);
    if !ani_path.exists() {
        return Err("Cursor file does not exist".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        // 1. Copy selected .ani to AppData active directory
        let active_dir = crate::pack::get_app_dir().join("active");
        fs::create_dir_all(&active_dir).map_err(|e| e.to_string())?;

        let target_ani_path = active_dir.join("active_cursor.ani");
        fs::copy(ani_path, &target_ani_path).map_err(|e| e.to_string())?;

        // 2. Write to registry
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let mut cursors_key = hkcu
            .open_subkey_with_flags(REGISTRY_CURSORS_KEY, KEY_WRITE)
            .map_err(|e| e.to_string())?;

        let target_path_str = target_ani_path.to_string_lossy().to_string();

        // Update main cursor types
        cursors_key
            .set_value("Arrow", &target_path_str)
            .map_err(|e| e.to_string())?;
        cursors_key
            .set_value("AppStarting", &target_path_str)
            .map_err(|e| e.to_string())?;
        cursors_key
            .set_value("Wait", &target_path_str)
            .map_err(|e| e.to_string())?;
        cursors_key
            .set_value("Hand", &target_path_str)
            .map_err(|e| e.to_string())?;

        // 3. Broadcast setting change
        unsafe {
            use windows_sys::Win32::UI::WindowsAndMessaging::{
                SystemParametersInfoW, SPIF_SENDCHANGE, SPI_SETCURSORS,
            };
            SystemParametersInfoW(SPI_SETCURSORS, 0, std::ptr::null_mut(), SPIF_SENDCHANGE);
        }
    }

    #[cfg(target_os = "linux")]
    {
        // 1. Parse .ani once — shared across all cursor files
        let data = fs::read(ani_path).map_err(|e| e.to_string())?;
        // cap: malicious/malformed .ani >50MB → reject early
        if data.len() > 50 * 1024 * 1024 {
            return Err("ANI file too large: >50MB".to_string());
        }
        let cursor_info = parser::parse_ani(&data)?;

        if cursor_info.frames.is_empty() {
            return Err("ANI file has no frames".to_string());
        }

        // 2. Detect target size from filename, decode + resize all frames
        let size_label = crate::pack::detect_size_label(ani_path.to_string_lossy().as_ref());
        let target_size = match size_label.as_ref() {
            "256" => 256,
            "72-96-128" => 128,
            "96" => 96,
            "32-48-64" => 64,
            _ => 32,
        };
        let all_frames = decode_frames(&cursor_info.frames, cursor_info.rate, target_size)?;

        // 3. Set up Xcursor directory — write atomically to staging, then rename
        let home = dirs::home_dir().ok_or("Cannot find home directory")?;
        let icons_dir = home.join(".icons").join("anime-cursor");
        let cursors_dir = icons_dir.join("cursors");
        let stage_dir = icons_dir.join("cursors-staging");
        let previous_dir = icons_dir.join("cursors-previous");
        let _ = fs::remove_dir_all(&stage_dir);
        let _ = fs::remove_dir_all(&previous_dir);
        fs::create_dir_all(&stage_dir).map_err(|e| e.to_string())?;

        // 4. Write index.theme at icons_dir level — required by libXcursor for theme discovery
        let index_theme = icons_dir.join("index.theme");
        let mut idx = File::create(&index_theme).map_err(|e| e.to_string())?;
        idx.write_all(b"[Icon Theme]\nName=anime-cursor\nComment=Anime Cursor Theme\nDirectories=cursors\n\n[cursors]\nSize=32\nType=Scalable\nMinSize=16\nMaxSize=64\n")
            .map_err(|e| e.to_string())?;

        // 5. Write one Xcursor file per cursor name — static or animated
        // jiffies → ms; clamp rate ≥1 so a malformed rate=0 can't produce 0ms animation delay
        let delay_ms = cursor_info.rate.max(1) * 1000 / 60;
        for (name, animated) in CURSOR_NAMES {
            let frames: Vec<_> = if *animated {
                all_frames.clone()
            } else {
                vec![all_frames[0].clone()]
            };
            let cursor_path = stage_dir.join(name);
            let mut f = File::create(&cursor_path).map_err(|e| e.to_string())?;
            write_xcursor(
                &mut f,
                &frames,
                if *animated { delay_ms } else { 0 },
                target_size,
            )?;
        }

        // 6. Same-filesystem swap with rollback. Never delete active cursors first.
        if cursors_dir.exists() {
            fs::rename(&cursors_dir, &previous_dir).map_err(|e| e.to_string())?;
        }
        if let Err(error) = fs::rename(&stage_dir, &cursors_dir) {
            if previous_dir.exists() {
                let _ = fs::rename(&previous_dir, &cursors_dir);
            }
            return Err(error.to_string());
        }
        let _ = fs::remove_dir_all(&previous_dir);

        // 6a. Create symlinks for common GTK/X11 cursor names so GTK doesn't spam warnings
        for (alias, target) in SYMLINK_MAP {
            let link = cursors_dir.join(alias);
            let target_path = cursors_dir.join(target);
            if target_path.exists() {
                let _ = std::os::unix::fs::symlink(target_path, link);
            }
        }

        // 7. Set via gsettings — toggle to flush GTK cache, then apply
        let _ = std::process::Command::new("gsettings")
            .args([
                "set",
                "org.gnome.desktop.interface",
                "cursor-theme",
                "Adwaita",
            ])
            .status();
        let status = std::process::Command::new("gsettings")
            .args([
                "set",
                "org.gnome.desktop.interface",
                "cursor-theme",
                "anime-cursor",
            ])
            .status()
            .map_err(|e| e.to_string())?;

        if !status.success() {
            return Err("Failed to set gsettings cursor-theme".to_string());
        }

        // Size set via gsettings (GNOME) or kapplymousetheme (KDE)
        let _ = std::process::Command::new("gsettings")
            .args(["set", "org.gnome.desktop.interface", "cursor-size", "32"])
            .status();
    }

    #[cfg(target_os = "macos")]
    {
        return Err(
            "macOS cursor theming not supported yet. Use Mousecape or similar tool.".to_string(),
        );
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn decode_frames(
    frames: &[parser::Frame],
    _rate: u32,
    target_size: u32,
) -> Result<Vec<DecodedFrame>, String> {
    // (width, height, delay_ms, rgba, hot_x, hot_y)
    let mut out = Vec::with_capacity(frames.len());
    for frame in frames {
        let mut data_to_decode = frame.data.clone();
        crate::pack::ensure_ico_data(&mut data_to_decode);
        let mut img = image::load_from_memory(&data_to_decode).map_err(|e| e.to_string())?;
        let orig_w = img.width().max(1);
        let orig_h = img.height().max(1);
        if img.width() != target_size || img.height() != target_size {
            img = img.resize_exact(
                target_size,
                target_size,
                image::imageops::FilterType::Lanczos3,
            );
        }
        let rgba = img.to_rgba8().into_raw();
        let scaled_hot_x = frame.hotspot_x * target_size / orig_w;
        let scaled_hot_y = frame.hotspot_y * target_size / orig_h;
        let clamped_hot_x = scaled_hot_x.min(target_size - 1);
        let clamped_hot_y = scaled_hot_y.min(target_size - 1);
        out.push((
            target_size,
            target_size,
            0u32,
            rgba,
            clamped_hot_x,
            clamped_hot_y,
        ));
    }
    Ok(out)
}

#[cfg(target_os = "linux")]
fn write_xcursor(
    wrt: &mut File,
    decoded_frames: &[DecodedFrame],
    delay: u32,
    target_size: u32,
) -> Result<(), String> {
    let magic = b"Xcur";
    let header_size = 16u32;
    let version = 0x00010000u32;
    let toc_count = decoded_frames.len() as u32;

    wrt.write_all(magic).map_err(|e| e.to_string())?;
    wrt.write_all(&header_size.to_le_bytes())
        .map_err(|e| e.to_string())?;
    wrt.write_all(&version.to_le_bytes())
        .map_err(|e| e.to_string())?;
    wrt.write_all(&toc_count.to_le_bytes())
        .map_err(|e| e.to_string())?;

    let mut current_offset = header_size + (toc_count * 12);
    let pixel_data_size = target_size * target_size * 4;

    // Write TOC (Table of Contents)
    for _ in decoded_frames {
        let chunk_type = 0xfffd0002u32; // TYPE_IMAGE
        wrt.write_all(&chunk_type.to_le_bytes())
            .map_err(|e| e.to_string())?;
        wrt.write_all(&target_size.to_le_bytes())
            .map_err(|e| e.to_string())?;
        wrt.write_all(&current_offset.to_le_bytes())
            .map_err(|e| e.to_string())?;

        current_offset += 36 + pixel_data_size;
    }

    // Write Image Chunks
    for (_, _, _, rgba, xhot, yhot) in decoded_frames {
        let chunk_header_size = 36u32;
        let chunk_type = 0xfffd0002u32;
        let chunk_version = 1u32;

        wrt.write_all(&chunk_header_size.to_le_bytes())
            .map_err(|e| e.to_string())?;
        wrt.write_all(&chunk_type.to_le_bytes())
            .map_err(|e| e.to_string())?;
        wrt.write_all(&target_size.to_le_bytes())
            .map_err(|e| e.to_string())?;
        wrt.write_all(&chunk_version.to_le_bytes())
            .map_err(|e| e.to_string())?;
        wrt.write_all(&target_size.to_le_bytes())
            .map_err(|e| e.to_string())?;
        wrt.write_all(&target_size.to_le_bytes())
            .map_err(|e| e.to_string())?;
        wrt.write_all(&xhot.to_le_bytes())
            .map_err(|e| e.to_string())?;
        wrt.write_all(&yhot.to_le_bytes())
            .map_err(|e| e.to_string())?;
        wrt.write_all(&delay.to_le_bytes())
            .map_err(|e| e.to_string())?;

        wrt.write_all(rgba).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn test_apply_example_ani() {
        let path = "../.example/ani/madoka-magica-normal-9f4a71cf/Madoka Magica - Miki Sayaka - Puella Magi Madoka M_256.ani";
        apply_cursor_variant(path).expect("Failed to apply cursor variant");
    }
    #[test]
    #[cfg(target_os = "linux")]
    fn test_cursor_size_not_changed_on_repeated_apply() {
        let _lock = crate::GSETTINGS_TEST_LOCK.lock().unwrap();
        let path = "../.example/ani/madoka-magica-normal-9f4a71cf/Madoka Magica - Miki Sayaka - Puella Magi Madoka M_256.ani";

        apply_cursor_variant(path).expect("Apply should succeed");
        let first_size = std::process::Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "cursor-size"])
            .output()
            .unwrap()
            .stdout;

        apply_cursor_variant(path).expect("Second apply should succeed");
        let second_size = std::process::Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "cursor-size"])
            .output()
            .unwrap()
            .stdout;

        assert_eq!(
            first_size, second_size,
            "Cursor size should not change on repeated apply"
        );

        let _ = std::process::Command::new("gsettings")
            .args([
                "set",
                "org.gnome.desktop.interface",
                "cursor-theme",
                "Adwaita",
            ])
            .status();
        let _ = std::process::Command::new("gsettings")
            .args(["set", "org.gnome.desktop.interface", "cursor-size", "32"])
            .status();
        if let Some(home) = dirs::home_dir() {
            let _ = std::fs::remove_dir_all(home.join(".icons").join("anime-cursor"));
        }
    }

    #[test]
    fn test_symlink_map_no_pointer_self_reference() {
        let mut found_pointer_pointer = false;
        for (alias, target) in SYMLINK_MAP {
            if *alias == "pointer" && *target == "pointer" {
                found_pointer_pointer = true;
            }
        }
        assert!(
            !found_pointer_pointer,
            "SYMLINK_MAP should not contain (\"pointer\", \"pointer\") self-reference"
        );
    }
}
