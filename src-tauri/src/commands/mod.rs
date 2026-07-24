use crate::accounts;
use crate::instances;
use crate::versions;
use crate::launch;
use crate::modplatform;
use crate::import_mod;
use serde_json;

#[tauri::command]
pub fn list_instances() -> Result<String, String> {
    use base64::Engine;
    let mut list = instances::list_instances()?;
    let instances_dir = instances::get_instances_dir()?;
    for inst in list.iter_mut() {
        if let Some(ref icon) = inst.icon {
            if !icon.starts_with("http") && !icon.starts_with("data:") {
                let icon_path = instances_dir.join(&inst.id).join(icon);
                if icon_path.exists() {
                    if let Ok(bytes) = std::fs::read(&icon_path) {
                        let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
                        inst.icon = Some(format!("data:image/png;base64,{}", b64));
                    }
                }
            }
        }
    }
    serde_json::to_string(&list).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_instance(params_json: String) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct CreateParams {
        name: String,
        mc_version: String,
        #[serde(default)]
        loader: Option<String>,
        #[serde(default)]
        loader_version: Option<String>,
        #[serde(default)]
        group: Option<String>,
    }

    let params: CreateParams = serde_json::from_str(&params_json)
        .map_err(|e| format!("Invalid params: {}", e))?;

    let info = instances::create_instance(
        &params.name,
        &params.mc_version,
        params.loader.as_deref(),
        params.loader_version.as_deref(),
        params.group.as_deref(),
    )?;

    serde_json::to_string(&info).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_instance_config(params_json: String) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct UpdateParams {
        id: String,
        name: Option<String>,
        group: Option<Option<String>>,
        jvm_args: Option<Option<String>>,
        icon: Option<Option<String>>,
        mc_version: Option<String>,
        loader: Option<Option<String>>,
        loader_version: Option<Option<String>>,
        min_memory_mb: Option<Option<u32>>,
        max_memory_mb: Option<Option<u32>>,
        game_width: Option<Option<u32>>,
        game_height: Option<Option<u32>>,
        fullscreen: Option<Option<bool>>,
    }

    let params: UpdateParams = serde_json::from_str(&params_json)
        .map_err(|e| format!("Invalid params: {}", e))?;

    let cfg = instances::update_instance_config(
        &params.id,
        params.name,
        params.group,
        params.jvm_args,
        params.icon,
        params.mc_version,
        params.loader,
        params.loader_version,
        params.min_memory_mb,
        params.max_memory_mb,
        params.game_width,
        params.game_height,
        params.fullscreen,
    )?;
    serde_json::to_string(&cfg).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_instance(instance_id: String) -> Result<(), String> {
    instances::delete_instance(&instance_id)
}

#[tauri::command]
pub fn get_instance_config(instance_id: String) -> Result<String, String> {
    let cfg = instances::get_instance_config(&instance_id)?;
    serde_json::to_string(&cfg).map_err(|e| e.to_string())
}

#[derive(serde::Serialize)]
pub struct InstalledModInfo {
    pub filename: String,
    pub name: String,
    pub enabled: bool,
    pub size_bytes: u64,
    pub icon_base64: Option<String>,
}

fn get_mod_icon_base64(jar_path: &std::path::Path, mods_dir: &std::path::Path, filename: &str) -> Option<String> {
    use base64::Engine;
    let icons_dir = mods_dir.join(".icons");
    let _ = std::fs::create_dir_all(&icons_dir);
    let icon_dest = icons_dir.join(format!("{}.png", filename));
    
    if !icon_dest.exists() {
        if let Ok(file) = std::fs::File::open(jar_path) {
            if let Ok(mut archive) = zip::ZipArchive::new(file) {
                let mut icon_in_jar = None;
                if let Ok(mut f) = archive.by_name("fabric.mod.json") {
                    let mut contents = String::new();
                    if std::io::Read::read_to_string(&mut f, &mut contents).is_ok() {
                        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&contents) {
                            if let Some(i) = v.get("icon").and_then(|i| i.as_str()) {
                                icon_in_jar = Some(i.to_string());
                            }
                        }
                    }
                }
                if icon_in_jar.is_none() {
                    if let Ok(mut f) = archive.by_name("META-INF/mods.toml") {
                        let mut contents = String::new();
                        if std::io::Read::read_to_string(&mut f, &mut contents).is_ok() {
                            for line in contents.lines() {
                                if line.starts_with("logoFile=") {
                                    let p = line.trim_start_matches("logoFile=").trim_matches('"');
                                    icon_in_jar = Some(p.to_string());
                                    break;
                                }
                            }
                        }
                    }
                }
                if icon_in_jar.is_none() && archive.by_name("logo.png").is_ok() {
                    icon_in_jar = Some("logo.png".to_string());
                }
                if let Some(ipath) = icon_in_jar {
                    if let Ok(mut icon_file) = archive.by_name(&ipath) {
                        if let Ok(mut out) = std::fs::File::create(&icon_dest) {
                            let _ = std::io::copy(&mut icon_file, &mut out);
                        }
                    }
                }
            }
        }
    }
    
    if icon_dest.exists() {
        if let Ok(bytes) = std::fs::read(&icon_dest) {
            let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
            return Some(format!("data:image/png;base64,{}", b64));
        }
    }
    None
}

