use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use zip::ZipArchive;

use crate::instances;

/// Supported import formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportFormat {
    Mrpack,      // Modrinth .mrpack
    CurseForge,  // CurseForge zip with manifest.json
    MultiMC,     // MultiMC/Prism/GDLauncher instance.cfg
    Unknown,
}

/// Import result
#[derive(Debug, Serialize)]
pub struct ImportResult {
    pub success: bool,
    pub instance_id: String,
    pub name: String,
    pub message: String,
}

/// Detect format from zip contents
pub fn detect_format(zip: &mut ZipArchive<std::fs::File>) -> ImportFormat {
    let has_mrpack_manifest = zip.by_name("modrinth.index.json").is_ok();
    let has_curseforge_manifest = zip.by_name("manifest.json").is_ok();
    let has_multimc_cfg = zip.by_name("instance.cfg").is_ok();

    if has_mrpack_manifest {
        ImportFormat::Mrpack
    } else if has_curseforge_manifest {
        ImportFormat::CurseForge
    } else if has_multimc_cfg {
        ImportFormat::MultiMC
    } else {
        ImportFormat::Unknown
    }
}

/// Import a zip or mrpack file as an instance
pub async fn import_zip(zip_path: &str) -> Result<ImportResult, String> {
    let clean_path = zip_path.trim_matches('"').trim_start_matches("file:///").trim_start_matches("file://");
    let path = PathBuf::from(clean_path);
    if !path.exists() {
        return Err(format!("File not found: {}", clean_path));
    }

    let file = fs::File::open(&path).map_err(|e| format!("Cannot open file: {}", e))?;
    let mut zip = ZipArchive::new(file).map_err(|e| format!("Invalid zip: {}", e))?;

    let format = detect_format(&mut zip);
    eprintln!("[Import] Detected format: {:?}", format);

    match format {
        ImportFormat::Mrpack => import_mrpack(&mut zip).await,
        ImportFormat::CurseForge => import_curseforge(&mut zip),
        ImportFormat::MultiMC => import_multimc(&mut zip),
        ImportFormat::Unknown => Err("Unsupported archive format. Expected .mrpack, CurseForge zip, or MultiMC instance.".to_string()),
    }
}

/// Import Modrinth .mrpack format
async fn import_mrpack(zip: &mut ZipArchive<std::fs::File>) -> Result<ImportResult, String> {
    // Read modrinth.index.json
    let mut manifest_str = String::new();
    zip.by_name("modrinth.index.json")
        .map_err(|e| format!("Cannot read modrinth.index.json: {}", e))?
        .read_to_string(&mut manifest_str)
        .map_err(|e| e.to_string())?;

    let manifest: MrpackManifest = serde_json::from_str(&manifest_str)
        .map_err(|e| format!("Parse error: {}", e))?;

    let name = manifest.name.clone().unwrap_or_else(|| "Imported Modpack".to_string());
    let mc_ver = manifest.dependencies.get("minecraft").map(|s| s.as_str()).unwrap_or("1.20.1");
    let loader = manifest.dependencies.get("fabric-loader").map(|_| "fabric")
        .or_else(|| manifest.dependencies.get("forge").map(|_| "forge"))
        .or_else(|| manifest.dependencies.get("neoforge").map(|_| "neoforge"))
        .or_else(|| manifest.dependencies.get("quilt-loader").map(|_| "quilt"));
    
    let loader_version = manifest.dependencies.get("fabric-loader")
        .or_else(|| manifest.dependencies.get("forge"))
        .or_else(|| manifest.dependencies.get("neoforge"))
        .or_else(|| manifest.dependencies.get("quilt-loader"))
        .map(|s| s.as_str());

    // Create instance
    let info = instances::create_instance(
        &name,
        mc_ver,
        loader,
        loader_version,
        None,
    )?;

    // Extract files
    let instances_dir = instances::get_instances_dir()?;
    let instance_dir = instances_dir.join(&info.id);
    let game_dir = instance_dir.join(".minecraft");
    fs::create_dir_all(&game_dir).ok();

    for i in 0..zip.len() {
        if let Ok(mut file) = zip.by_index(i) {
            let name = file.name().to_string();
            if name.starts_with("modrinth.index.json") { continue; }
            if name.starts_with("overrides/") {
                let rel = name.strip_prefix("overrides/").unwrap_or(&name);
                let out_path = game_dir.join(rel);
                if let Some(parent) = out_path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                let _ = std::io::copy(&mut file, &mut fs::File::create(&out_path).unwrap_or_else(|_| fs::File::create(game_dir.join("dummy")).unwrap()));
            }
        }
    }

    // Download files listed in .mrpack manifest
    let mut tasks = Vec::new();
    for file_entry in manifest.files {
        if let Some(download_url) = file_entry.downloads.first().cloned() {
            let out_path = game_dir.join(&file_entry.path);
            if !out_path.exists() {
                tasks.push(tokio::spawn(async move {
                    if let Some(parent) = out_path.parent() {
                        let _ = fs::create_dir_all(parent);
                    }
                    if let Ok(resp) = reqwest::get(&download_url).await {
                        if let Ok(bytes) = resp.bytes().await {
                            let _ = fs::write(&out_path, &bytes);
                        }
                    }
                }));
            }
        }
    }
    for task in tasks {
        let _ = task.await;
    }

    Ok(ImportResult {
        success: true,
        instance_id: info.id,
        name: info.name,
        message: format!("Imported Modrinth modpack: {}", name),
    })
}

