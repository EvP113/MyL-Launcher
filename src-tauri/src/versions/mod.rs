use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const VERSION_MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json";
// Prism Launcher's public meta mirror aggregates full version lists for every loader with a
// `requires: [{uid: "...", equals: "<value>"}]` pin per entry — data the loaders' own official
// APIs either don't expose at all (Forge's promotions_slim.json only has "latest"/"recommended")
// or expose inconsistently (Quilt's endpoint has no "stable" flag, 404s instead of `[]` for an
// unsupported MC version, and returns builds in a non-chronological order). Prism's mirror gives
// all four loaders (Fabric/Quilt/Forge/NeoForge) the same shape, so one generic pipeline below
// (fetch_prism_component + filter_prism_versions_for_mc) handles all of them uniformly.
// We only consume this as a JSON data source (same as the Mojang endpoint below),
// not any of Prism's own (GPL-3.0) source code.
const PRISM_META_BASE: &str = "https://meta.prismlauncher.org/v1";

// === Mojang Version Manifest ===

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionManifest {
    pub latest: LatestVersions,
    pub versions: Vec<VersionEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LatestVersions {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionEntry {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
}

// === Combined result for frontend ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoaderVersion {
    pub version: String,
    pub stable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModLoaderInfo {
    pub name: String,
    pub versions: Vec<LoaderVersion>,
}

/// Sort key for a dotted version string: numeric segments compared component-wise,
/// with a plain release (no "-suffix") ranked above a pre-release of the same numbers
/// (e.g. "0.24.0" > "0.24.0-beta.9"). Used to normalize loader lists into newest-first
/// order regardless of what order the upstream API happened to return them in.
fn version_sort_key(v: &str) -> (Vec<u64>, u8) {
    let base = v.split('+').next().unwrap_or(v);
    let (core, has_pre) = match base.find('-') {
        Some(i) => (&base[..i], true),
        None => (base, false),
    };
    let nums: Vec<u64> = core
        .split('.')
        .map(|p| {
            p.chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
                .unwrap_or(0)
        })
        .collect();
    (nums, if has_pre { 0 } else { 1 })
}

fn sort_versions_desc(versions: &mut Vec<LoaderVersion>) {
    versions.sort_by(|a, b| version_sort_key(&b.version).cmp(&version_sort_key(&a.version)));
}

// === Cache ===

fn cache_dir() -> Result<PathBuf, String> {
    let appdata = dirs::data_local_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("AppData").join("Roaming")))
        .ok_or_else(|| "Cannot determine AppData path".to_string())?;
    let dir = appdata.join("MyL").join("meta");
    fs::create_dir_all(&dir).map_err(|e| format!("Cannot create meta dir: {}", e))?;
    Ok(dir)
}

fn is_cache_fresh(path: &PathBuf, max_age_secs: u64) -> bool {
    if let Ok(meta) = fs::metadata(path) {
        if let Ok(modified) = meta.modified() {
            if let Ok(elapsed) = modified.elapsed() {
                return elapsed.as_secs() < max_age_secs;
            }
        }
    }
    false
}

// === Public API ===

/// Fetch Minecraft version manifest (with 1h cache)
pub async fn get_mc_versions() -> Result<VersionManifest, String> {
    let cache_path = cache_dir()?.join("version_manifest.json");

    if is_cache_fresh(&cache_path, 3600) {
        let data = fs::read_to_string(&cache_path)
            .map_err(|e| format!("Cache read error: {}", e))?;
        return serde_json::from_str(&data).map_err(|e| format!("Cache parse error: {}", e));
    }

    let resp = reqwest::get(VERSION_MANIFEST_URL)
        .await
        .map_err(|e| format!("HTTP error: {}", e))?;
    let text = resp.text().await.map_err(|e| format!("Read error: {}", e))?;

    // Validate it's valid JSON
    let _: VersionManifest = serde_json::from_str(&text)
        .map_err(|e| format!("Parse error: {}", e))?;

    // Save to cache
    let _ = fs::write(&cache_path, &text);

    serde_json::from_str(&text).map_err(|e| e.to_string())
}

/// Get Minecraft versions filtered by type
pub async fn get_mc_versions_filtered(type_filter: &str) -> Result<Vec<VersionEntry>, String> {
    let manifest = get_mc_versions().await?;
    let types: Vec<&str> = type_filter.split(',').collect();
    let show_all = types.contains(&"all") || types.contains(&"none");
    let filtered: Vec<VersionEntry> = manifest.versions.into_iter()
        .filter(|v| show_all || types.contains(&v.version_type.as_str()))
        .collect();
    Ok(filtered)
}

/// Fetch a Prism Launcher meta component's full version list (with 1h disk cache — these
/// files list every build the loader ever shipped, so we cache the raw JSON and re-filter
/// it locally per Minecraft version rather than hitting the network on every request).
async fn fetch_prism_component(uid: &str) -> Result<serde_json::Value, String> {
    let cache_path = cache_dir()?.join(format!("prism_{}.json", uid.replace('.', "_")));

    if is_cache_fresh(&cache_path, 3600) {
        if let Ok(data) = fs::read_to_string(&cache_path) {
            if let Ok(json) = serde_json::from_str(&data) {
                return Ok(json);
            }
        }
    }

    let url = format!("{}/{}/", PRISM_META_BASE, uid);
    let resp = reqwest::get(&url).await.map_err(|e| format!("HTTP error: {}", e))?;
    let text = resp.text().await.map_err(|e| format!("Read error: {}", e))?;
    let json: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| format!("Parse error: {}", e))?;
    let _ = fs::write(&cache_path, &text);
    Ok(json)
}