#[tauri::command]
pub fn list_installed_mods(instance_id: String) -> Result<String, String> {
    let appdata = dirs::data_local_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("AppData").join("Roaming")))
        .ok_or("Cannot determine AppData")?;
    let mods_dir = appdata.join("MyL").join("instances").join(&instance_id).join(".minecraft").join("mods");

    let mut result = Vec::new();
    if mods_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&mods_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
                    let is_jar = filename.ends_with(".jar");
                    let is_off = filename.ends_with(".jar.off");
                    if is_jar || is_off {
                        let metadata = entry.metadata().ok();
                        let size_bytes = metadata.map(|m| m.len()).unwrap_or(0);
                        let name = if is_off {
                            filename.trim_end_matches(".jar.off").to_string()
                        } else {
                            filename.trim_end_matches(".jar").to_string()
                        };
                        let icon_base64 = if is_jar { get_mod_icon_base64(&path, &mods_dir, &filename) } else { None };
                        result.push(InstalledModInfo {
                            filename: filename.clone(),
                            name,
                            enabled: is_jar,
                            size_bytes,
                            icon_base64,
                        });
                    }
                }
            }
        }
    }
    result.sort_by(|a, b| a.name.cmp(&b.name));
    serde_json::to_string(&result).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_mod(instance_id: String, filename: String, enable: bool) -> Result<(), String> {
    let appdata = dirs::data_local_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("AppData").join("Roaming")))
        .ok_or("Cannot determine AppData")?;
    let mods_dir = appdata.join("MyL").join("instances").join(&instance_id).join(".minecraft").join("mods");
    let current_path = mods_dir.join(&filename);

    if !current_path.exists() {
        return Err(format!("Mod file '{}' does not exist", filename));
    }

    if enable && filename.ends_with(".jar.off") {
        let new_name = filename.trim_end_matches(".off");
        let target_path = mods_dir.join(new_name);
        std::fs::rename(current_path, target_path).map_err(|e| format!("Rename error: {}", e))?;
    } else if !enable && filename.ends_with(".jar") {
        let new_name = format!("{}.off", filename);
        let target_path = mods_dir.join(new_name);
        std::fs::rename(current_path, target_path).map_err(|e| format!("Rename error: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
pub fn delete_mod(instance_id: String, filename: String) -> Result<(), String> {
    let appdata = dirs::data_local_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("AppData").join("Roaming")))
        .ok_or("Cannot determine AppData")?;
    let target = appdata.join("MyL").join("instances").join(&instance_id).join(".minecraft").join("mods").join(&filename);
    if target.exists() {
        std::fs::remove_file(target).map_err(|e| format!("Delete error: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
pub fn add_mod_file(instance_id: String, src_path: String) -> Result<String, String> {
    let appdata = dirs::data_local_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("AppData").join("Roaming")))
        .ok_or("Cannot determine AppData")?;
    let mods_dir = appdata.join("MyL").join("instances").join(&instance_id).join(".minecraft").join("mods");
    std::fs::create_dir_all(&mods_dir).ok();

    let src = std::path::Path::new(&src_path);
    if !src.exists() {
        return Err(format!("Source file does not exist: {}", src_path));
    }
    let filename = src.file_name().and_then(|n| n.to_str()).ok_or("Invalid filename")?;
    let dest = mods_dir.join(filename);
    std::fs::copy(src, &dest).map_err(|e| format!("Copy failed: {}", e))?;
    Ok(filename.to_string())
}

#[tauri::command]
pub fn stop_game_by_pid(pid: u32) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("taskkill")
            .args(["/F", "/PID", &pid.to_string()])
            .output();
    }
    let _ = launch::stop_game();
    Ok(())
}


// === Account commands ===

#[tauri::command]
pub fn list_accounts() -> Result<String, String> {
    let active_id = accounts::get_active_id();
    let list = accounts::list_accounts(active_id.as_deref())?;
    serde_json::to_string(&list).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_account_offline(username: String) -> Result<String, String> {
    let info = accounts::add_offline(&username)?;
    serde_json::to_string(&info).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_account(account_id: String) -> Result<(), String> {
    accounts::remove_account(&account_id)
}

#[tauri::command]
pub fn set_active_account(account_id: String) -> Result<(), String> {
    accounts::set_active_id(&account_id)
}

#[tauri::command]
pub fn get_account(account_id: String) -> Result<String, String> {
    let info = accounts::get_account(&account_id)?;
    serde_json::to_string(&info).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn login_msa_pkce() -> Result<String, String> {
    let info = accounts::authenticate_msa_pkce().await?;
    serde_json::to_string(&info).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn start_msa_auth() -> Result<String, String> {
    let dc = accounts::start_msa_device_code().await?;
    serde_json::to_string(&dc).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn poll_msa_auth(device_code: String) -> Result<String, String> {
    let info = accounts::poll_msa_token(&device_code).await?;
    serde_json::to_string(&info).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn refresh_msa_account(account_id: String) -> Result<String, String> {
    let info = accounts::refresh_msa_account(&account_id).await?;
    serde_json::to_string(&info).map_err(|e| e.to_string())
}

// === Stubs for future phases ===

#[tauri::command]
pub async fn launch_instance(app: tauri::AppHandle, instance_id: String, account_id: String) -> Result<String, String> {
    let account_info = accounts::get_account(&account_id)?;
    let username = account_info.username;
    let uuid = account_info.uuid.unwrap_or_else(|| "00000000-0000-0000-0000-000000000000".to_string());

    let result = launch::launch_game(app, &instance_id, &username, &uuid).await?;
    serde_json::to_string(&result).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn stop_game() -> Result<(), String> {
    launch::stop_game()
}

#[tauri::command]
pub fn open_folder(path: String) -> Result<(), String> {
    // Resolve the path properly
    let expanded = if path.contains("%APPDATA%") {
        let appdata = dirs::data_local_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join("AppData").join("Roaming")))
            .ok_or("Cannot determine AppData")?;
        path.replace("%APPDATA%", &appdata.to_string_lossy())
    } else {
        path
    };
    let p = std::path::Path::new(&expanded);
    if !p.exists() {
        return Err(format!("Path does not exist: {}", expanded));
    }
    std::process::Command::new("explorer")
        .arg(&expanded)
        .spawn()
        .map_err(|e| format!("Failed to open folder: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn ensure_java(params_json: String) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct Params {
        mc_version: String,
    }
    let params: Params = serde_json::from_str(&params_json)
        .map_err(|e| format!("Invalid params: {}", e))?;
    let java_path = launch::ensure_java_for_version(&params.mc_version).await?;
    Ok(java_path)
}

#[tauri::command]
pub async fn search_mods(query: String, mc_version: String, loader: String, page: u32) -> Result<String, String> {
    let mc = if mc_version.is_empty() { None } else { Some(mc_version.as_str()) };
    let ld = if loader.is_empty() { None } else { Some(loader.as_str()) };
    let results = modplatform::search_modrinth(&query, mc, ld, page).await?;
    serde_json::to_string(&results).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_mod_versions(slug: String, mc_version: String, loader: String) -> Result<String, String> {
    let mc = if mc_version.is_empty() { None } else { Some(mc_version.as_str()) };
    let ld = if loader.is_empty() { None } else { Some(loader.as_str()) };
    let versions = modplatform::get_modrinth_versions(&slug, mc, ld).await?;
    serde_json::to_string(&versions).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn download_mod(url: String, instance_id: String, filename: String) -> Result<String, String> {
    let game_dir = std::path::PathBuf::from(format!(
        "{}/{}/.minecraft/mods",
        dirs::data_local_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join("AppData").join("Roaming")))
            .ok_or("Cannot determine AppData")?
            .join("MyL").join("instances").to_string_lossy(),
        instance_id
    ));
    let path = modplatform::download_mod(&url, &game_dir, &filename).await?;
    Ok(path)
}

#[tauri::command]
pub async fn get_mod_loaders(mc_version: String) -> Result<String, String> {
    let loaders = versions::get_mod_loaders(&mc_version).await?;
    serde_json::to_string(&loaders).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_mc_versions(type_filter: String) -> Result<String, String> {
    let versions = versions::get_mc_versions_filtered(&type_filter).await?;
    serde_json::to_string(&versions).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_instance(file_path: String) -> Result<String, String> {
    let result = import_mod::import_zip(&file_path).await?;
    serde_json::to_string(&result).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_shortcut(instance_id: String, target_path: String) -> Result<(), String> {
    let _ = (instance_id, target_path);
    todo!("Phase 9")
}

// === Native file dialog (fixes Tauri 2 path issue) ===

#[tauri::command]
pub async fn open_file_dialog(app: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    let path = app
        .dialog()
        .file()
        .add_filter("Архивы сборок", &["zip", "mrpack"])
        .blocking_pick_file();

    Ok(path.map(|p| p.to_string()))
}

// === Modpacks (Modrinth) ===

#[tauri::command]
pub async fn search_modpacks(query: String, loader: String, page: u32) -> Result<String, String> {
    let ld = if loader.is_empty() { None } else { Some(loader.as_str()) };
    let results = modplatform::search_modrinth_modpacks(&query, ld, page).await?;
    serde_json::to_string(&results).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_modpack_versions(slug: String) -> Result<String, String> {
    let versions = modplatform::get_modrinth_modpack_versions(&slug).await?;
    serde_json::to_string(&versions).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_modpack_from_url(url: String, filename: String, icon_url: Option<String>) -> Result<String, String> {
    // Download .mrpack to temp folder
    let tmp_path = modplatform::download_mrpack_to_temp(&url, &filename).await?;
    // Import using existing import logic
    let result = import_mod::import_zip(&tmp_path).await?;
    
    // Download and set icon if provided
    if let Some(i_url) = icon_url {
        if !i_url.is_empty() {
            if let Ok(instances_dir) = instances::get_instances_dir() {
                let instance_dir = instances_dir.join(&result.instance_id);
                if let Ok(resp) = reqwest::get(&i_url).await {
                    if let Ok(bytes) = resp.bytes().await {
                        let icon_path = instance_dir.join("icon.png");
                        let _ = std::fs::write(&icon_path, &bytes);
                        let _ = instances::update_instance_config(
                            &result.instance_id, None, None, None, Some(Some("icon.png".to_string())),
                            None, None, None, None, None, None, None, None
                        );
                    }
                }
            }
        }
    }
    
    // Clean up temp file
    let _ = std::fs::remove_file(&tmp_path);
    serde_json::to_string(&result).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_instance_icon(app: tauri::AppHandle, instance_id: String) -> Result<String, String> {
    use tauri_plugin_dialog::DialogExt;
    let path = app
        .dialog()
        .file()
        .add_filter("Изображения", &["png", "jpg", "jpeg"])
        .blocking_pick_file();

    if let Some(file_path) = path {
        let instances_dir = instances::get_instances_dir()?;
        let instance_dir = instances_dir.join(&instance_id);
        if !instance_dir.exists() {
            return Err("Instance not found".to_string());
        }
        let ext = std::path::Path::new(file_path.as_path().unwrap().to_str().unwrap()).extension().and_then(|e| e.to_str()).unwrap_or("png");
        let dest_name = format!("icon.{}", ext);
        let dest_path = instance_dir.join(&dest_name);
        
        std::fs::copy(file_path.as_path().unwrap().to_str().unwrap(), &dest_path).map_err(|e| format!("Copy error: {}", e))?;
        
        let _cfg = instances::update_instance_config(
            &instance_id, None, None, None, Some(Some(dest_name.clone())),
            None, None, None, None, None, None, None, None
        )?;
        
        // Return base64 of the new icon so frontend can update immediately
        if let Ok(bytes) = std::fs::read(&dest_path) {
            use base64::Engine;
            let mime = if ext.eq_ignore_ascii_case("jpg") || ext.eq_ignore_ascii_case("jpeg") { "image/jpeg" } else { "image/png" };
            let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
            return Ok(format!("data:{};base64,{}", mime, b64));
        }
        return Ok(dest_name);
    }
    Err("No file selected".to_string())
}

#[derive(serde::Serialize)]
pub struct ResourcePackInfo {
    pub name: String,
    pub filename: String,
    pub description: String,
    pub icon_base64: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ShaderPackInfo {
    pub name: String,
    pub filename: String,
}

#[derive(serde::Serialize)]
pub struct WorldInfo {
    pub name: String,
    pub folder_name: String,
    pub icon_base64: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ServerInfo {
    pub name: String,
    pub ip: String,
    pub icon_base64: Option<String>,
}

#[tauri::command]
pub fn list_resourcepacks(instance_id: String) -> Result<String, String> {
    use base64::Engine;
    let instances_dir = instances::get_instances_dir()?;
    let rp_dir = instances_dir.join(&instance_id).join(".minecraft").join("resourcepacks");
    let mut list = Vec::new();
    if !rp_dir.exists() {
        let _ = std::fs::create_dir_all(&rp_dir);
        return Ok(serde_json::to_string(&list).unwrap());
    }

    if let Ok(entries) = std::fs::read_dir(&rp_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let filename = entry.file_name().to_string_lossy().to_string();
            let mut name = filename.clone();
            let mut description = String::new();
            let mut icon_base64 = None;

            if path.is_file() && filename.ends_with(".zip") {
                if let Ok(file) = std::fs::File::open(&path) {
                    if let Ok(mut archive) = zip::ZipArchive::new(file) {
                        if let Ok(mut f) = archive.by_name("pack.png") {
                            let mut buf = Vec::new();
                            if std::io::Read::read_to_end(&mut f, &mut buf).is_ok() {
                                let b64 = base64::engine::general_purpose::STANDARD.encode(&buf);
                                icon_base64 = Some(format!("data:image/png;base64,{}", b64));
                            }
                        }
                        if let Ok(mut f) = archive.by_name("pack.mcmeta") {
                            let mut txt = String::new();
                            if std::io::Read::read_to_string(&mut f, &mut txt).is_ok() {
                                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&txt) {
                                    if let Some(desc) = v.get("pack").and_then(|p| p.get("description")) {
                                        description = desc.as_str().unwrap_or("").to_string();
                                    }
                                }
                            }
                        }
                    }
                }
            } else if path.is_dir() {
                let pack_png = path.join("pack.png");
                if pack_png.exists() {
                    if let Ok(bytes) = std::fs::read(&pack_png) {
                        let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
                        icon_base64 = Some(format!("data:image/png;base64,{}", b64));
                    }
                }
                let pack_mcmeta = path.join("pack.mcmeta");
                if pack_mcmeta.exists() {
                    if let Ok(txt) = std::fs::read_to_string(&pack_mcmeta) {
                        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&txt) {
                            if let Some(desc) = v.get("pack").and_then(|p| p.get("description")) {
                                description = desc.as_str().unwrap_or("").to_string();
                            }
                        }
                    }
                }
            } else {
                continue;
            }

            list.push(ResourcePackInfo {
                name,
                filename,
                description,
                icon_base64,
            });
        }
    }
    serde_json::to_string(&list).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_shaderpacks(instance_id: String) -> Result<String, String> {
    let instances_dir = instances::get_instances_dir()?;
    let sp_dir = instances_dir.join(&instance_id).join(".minecraft").join("shaderpacks");
    let mut list = Vec::new();
    if !sp_dir.exists() {
        let _ = std::fs::create_dir_all(&sp_dir);
        return Ok(serde_json::to_string(&list).unwrap());
    }

    if let Ok(entries) = std::fs::read_dir(&sp_dir) {
        for entry in entries.flatten() {
            let filename = entry.file_name().to_string_lossy().to_string();
            list.push(ShaderPackInfo {
                name: filename.clone(),
                filename,
            });
        }
    }
    serde_json::to_string(&list).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_worlds(instance_id: String) -> Result<String, String> {
    use base64::Engine;
    let instances_dir = instances::get_instances_dir()?;
    let saves_dir = instances_dir.join(&instance_id).join(".minecraft").join("saves");
    let mut list = Vec::new();
    if !saves_dir.exists() {
        let _ = std::fs::create_dir_all(&saves_dir);
        return Ok(serde_json::to_string(&list).unwrap());
    }

    if let Ok(entries) = std::fs::read_dir(&saves_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let folder_name = entry.file_name().to_string_lossy().to_string();
                let icon_file = path.join("icon.png");
                let mut icon_base64 = None;
                if icon_file.exists() {
                    if let Ok(bytes) = std::fs::read(&icon_file) {
                        let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
                        icon_base64 = Some(format!("data:image/png;base64,{}", b64));
                    }
                }
                list.push(WorldInfo {
                    name: folder_name.clone(),
                    folder_name,
                    icon_base64,
                });
            }
        }
    }
    serde_json::to_string(&list).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_world_zip(app: tauri::AppHandle, instance_id: String) -> Result<String, String> {
    use tauri_plugin_dialog::DialogExt;
    let path = app
        .dialog()
        .file()
        .add_filter("Архив мира", &["zip"])
        .blocking_pick_file();

    if let Some(file_path) = path {
        let instances_dir = instances::get_instances_dir()?;
        let saves_dir = instances_dir.join(&instance_id).join(".minecraft").join("saves");
        std::fs::create_dir_all(&saves_dir).ok();

        let clean_path = file_path.as_path().unwrap().to_str().unwrap();
        let file = std::fs::File::open(clean_path).map_err(|e| format!("Open error: {}", e))?;
        let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("Zip error: {}", e))?;

        let zip_stem = std::path::Path::new(clean_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("World");

        let dest_dir = saves_dir.join(zip_stem);
        std::fs::create_dir_all(&dest_dir).ok();

        for i in 0..archive.len() {
            if let Ok(mut file) = archive.by_index(i) {
                let out_path = match file.enclosed_name() {
                    Some(p) => dest_dir.join(p),
                    None => continue,
                };
                if file.is_dir() {
                    let _ = std::fs::create_dir_all(&out_path);
                } else {
                    if let Some(p) = out_path.parent() {
                        let _ = std::fs::create_dir_all(p);
                    }
                    let mut outfile = std::fs::File::create(&out_path).ok();
                    if let Some(ref mut out) = outfile {
                        let _ = std::io::copy(&mut file, out);
                    }
                }
            }
        }
        return Ok(zip_stem.to_string());
    }
    Err("No file selected".to_string())
}

fn read_nbt_string(cur: &mut &[u8]) -> Option<String> {
    if cur.len() < 2 { return None; }
    let len = u16::from_be_bytes([cur[0], cur[1]]) as usize;
    *cur = &cur[2..];
    if cur.len() < len { return None; }
    let s = String::from_utf8_lossy(&cur[..len]).to_string();
    *cur = &cur[len..];
    Some(s)
}

fn skip_nbt_value(tag_id: u8, cur: &mut &[u8]) {
    match tag_id {
        1 => if cur.len() >= 1 { *cur = &cur[1..]; },
        2 => if cur.len() >= 2 { *cur = &cur[2..]; },
        3 => if cur.len() >= 4 { *cur = &cur[4..]; },
        4 => if cur.len() >= 8 { *cur = &cur[8..]; },
        5 => if cur.len() >= 4 { *cur = &cur[4..]; },
        6 => if cur.len() >= 8 { *cur = &cur[8..]; },
        7 => {
            if cur.len() >= 4 {
                let len = i32::from_be_bytes([cur[0], cur[1], cur[2], cur[3]]) as usize;
                *cur = &cur[4..];
                if cur.len() >= len { *cur = &cur[len..]; }
            }
        },
        8 => { let _ = read_nbt_string(cur); },
        9 => {
            if cur.len() >= 5 {
                let elem_type = cur[0];
                let count = i32::from_be_bytes([cur[1], cur[2], cur[3], cur[4]]) as usize;
                *cur = &cur[5..];
                for _ in 0..count { skip_nbt_value(elem_type, cur); }
            }
        },
        10 => {
            while !cur.is_empty() {
                let t = cur[0];
                *cur = &cur[1..];
                if t == 0 { break; }
                let _ = read_nbt_string(cur);
                skip_nbt_value(t, cur);
            }
        },
        11 => {
            if cur.len() >= 4 {
                let len = i32::from_be_bytes([cur[0], cur[1], cur[2], cur[3]]) as usize * 4;
                *cur = &cur[4..];
                if cur.len() >= len { *cur = &cur[len..]; }
            }
        },
        12 => {
            if cur.len() >= 4 {
                let len = i32::from_be_bytes([cur[0], cur[1], cur[2], cur[3]]) as usize * 8;
                *cur = &cur[4..];
                if cur.len() >= len { *cur = &cur[len..]; }
            }
        },
        _ => {},
    }
}

fn parse_nbt_servers(data: &[u8]) -> Vec<ServerInfo> {
    use flate2::read::GzDecoder;
    use std::io::Read;

    let mut uncompressed = Vec::new();
    let mut gz = GzDecoder::new(data);
    if gz.read_to_end(&mut uncompressed).is_err() || uncompressed.is_empty() {
        uncompressed = data.to_vec();
    }

    let mut cur = &uncompressed[..];
    if cur.is_empty() || cur[0] != 10 { return Vec::new(); }
    cur = &cur[1..];
    let _ = read_nbt_string(&mut cur);

    let mut servers = Vec::new();
    while !cur.is_empty() {
        let tag_type = cur[0];
        cur = &cur[1..];
        if tag_type == 0 { break; }
        let name = read_nbt_string(&mut cur).unwrap_or_default();
        if tag_type == 9 && name == "servers" {
            if cur.len() >= 5 {
                let elem_type = cur[0];
                let count = i32::from_be_bytes([cur[1], cur[2], cur[3], cur[4]]) as usize;
                cur = &cur[5..];
                if elem_type == 10 {
                    for _ in 0..count {
                        let mut s_name = String::new();
                        let mut s_ip = String::new();
                        let mut s_icon = None;
                        while !cur.is_empty() {
                            let t = cur[0];
                            cur = &cur[1..];
                            if t == 0 { break; }
                            let fname = read_nbt_string(&mut cur).unwrap_or_default();
                            if t == 8 {
                                let val = read_nbt_string(&mut cur).unwrap_or_default();
                                if fname == "name" { s_name = val; }
                                else if fname == "ip" { s_ip = val; }
                                else if fname == "icon" {
                                    if val.starts_with("data:") { s_icon = Some(val); }
                                    else { s_icon = Some(format!("data:image/png;base64,{}", val)); }
                                }
                            } else {
                                skip_nbt_value(t, &mut cur);
                            }
                        }
                        if !s_name.is_empty() || !s_ip.is_empty() {
                            servers.push(ServerInfo { name: s_name, ip: s_ip, icon_base64: s_icon });
                        }
                    }
                }
            }
            break;
        } else {
            skip_nbt_value(tag_type, &mut cur);
        }
    }
    servers
}

#[tauri::command]
pub fn list_servers(instance_id: String) -> Result<String, String> {
    let instances_dir = instances::get_instances_dir()?;
    let s_file = instances_dir.join(&instance_id).join(".minecraft").join("servers.dat");
    if !s_file.exists() {
        return Ok(serde_json::to_string(&Vec::<ServerInfo>::new()).unwrap());
    }
    let data = std::fs::read(&s_file).map_err(|e| format!("Read error: {}", e))?;
    let servers = parse_nbt_servers(&data);
    serde_json::to_string(&servers).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_resourcepack(instance_id: String, filename: String) -> Result<(), String> {
    let instances_dir = instances::get_instances_dir()?;
    let target = instances_dir.join(&instance_id).join(".minecraft").join("resourcepacks").join(filename);
    if target.exists() {
        if target.is_dir() {
            std::fs::remove_dir_all(&target).map_err(|e| e.to_string())?;
        } else {
            std::fs::remove_file(&target).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn delete_shaderpack(instance_id: String, filename: String) -> Result<(), String> {
    let instances_dir = instances::get_instances_dir()?;
    let target = instances_dir.join(&instance_id).join(".minecraft").join("shaderpacks").join(filename);
    if target.exists() {
        if target.is_dir() {
            std::fs::remove_dir_all(&target).map_err(|e| e.to_string())?;
        } else {
            std::fs::remove_file(&target).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn delete_world(instance_id: String, folder_name: String) -> Result<(), String> {
    let instances_dir = instances::get_instances_dir()?;
    let target = instances_dir.join(&instance_id).join(".minecraft").join("saves").join(folder_name);
    if target.exists() {
        if target.is_dir() {
            std::fs::remove_dir_all(&target).map_err(|e| e.to_string())?;
        } else {
            std::fs::remove_file(&target).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
