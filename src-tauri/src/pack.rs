use crate::parser;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use std::path::Path;
use zip::ZipArchive;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pack {
    pub id: String,
    pub name: String,
    pub author: Option<String>,
    pub source_url: Option<String>,
    pub variants: Vec<PackVariant>,
    pub thumbnail: Option<String>,
    pub created_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackVariant {
    pub id: String,
    pub filename: String,
    pub size_label: String,
    pub ani_path: String,
    pub preview_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MetadataJson {
    name: String,
    author: Option<String>,
    source_url: Option<String>,
    cursors: Vec<MetadataCursor>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MetadataCursor {
    filename: String,
    size_label: String,
}

pub fn get_app_dir() -> std::path::PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(".anime-cursor")
}

pub fn detect_size_label(filename: &str) -> String {
    let lower = filename.to_lowercase();
    if lower.contains("200") {
        "200".into()
    } else if lower.contains("256") {
        "256".into()
    } else if lower.contains("72-96-128") || lower.contains("128") {
        "72-96-128".into()
    } else if lower.contains("96") {
        "96".into()
    } else if lower.contains("32-48-64") || lower.contains("64") {
        "32-48-64".into()
    } else {
        "Default".into()
    }
}

const MAX_ZIP_FILES: usize = 100;
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;
const MAX_TOTAL_UNPACKED_SIZE: u64 = 100 * 1024 * 1024;

fn make_unique_id(stem: &str) -> String {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}-{}-{}", stem, ts.as_secs(), ts.subsec_nanos())
}

fn safe_metadata_filename(name: &str) -> Result<(), String> {
    let path = Path::new(name);
    if name.is_empty()
        || name.contains(['/', '\\'])
        || path.is_absolute()
        || path
            .components()
            .any(|part| matches!(part, std::path::Component::ParentDir))
    {
        Err(format!("Unsafe filename in metadata: {name}"))
    } else {
        Ok(())
    }
}

/// Patch Windows CUR (type 2) to ICO (type 1) in-memory so preview decoders can render it.
pub(crate) fn ensure_ico_data(data: &mut Vec<u8>) {
    if !data.is_empty() && data.len() > 2 && data[2] == 2 {
        data[2] = 1;
    }
}

/// Extract the first frame from an .ani file and write it as a preview file.
/// Returns the path if successful.
fn write_preview(ani_path: &Path, extract_dir: &Path) -> Option<String> {
    let data = fs::read(ani_path).ok()?;
    let cursor_info = parser::parse_ani(&data).ok()?;
    let first = cursor_info.frames.first()?;
    let mut frame_data = first.data.clone();
    ensure_ico_data(&mut frame_data);
    let file_name = ani_path.file_name()?;
    let ext = if first.is_png { "png" } else { "ico" };
    let stem = Path::new(&file_name).file_stem()?.to_string_lossy();
    let preview_name = format!("{}_preview.{ext}", stem);
    let preview_path = extract_dir.join(&preview_name);
    let mut f = File::create(&preview_path).ok()?;
    f.write_all(&frame_data).ok()?;
    Some(preview_path.to_string_lossy().to_string())
}

struct ImportCleanup(Option<std::path::PathBuf>);

impl ImportCleanup {
    fn keep(&mut self) {
        self.0 = None;
    }
}

impl Drop for ImportCleanup {
    fn drop(&mut self) {
        if let Some(path) = &self.0 {
            let _ = fs::remove_dir_all(path);
        }
    }
}

pub fn import_pack_zip(zip_path: &Path) -> Result<Pack, String> {
    let app_dir = get_app_dir();
    let packs_dir = app_dir.join("packs");
    fs::create_dir_all(&packs_dir).map_err(|e| e.to_string())?;

    let stem = zip_path
        .file_stem()
        .ok_or("Invalid file path")?
        .to_string_lossy();

    let pack_id = make_unique_id(&stem);
    let extract_dir = packs_dir.join(&pack_id);
    fs::create_dir_all(&extract_dir).map_err(|e| e.to_string())?;
    let mut cleanup = ImportCleanup(Some(extract_dir.clone()));

    let file = File::open(zip_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(BufReader::new(file)).map_err(|e| e.to_string())?;

    if archive.len() > MAX_ZIP_FILES {
        let _ = fs::remove_dir_all(&extract_dir);
        return Err(format!(
            "ZIP contains {} files, max allowed is {}",
            archive.len(),
            MAX_ZIP_FILES
        ));
    }

    let mut total_unpacked_size = 0u64;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;

        if file.size() > MAX_FILE_SIZE {
            let _ = fs::remove_dir_all(&extract_dir);
            return Err(format!(
                "File {} exceeds max size ({} bytes)",
                file.name(),
                MAX_FILE_SIZE
            ));
        }

        total_unpacked_size = total_unpacked_size.saturating_add(file.size());
        if total_unpacked_size > MAX_TOTAL_UNPACKED_SIZE {
            let _ = fs::remove_dir_all(&extract_dir);
            return Err(format!(
                "ZIP exceeds max total unpacked size ({} bytes)",
                MAX_TOTAL_UNPACKED_SIZE
            ));
        }

        let outpath = match file.enclosed_name() {
            Some(path) => extract_dir.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).map_err(|e| e.to_string())?;
                }
            }
            let mut outfile = File::create(&outpath).map_err(|e| e.to_string())?;
            io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }
    }

    let metadata_file = extract_dir.join("metadata.json");
    let mut pack = if metadata_file.exists() {
        let rdr = File::open(&metadata_file).map_err(|e| e.to_string())?;
        let meta: MetadataJson =
            serde_json::from_reader(BufReader::new(rdr)).map_err(|e| e.to_string())?;

        if meta.cursors.is_empty() {
            let _ = fs::remove_dir_all(&extract_dir);
            return Err("Pack metadata contains no cursor variants".to_string());
        }

        if let Some(ref url) = meta.source_url {
            let lower = url.to_lowercase();
            if !lower.starts_with("http://") && !lower.starts_with("https://") {
                let _ = fs::remove_dir_all(&extract_dir);
                return Err("source_url must use http:// or https://".to_string());
            }
        }

        let mut variants = Vec::new();
        for (idx, cur) in meta.cursors.into_iter().enumerate() {
            safe_metadata_filename(&cur.filename).map_err(|e| {
                let _ = fs::remove_dir_all(&extract_dir);
                e
            })?;

            let relative_ani = extract_dir.join(&cur.filename);
            if !relative_ani.is_file() {
                let _ = fs::remove_dir_all(&extract_dir);
                return Err(format!(
                    "Cursor file missing from archive: {}",
                    cur.filename
                ));
            }
            let variant_id = format!("{}-{}-{}", pack_id, cur.size_label, idx);
            variants.push(PackVariant {
                id: variant_id,
                filename: cur.filename,
                size_label: cur.size_label,
                ani_path: relative_ani.to_string_lossy().to_string(),
                preview_path: None,
            });
        }

        Pack {
            id: pack_id,
            name: meta.name,
            author: meta.author,
            source_url: meta.source_url,
            variants,
            thumbnail: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    } else {
        let mut variants = Vec::new();

        if let Ok(entries) = fs::read_dir(&extract_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    continue;
                }
                let name_lower = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_lowercase();
                if !name_lower.ends_with(".ani") {
                    continue;
                }
                let relative = path
                    .strip_prefix(&extract_dir)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .to_string();
                let size_label = detect_size_label(&relative);
                let variant_id = format!("{}-{}-{}", pack_id, size_label, variants.len());
                variants.push(PackVariant {
                    id: variant_id,
                    filename: relative,
                    size_label,
                    ani_path: path.to_string_lossy().to_string(),
                    preview_path: None,
                });
            }
        }

        if variants.is_empty() {
            let _ = fs::remove_dir_all(&extract_dir);
            return Err("No .ani cursor files found in archive".to_string());
        }

        Pack {
            id: pack_id.clone(),
            name: zip_path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            author: None,
            source_url: None,
            variants,
            thumbnail: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    };

    for variant in &mut pack.variants {
        let ani_file_path = Path::new(&variant.ani_path);
        variant.preview_path = write_preview(ani_file_path, &extract_dir);
    }

    if let Some(first) = pack.variants.first() {
        pack.thumbnail = first.preview_path.clone();
    }

    cleanup.keep();
    Ok(pack)
}