/// Extract this component's versions that apply to `mc_version`. An entry counts as applicable
/// if it either has no hard `net.minecraft` requirement (loader is version-agnostic, e.g. Fabric)
/// or its requirement's `equals` matches exactly (e.g. every Forge/NeoForge build pins one).
fn filter_prism_versions_for_mc(data: &serde_json::Value, mc_version: &str) -> Vec<LoaderVersion> {
    let mut versions: Vec<LoaderVersion> = data
        .get("versions")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter(|entry| {
                    let mc_requirement = entry
                        .get("requires")
                        .and_then(|r| r.as_array())
                        .and_then(|reqs| reqs.iter().find(|r| r.get("uid").and_then(|u| u.as_str()) == Some("net.minecraft")));
                    match mc_requirement {
                        Some(req) => req.get("equals").and_then(|e| e.as_str()) == Some(mc_version),
                        None => true,
                    }
                })
                .filter_map(|entry| {
                    let version = entry.get("version")?.as_str()?.to_string();
                    let stable = entry.get("recommended").and_then(|s| s.as_bool()).unwrap_or(false);
                    Some(LoaderVersion { version, stable })
                })
                .collect()
        })
        .unwrap_or_default();
    sort_versions_desc(&mut versions);
    versions
}

/// Get Forge loader versions for specific MC version
pub async fn get_forge_versions(mc_version: &str) -> Result<Vec<LoaderVersion>, String> {
    let data = fetch_prism_component("net.minecraftforge").await?;
    Ok(filter_prism_versions_for_mc(&data, mc_version))
}

/// Get NeoForge loader versions for specific MC version
pub async fn get_neoforge_versions(mc_version: &str) -> Result<Vec<LoaderVersion>, String> {
    let data = fetch_prism_component("net.neoforged").await?;
    Ok(filter_prism_versions_for_mc(&data, mc_version))
}

/// True if this Prism meta component has an entry pinned to exactly `mc_version` via a
/// `requires: [{uid: "net.minecraft", equals: "<mc_version>"}]` entry.
fn prism_supports_mc_version(data: &serde_json::Value, mc_version: &str) -> bool {
    data.get("versions")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter().any(|entry| {
                entry
                    .get("requires")
                    .and_then(|r| r.as_array())
                    .map(|reqs| {
                        reqs.iter().any(|r| {
                            r.get("uid").and_then(|u| u.as_str()) == Some("net.minecraft")
                                && r.get("equals").and_then(|e| e.as_str()) == Some(mc_version)
                        })
                    })
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false)
}

/// Fabric and Quilt loader builds are themselves version-agnostic (they only require an
/// "intermediary" mapping layer, not a specific `net.minecraft`), so the loader lists alone
/// can't tell us whether a Minecraft version is actually supported. The intermediary mapping
/// component IS pinned per MC version and is shared by both loaders — check it once to gate.
async fn get_fabric_family_versions(mc_version: &str, loader_uid: &str) -> Result<Vec<LoaderVersion>, String> {
    let intermediary = fetch_prism_component("net.fabricmc.intermediary").await?;
    if !prism_supports_mc_version(&intermediary, mc_version) {
        return Ok(vec![]);
    }
    let data = fetch_prism_component(loader_uid).await?;
    Ok(filter_prism_versions_for_mc(&data, mc_version))
}

/// Get Fabric loader versions for specific MC version
pub async fn get_fabric_versions(mc_version: &str) -> Result<Vec<LoaderVersion>, String> {
    get_fabric_family_versions(mc_version, "net.fabricmc.fabric-loader").await
}

/// Get Quilt loader versions for specific MC version
pub async fn get_quilt_versions(mc_version: &str) -> Result<Vec<LoaderVersion>, String> {
    get_fabric_family_versions(mc_version, "org.quiltmc.quilt-loader").await
}

/// Get all available mod loaders filtered by Minecraft version
pub async fn get_mod_loaders(mc_version: &str) -> Result<Vec<ModLoaderInfo>, String> {
    let mut loaders = Vec::new();

    // Fabric
    match get_fabric_versions(mc_version).await {
        Ok(versions) if !versions.is_empty() => {
            loaders.push(ModLoaderInfo {
                name: "Fabric".to_string(),
                versions,
            });
        }
        _ => {}
    }

    // Quilt
    match get_quilt_versions(mc_version).await {
        Ok(versions) if !versions.is_empty() => {
            loaders.push(ModLoaderInfo {
                name: "Quilt".to_string(),
                versions,
            });
        }
        _ => {}
    }

    // Forge
    match get_forge_versions(mc_version).await {
        Ok(versions) if !versions.is_empty() => {
            loaders.push(ModLoaderInfo {
                name: "Forge".to_string(),
                versions,
            });
        }
        _ => {}
    }

    // NeoForge
    match get_neoforge_versions(mc_version).await {
        Ok(versions) if !versions.is_empty() => {
            loaders.push(ModLoaderInfo {
                name: "NeoForge".to_string(),
                versions,
            });
        }
        _ => {}
    }

    Ok(loaders)
}
