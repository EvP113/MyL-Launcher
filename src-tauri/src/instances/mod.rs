use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Config stored in each instance folder as instance.cfg (JSON)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceConfig {
    pub name: String,
    pub mc_version: String,
    #[serde(default)]
    pub loader: Option<String>,
    #[serde(default)]
    pub loader_version: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub jvm_args: Option<String>,
    #[serde(default)]
    pub min_memory_mb: Option<u32>,
    #[serde(default)]
    pub max_memory_mb: Option<u32>,
    #[serde(default)]
    pub game_width: Option<u32>,
    #[serde(default)]
    pub game_height: Option<u32>,
    #[serde(default)]
    pub fullscreen: Option<bool>,
    #[serde(default = "default_last_played")]
    pub last_played: Option<String>,
}

fn default_last_played() -> Option<String> {
    None
}

/// Summary returned to the frontend (subset of InstanceConfig + id)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceInfo {
    pub id: String,
    pub name: String,
    pub mc_version: String,
    pub loader: Option<String>,
    pub icon: Option<String>,
    pub group: Option<String>,
}

impl InstanceConfig {
    fn from_config(path: &PathBuf) -> Result<Self, String> {
        let data = fs::read_to_string(path).map_err(|e| format!("Failed to read {:?}: {}", path, e))?;
        serde_json::from_str(&data).map_err(|e| format!("Failed to parse {:?}: {}", path, e))
    }
}

/// Returns the root instances directory: %APPDATA%\MyL\instances
fn instances_root() -> Result<PathBuf, String> {
    let appdata = dirs::data_local_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("AppData").join("Roaming")))
        .ok_or_else(|| "Cannot determine AppData path".to_string())?;
    let root = appdata.join("MyL").join("instances");
    Ok(root)
}

/// Public accessor for instances directory
pub fn get_instances_dir() -> Result<PathBuf, String> {
    instances_root()
}