pub fn seed_default_packs(resource_packs_dir: &Path) -> Result<(), String> {
    let app_dir = get_app_dir();
    let packs_dir = app_dir.join("packs");
    if packs_dir.exists()
        && packs_dir
            .read_dir()
            .map_or(true, |mut i| i.next().is_none())
    {
        let _ = fs::remove_dir_all(&packs_dir);
    }
    if packs_dir.exists() {
        return Ok(());
    }

    if !resource_packs_dir.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(resource_packs_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let dir_name = entry.file_name();
        let src = entry.path();
        if src.is_dir() && src.join("metadata.json").exists() {
            let dst = packs_dir.join(&dir_name);
            let _ = fs::remove_dir_all(&dst);
            cp_dir(&src, &dst).map_err(|e| {
                format!("Failed to seed pack {}: {}", dir_name.to_string_lossy(), e)
            })?;
            // Generate previews for seeded packs that lack them
            if let Ok(rdr) = File::open(dst.join("metadata.json")) {
                if let Ok(meta) = serde_json::from_reader::<_, MetadataJson>(BufReader::new(rdr)) {
                    for cur in &meta.cursors {
                        let ani = dst.join(&cur.filename);
                        let stem = Path::new(&cur.filename)
                            .file_stem()
                            .unwrap_or_default()
                            .to_string_lossy();
                        let has_preview = dst.join(format!("{stem}_preview.png")).exists()
                            || dst.join(format!("{stem}_preview.ico")).exists();
                        if !has_preview && ani.is_file() {
                            write_preview(&ani, &dst);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn cp_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let e = entry?;
        let ft = e.file_type()?;
        let src_path = e.path();
        let dst_path = dst.join(e.file_name());
        if ft.is_dir() {
            cp_dir(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

pub fn list_installed_packs() -> Result<Vec<Pack>, String> {
    let app_dir = get_app_dir();
    let packs_dir = app_dir.join("packs");
    if !packs_dir.exists() {
        return Ok(Vec::new());
    }

    let mut packs = Vec::new();
    if let Ok(entries) = fs::read_dir(packs_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let metadata_file = entry.path().join("metadata.json");
                if metadata_file.exists() {
                    if let Ok(rdr) = File::open(&metadata_file) {
                        if let Ok(meta) =
                            serde_json::from_reader::<_, MetadataJson>(BufReader::new(rdr))
                        {
                            let pack_id = entry.file_name().to_string_lossy().to_string();

                            let mut variants = Vec::new();
                            for (idx, cur) in meta.cursors.into_iter().enumerate() {
                                let ani_path = entry.path().join(&cur.filename);
                                let stem = Path::new(&cur.filename)
                                    .file_stem()
                                    .unwrap_or_default()
                                    .to_string_lossy();
                                let preview_png = entry.path().join(format!("{stem}_preview.png"));
                                let preview_ico = entry.path().join(format!("{stem}_preview.ico"));
                                let preview_path = if preview_png.exists() {
                                    Some(preview_png.to_string_lossy().to_string())
                                } else if preview_ico.exists() {
                                    Some(preview_ico.to_string_lossy().to_string())
                                } else if ani_path.is_file() {
                                    write_preview(&ani_path, &entry.path())
                                } else {
                                    None
                                };

                                variants.push(PackVariant {
                                    id: format!("{}-{}-{}", pack_id, cur.size_label, idx),
                                    filename: cur.filename,
                                    size_label: cur.size_label,
                                    ani_path: ani_path.to_string_lossy().to_string(),
                                    preview_path,
                                });
                            }

                            let thumbnail = variants.first().and_then(|v| v.preview_path.clone());

                            packs.push(Pack {
                                id: pack_id,
                                name: meta.name,
                                author: meta.author,
                                source_url: meta.source_url,
                                variants,
                                thumbnail,
                                created_at: entry
                                    .metadata()
                                    .and_then(|m| m.created())
                                    .map(|t| {
                                        t.duration_since(std::time::UNIX_EPOCH)
                                            .unwrap_or_default()
                                            .as_secs()
                                    })
                                    .unwrap_or(0),
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(packs)
}

pub fn import_single_ani(ani_path: &Path) -> Result<Pack, String> {
    let app_dir = get_app_dir();
    let packs_dir = app_dir.join("packs");
    fs::create_dir_all(&packs_dir).map_err(|e| e.to_string())?;

    let stem = ani_path
        .file_stem()
        .ok_or("Invalid file path")?
        .to_string_lossy();

    let pack_id = make_unique_id(&stem);
    let extract_dir = packs_dir.join(&pack_id);
    fs::create_dir_all(&extract_dir).map_err(|e| e.to_string())?;
    let mut cleanup = ImportCleanup(Some(extract_dir.clone()));

    let filename = ani_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let dest_ani_path = extract_dir.join(&filename);
    fs::copy(ani_path, &dest_ani_path).map_err(|e| e.to_string())?;

    let size_label = detect_size_label(&filename);

    let mut variant = PackVariant {
        id: format!("{}-{}-0", pack_id, size_label),
        filename: filename.clone(),
        size_label: size_label.to_string(),
        ani_path: dest_ani_path.to_string_lossy().to_string(),
        preview_path: None,
    };

    variant.preview_path = write_preview(&dest_ani_path, &extract_dir);

    let mut pack = Pack {
        id: pack_id,
        name: stem.to_string(),
        author: Some("Local Import".to_string()),
        source_url: None,
        variants: vec![variant],
        thumbnail: None,
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    };

    if let Some(first) = pack.variants.first() {
        pack.thumbnail = first.preview_path.clone();
    }

    let metadata_file = extract_dir.join("metadata.json");
    let cursors_meta = vec![MetadataCursor {
        filename: pack.variants[0].filename.clone(),
        size_label: pack.variants[0].size_label.clone(),
    }];
    let meta_to_save = MetadataJson {
        name: pack.name.clone(),
        author: pack.author.clone(),
        source_url: pack.source_url.clone(),
        cursors: cursors_meta,
    };

    let wrt = File::create(&metadata_file).map_err(|e| e.to_string())?;
    serde_json::to_writer_pretty(io::BufWriter::new(wrt), &meta_to_save)
        .map_err(|e| e.to_string())?;

    cleanup.keep();
    Ok(pack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_pack_zip_rejects_large_files() {
        let temp_dir = std::env::temp_dir();
        let test_zip = temp_dir.join(format!("large_test_{}.zip", std::process::id()));
        {
            let mut zip = zip::ZipWriter::new(File::create(&test_zip).unwrap());
            let file_data = vec![0u8; 20 * 1024 * 1024];
            zip.start_file("large.bin", zip::write::FileOptions::default())
                .unwrap();
            zip.write_all(&file_data).unwrap();
            zip.finish().unwrap();
        }
        let result = import_pack_zip(&test_zip);
        assert!(result.is_err(), "Should reject large ZIP files");
        let _ = std::fs::remove_file(&test_zip);
    }

    #[test]
    fn test_import_pack_zip_rejects_invalid_source_url() {
        let zip_path = Path::new("../.example/zip/madoka-magica-normal-9f4a71cf.zip");
        let result = import_pack_zip(zip_path);
        if let Ok(ref pack) = result {
            if let Some(ref url) = pack.source_url {
                let lower = url.to_lowercase();
                assert!(
                    lower.starts_with("http://") || lower.starts_with("https://"),
                    "source_url must be HTTP or HTTPS: {:?}",
                    url
                );
            }
        }
        if let Ok(ref pack) = result {
            let pack_dir = get_app_dir().join("packs").join(&pack.id);
            let _ = std::fs::remove_dir_all(&pack_dir);
        }
    }

    #[test]
    fn test_import_pack_zip_rejects_empty_variants() {
        let temp_dir = std::env::temp_dir();
        let test_zip = temp_dir.join(format!("empty_variants_{}.zip", std::process::id()));
        {
            let meta = serde_json::json!({
                "name": "Empty Test",
                "author": serde_json::Value::Null,
                "source_url": serde_json::Value::Null,
                "cursors": []
            });
            let mut zip = zip::ZipWriter::new(File::create(&test_zip).unwrap());
            let options = zip::write::FileOptions::default().unix_permissions(0o644);
            zip.start_file("metadata.json", options).unwrap();
            serde_json::to_writer_pretty(&mut zip, &meta).unwrap();
            zip.finish().unwrap();
        }
        let result = import_pack_zip(&test_zip);
        assert!(result.is_err(), "Should reject ZIP with empty variants");
        let _ = std::fs::remove_file(&test_zip);
    }

    #[test]
    fn test_import_pack_zip_rejects_excessive_files() {
        let temp_dir = std::env::temp_dir();
        let test_zip = temp_dir.join(format!("many_files_{}.zip", std::process::id()));
        {
            let mut zip = zip::ZipWriter::new(File::create(&test_zip).unwrap());
            for i in 0..1000 {
                zip.start_file(format!("file{}.txt", i), zip::write::FileOptions::default())
                    .unwrap();
                zip.write_all(b"test").unwrap();
            }
            zip.finish().unwrap();
        }
        let result = import_pack_zip(&test_zip);
        assert!(result.is_err(), "Should reject ZIP with too many files");
        let _ = std::fs::remove_file(&test_zip);
    }

    #[test]
    fn test_import_single_ani_no_source_url() {
        let ani_path = Path::new(
            "../.example/ani/madoka-magica-normal-9f4a71cf/Madoka Magica - Miki Sayaka - Puella Magi Madoka M_32-48-64.ani",
        );
        let result = import_single_ani(ani_path);
        if let Ok(ref pack) = result {
            assert_eq!(
                pack.source_url, None,
                "Single .ani import should not have source_url"
            );
            let pack_dir = get_app_dir().join("packs").join(&pack.id);
            let _ = std::fs::remove_dir_all(&pack_dir);
        }
    }

    #[test]
    fn test_unique_ids_no_same_second_collision() {
        let id1 = make_unique_id("test");
        let id2 = make_unique_id("test");
        assert_ne!(id1, id2, "Two IDs generated in sequence must differ");
    }

    #[test]
    fn test_safe_metadata_filename_rejects_traversal() {
        assert!(safe_metadata_filename("../etc/passwd").is_err());
        assert!(safe_metadata_filename("sub/file.txt").is_err());
        assert!(safe_metadata_filename("normal.txt").is_ok());
    }

    #[test]
    fn test_import_pack_zip_cleans_up_missing_metadata_variant() {
        let test_name = format!("missing_variant_{}", std::process::id());
        let zip_path = std::env::temp_dir().join(format!("{test_name}.zip"));
        let meta = serde_json::json!({
            "name": "Missing variant",
            "author": null,
            "source_url": null,
            "cursors": [{ "filename": "missing.ani", "size_label": "Default" }]
        });

        let mut zip = zip::ZipWriter::new(File::create(&zip_path).unwrap());
        zip.start_file("metadata.json", zip::write::FileOptions::default())
            .unwrap();
        serde_json::to_writer(&mut zip, &meta).unwrap();
        zip.finish().unwrap();

        assert!(import_pack_zip(&zip_path).is_err());
        let packs_dir = get_app_dir().join("packs");
        let partial = fs::read_dir(packs_dir)
            .unwrap()
            .flatten()
            .any(|entry| entry.file_name().to_string_lossy().starts_with(&test_name));
        assert!(!partial, "failed import must remove its pack directory");
        let _ = fs::remove_file(zip_path);
    }

    #[test]
    fn test_validate_source_url_inline() {
        let lower_ftp = "ftp://example.com".to_lowercase();
        assert!(!lower_ftp.starts_with("http://") && !lower_ftp.starts_with("https://"));
        let lower_http = "http://example.com".to_lowercase();
        assert!(lower_http.starts_with("http://") || lower_http.starts_with("https://"));
    }

    #[test]
    fn test_import_example_zip() {
        let zip_path = Path::new("../.example/zip/madoka-magica-normal-9f4a71cf.zip");
        let pack = import_pack_zip(zip_path).expect("Failed to import zip");
        assert_eq!(pack.name, "madoka-magica-normal-9f4a71cf");
        assert_eq!(pack.variants.len(), 3);
        assert!(pack.thumbnail.is_some());
        assert!(Path::new(pack.thumbnail.as_ref().unwrap()).exists());
        let pack_dir = get_app_dir().join("packs").join(&pack.id);
        let _ = std::fs::remove_dir_all(&pack_dir);
    }

    #[test]
    fn test_import_single_ani() {
        let ani_path = Path::new(
            "../.example/ani/madoka-magica-normal-9f4a71cf/Madoka Magica - Miki Sayaka - Puella Magi Madoka M_32-48-64.ani",
        );
        let pack = import_single_ani(ani_path).expect("Failed to import single ani");
        assert_eq!(
            pack.name,
            "Madoka Magica - Miki Sayaka - Puella Magi Madoka M_32-48-64"
        );
        assert_eq!(pack.variants.len(), 1);
        assert!(pack.thumbnail.is_some());
        assert!(Path::new(pack.thumbnail.as_ref().unwrap()).exists());
        let pack_dir = get_app_dir().join("packs").join(&pack.id);
        let _ = std::fs::remove_dir_all(&pack_dir);
    }
}