#[derive(Debug, Deserialize)]
struct MrpackFile {
    path: String,
    downloads: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct MrpackManifest {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    dependencies: std::collections::HashMap<String, String>,
    #[serde(default)]
    files: Vec<MrpackFile>,
}

/// Import CurseForge zip format
fn import_curseforge(zip: &mut ZipArchive<std::fs::File>) -> Result<ImportResult, String> {
    let mut manifest_str = String::new();
    zip.by_name("manifest.json")
        .map_err(|e| format!("Cannot read manifest.json: {}", e))?
        .read_to_string(&mut manifest_str)
        .map_err(|e| e.to_string())?;

    let manifest: CurseForgeManifest = serde_json::from_str(&manifest_str)
        .map_err(|e| format!("Parse error: {}", e))?;

    let name = manifest.name.clone().unwrap_or_else(|| "Imported Modpack".to_string());

    let mut loader = None;
    let mut loader_version = None;
    
    if let Some(loaders) = &manifest.minecraft.modLoaders {
        if let Some(primary) = loaders.iter().find(|l| l.primary).or(loaders.first()) {
            let id = &primary.id;
            if let Some(stripped) = id.strip_prefix("forge-") {
                loader = Some("forge");
                loader_version = Some(stripped);
            } else if let Some(stripped) = id.strip_prefix("fabric-") {
                loader = Some("fabric");
                loader_version = Some(stripped);
            } else if let Some(stripped) = id.strip_prefix("neoforge-") {
                loader = Some("neoforge");
                loader_version = Some(stripped);
            } else if let Some(stripped) = id.strip_prefix("quilt-") {
                loader = Some("quilt");
                loader_version = Some(stripped);
            }
        }
    }

    let info = instances::create_instance(
        &name,
        &manifest.minecraft.version,
        loader,
        loader_version,
        None,
    )?;

    // Extract modpack files
    let instances_dir = instances::get_instances_dir()?;
    let instance_dir = instances_dir.join(&info.id);
    let game_dir = instance_dir.join(".minecraft");
    fs::create_dir_all(&game_dir).ok();

    let overrides_path = format!("{}/", manifest.overrides.as_deref().unwrap_or("overrides"));

    for i in 0..zip.len() {
        if let Ok(mut file) = zip.by_index(i) {
            let name = file.name().to_string();
            if name == "manifest.json" { continue; }
            if name.starts_with(&overrides_path) {
                let rel = name.strip_prefix(&overrides_path).unwrap_or(&name);
                let out_path = game_dir.join(rel);
                if let Some(parent) = out_path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                let _ = std::io::copy(&mut file, &mut fs::File::create(&out_path).unwrap_or_else(|_| fs::File::create(game_dir.join("dummy")).unwrap()));
            }
        }
    }

    Ok(ImportResult {
        success: true,
        instance_id: info.id,
        name: info.name,
        message: format!("Imported CurseForge pack: {}", name),
    })
}

#[derive(Debug, Deserialize)]
struct CurseForgeManifest {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    overrides: Option<String>,
    minecraft: CurseForgeMinecraft,
}

#[derive(Debug, Deserialize)]
struct CurseForgeMinecraft {
    version: String,
    #[serde(default)]
    modLoaders: Option<Vec<CurseForgeModLoader>>,
}

#[derive(Debug, Deserialize)]
struct CurseForgeModLoader {
    id: String,
    primary: bool,
}

/// Import MultiMC/Prism format
fn import_multimc(zip: &mut ZipArchive<std::fs::File>) -> Result<ImportResult, String> {
    let mut cfg_str = String::new();
    zip.by_name("instance.cfg")
        .map_err(|e| format!("Cannot read instance.cfg: {}", e))?
        .read_to_string(&mut cfg_str)
        .map_err(|e| e.to_string())?;

    // Simple key=value parsing for instance.cfg
    let name = extract_cfg_value(&cfg_str, "name").unwrap_or_else(|| "Imported Instance".to_string());
    let version = extract_cfg_value(&cfg_str, "IntendedMcVersion").unwrap_or_else(|| "1.20.1".to_string());

    let info = instances::create_instance(&name, &version, None, None, None)?;

    // Extract .minecraft folder if present
    let instances_dir = instances::get_instances_dir()?;
    let instance_dir = instances_dir.join(&info.id);
    let game_dir = instance_dir.join(".minecraft");

    for i in 0..zip.len() {
        if let Ok(mut file) = zip.by_index(i) {
            let fname = file.name().to_string();
            if fname == "instance.cfg" { continue; }
            let out_path = game_dir.join(&fname);
            if let Some(parent) = out_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = std::io::copy(&mut file, &mut fs::File::create(&out_path).unwrap_or_else(|_| fs::File::create(game_dir.join("dummy")).unwrap()));
        }
    }

    Ok(ImportResult {
        success: true,
        instance_id: info.id,
        name: info.name,
        message: format!("Imported MultiMC instance: {}", name),
    })
}

fn extract_cfg_value(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        if let Some(val) = line.strip_prefix(&format!("{}=", key)) {
            return Some(val.trim().to_string());
        }
    }
    None
}