/// List all instances by scanning the instances directory
pub fn list_instances() -> Result<Vec<InstanceInfo>, String> {
    let root = instances_root()?;
    if !root.exists() {
        fs::create_dir_all(&root).map_err(|e| format!("Cannot create instances dir: {}", e))?;
        return Ok(vec![]);
    }

    let mut instances = Vec::new();
    let entries = fs::read_dir(&root).map_err(|e| format!("Cannot read instances dir: {}", e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let cfg_path = path.join("instance.cfg");
        if !cfg_path.exists() {
            continue;
        }
        match InstanceConfig::from_config(&cfg_path) {
            Ok(cfg) => {
                let id = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                instances.push(InstanceInfo {
                    id,
                    name: cfg.name,
                    mc_version: cfg.mc_version,
                    loader: cfg.loader,
                    icon: cfg.icon,
                    group: cfg.group,
                });
            }
            Err(e) => {
                eprintln!("Skipping instance {:?}: {}", path, e);
            }
        }
    }

    instances.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(instances)
}

/// Create a new instance directory with instance.cfg
pub fn create_instance(name: &str, mc_version: &str, loader: Option<&str>, loader_version: Option<&str>, group: Option<&str>) -> Result<InstanceInfo, String> {
    let root = instances_root()?;
    fs::create_dir_all(&root).map_err(|e| format!("Cannot create instances dir: {}", e))?;

    let base_name = name.trim();
    let clean_dir_name = base_name.replace(|c: char| c == '/' || c == '\\' || c == ':' || c == '*' || c == '?' || c == '"' || c == '<' || c == '>' || c == '|', "_");
    
    let mut final_name = base_name.to_string();
    let mut final_dir_name = clean_dir_name.clone();
    
    let mut counter = 1;
    while root.join(&final_dir_name).exists() {
        final_name = format!("{} ({})", base_name, counter);
        final_dir_name = format!("{}({})", clean_dir_name, counter);
        counter += 1;
    }

    let instance_dir = root.join(&final_dir_name);

    fs::create_dir_all(&instance_dir).map_err(|e| format!("Cannot create instance dir: {}", e))?;
    fs::create_dir_all(instance_dir.join(".minecraft").join("mods")).ok();
    fs::create_dir_all(instance_dir.join("mod-icons")).ok();

    let cfg = InstanceConfig {
        name: final_name,
        mc_version: mc_version.to_string(),
        loader: loader.map(|s| s.to_string()),
        loader_version: loader_version.map(|s| s.to_string()),
        icon: None,
        group: group.map(|s| s.to_string()),
        jvm_args: None,
        min_memory_mb: Some(1024),
        max_memory_mb: Some(4096),
        game_width: Some(854),
        game_height: Some(480),
        fullscreen: Some(false),
        last_played: None,
    };

    let cfg_json = serde_json::to_string_pretty(&cfg).map_err(|e| format!("Serialize error: {}", e))?;
    fs::write(instance_dir.join("instance.cfg"), cfg_json)
        .map_err(|e| format!("Cannot write instance.cfg: {}", e))?;

    Ok(InstanceInfo {
        id: final_dir_name,
        name: cfg.name,
        mc_version: cfg.mc_version,
        loader: cfg.loader,
        icon: cfg.icon,
        group: cfg.group,
    })
}

/// Delete an instance directory
pub fn delete_instance(id: &str) -> Result<(), String> {
    let root = instances_root()?;
    let instance_dir = root.join(id);
    if !instance_dir.exists() {
        return Err(format!("Instance '{}' not found", id));
    }
    fs::remove_dir_all(&instance_dir).map_err(|e| format!("Cannot delete instance: {}", e))
}

/// Get full config for an instance
pub fn get_instance_config(id: &str) -> Result<InstanceConfig, String> {
    let root = instances_root()?;
    let cfg_path = root.join(id).join("instance.cfg");
    InstanceConfig::from_config(&cfg_path)
}

/// Update config for an instance
pub fn update_instance_config(
    id: &str,
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
) -> Result<InstanceConfig, String> {
    let root = instances_root()?;
    let cfg_path = root.join(id).join("instance.cfg");
    let mut cfg = InstanceConfig::from_config(&cfg_path)?;

    if let Some(new_name) = name {
        if !new_name.trim().is_empty() {
            cfg.name = new_name.trim().to_string();
        }
    }
    if let Some(new_group) = group {
        cfg.group = new_group.map(|g| g.trim().to_string()).filter(|g| !g.is_empty());
    }
    if let Some(new_jvm_args) = jvm_args {
        cfg.jvm_args = new_jvm_args.map(|a| a.trim().to_string()).filter(|a| !a.is_empty());
    }
    if let Some(new_icon) = icon {
        cfg.icon = new_icon.map(|i| i.trim().to_string()).filter(|i| !i.is_empty());
    }
    if let Some(new_mc) = mc_version {
        if !new_mc.trim().is_empty() {
            cfg.mc_version = new_mc.trim().to_string();
        }
    }
    if let Some(new_loader) = loader {
        cfg.loader = new_loader.map(|l| l.trim().to_string()).filter(|l| !l.is_empty());
    }
    if let Some(new_loader_ver) = loader_version {
        cfg.loader_version = new_loader_ver.map(|l| l.trim().to_string()).filter(|l| !l.is_empty());
    }
    if let Some(v) = min_memory_mb { cfg.min_memory_mb = v; }
    if let Some(v) = max_memory_mb { cfg.max_memory_mb = v; }
    if let Some(v) = game_width { cfg.game_width = v; }
    if let Some(v) = game_height { cfg.game_height = v; }
    if let Some(v) = fullscreen { cfg.fullscreen = v; }

    let cfg_json = serde_json::to_string_pretty(&cfg).map_err(|e| format!("Serialize error: {}", e))?;
    fs::write(&cfg_path, cfg_json).map_err(|e| format!("Cannot write instance.cfg: {}", e))?;
    Ok(cfg)
}

