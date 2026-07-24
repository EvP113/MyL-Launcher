use serde::{Deserialize, Serialize};

const MODRINTH_API: &str = "https://api.modrinth.com/v2";

// === Modrinth Search ===

#[derive(Debug, Serialize, Deserialize)]
pub struct ModrinthSearchResult {
    pub hits: Vec<ModrinthHit>,
    pub offset: u32,
    pub limit: u32,
    pub total_hits: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModrinthHit {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub project_type: String,
    pub downloads: u64,
    pub icon_url: Option<String>,
    pub categories: Option<Vec<String>>,
    pub versions: Option<Vec<String>>,
    pub author: String,
    pub date_modified: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModrinthProject {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: Option<String>,
    pub project_type: String,
    pub downloads: u64,
    pub icon_url: Option<String>,
    pub categories: Option<Vec<String>>,
    pub versions: Option<Vec<String>>,
    pub author: String,
    pub date_modified: String,
    pub server_side: Option<String>,
    pub client_side: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModrinthVersion {
    pub id: String,
    pub name: String,
    pub version_number: String,
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub files: Vec<ModrinthFile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModrinthFile {
    pub url: String,
    pub filename: String,
    pub size: u64,
    pub primary: bool,
}

// === Common Mod Info (for UI) ===

#[derive(Debug, Serialize, Deserialize)]
pub struct ModInfo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub source: String, // "modrinth" or "curseforge"
    pub downloads: u64,
    pub icon_url: Option<String>,
    pub author: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModVersionInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub mc_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub download_url: String,
    pub filename: String,
    pub size: u64,
}

// === Search Mods (Modrinth) ===

pub async fn search_modrinth(
    query: &str,
    mc_version: Option<&str>,
    loader: Option<&str>,
    page: u32,
) -> Result<Vec<ModInfo>, String> {
    let mut url = format!("{}/search?limit=20&offset={}", MODRINTH_API, page * 20);

    if !query.is_empty() {
        url = format!("{}&query={}", url, urlencoding::encode(query));
    }

    // Build facets
    let mut facets = Vec::new();
    facets.push(r#"["project_type:mod"]"#.to_string());
    if let Some(v) = mc_version {
        facets.push(format!(r#"["versions:{}"]"#, v));
    }
    if let Some(l) = loader {
        facets.push(format!(r#"["categories:{}"]"#, l));
    }
    if !facets.is_empty() {
        url = format!("{}&facets=[{}]", url, facets.join(","));
    }

    let resp = reqwest::get(&url).await.map_err(|e| format!("Modrinth API error: {}", e))?;
    let result: ModrinthSearchResult = resp.json().await.map_err(|e| format!("Parse error: {}", e))?;

    Ok(result.hits.into_iter().map(|hit| ModInfo {
        id: hit.slug,
        title: hit.title,
        description: hit.description,
        source: "modrinth".to_string(),
        downloads: hit.downloads,
        icon_url: hit.icon_url,
        author: hit.author,
    }).collect())
}

// === Get Mod Versions (Modrinth) ===

pub async fn get_modrinth_versions(
    slug: &str,
    mc_version: Option<&str>,
    loader: Option<&str>,
) -> Result<Vec<ModVersionInfo>, String> {
    let mut url = format!("{}/project/{}/version", MODRINTH_API, slug);
    let mut params = Vec::new();
    if let Some(v) = mc_version {
        params.push(format!("game_versions=[\"{}\"]", v));
    }
    if let Some(l) = loader {
        params.push(format!("loaders=[\"{}\"]", l));
    }
    if !params.is_empty() {
        url = format!("{}?{}", url, params.join("&"));
    }

    let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    let versions: Vec<ModrinthVersion> = resp.json().await.map_err(|e| e.to_string())?;

    Ok(versions.into_iter().map(|v| {
        let primary_file = v.files.iter().find(|f| f.primary).or(v.files.first());
        ModVersionInfo {
            id: v.id,
            name: v.name,
            version: v.version_number,
            mc_versions: v.game_versions,
            loaders: v.loaders,
            download_url: primary_file.map(|f| f.url.clone()).unwrap_or_default(),
            filename: primary_file.map(|f| f.filename.clone()).unwrap_or_default(),
            size: primary_file.map(|f| f.size).unwrap_or(0),
        }
    }).collect())
}

// === Download Mod File ===

pub async fn download_mod(
    url: &str,
    target_dir: &std::path::Path,
    filename: &str,
) -> Result<String, String> {
    let resp = reqwest::get(url).await.map_err(|e| format!("Download error: {}", e))?;
    let bytes = resp.bytes().await.map_err(|e| e.to_string())?;

    fs::create_dir_all(target_dir).map_err(|e| e.to_string())?;
    let path = target_dir.join(filename);
    fs::write(&path, &bytes).map_err(|e| e.to_string())?;

    Ok(path.to_string_lossy().to_string())
}

use std::fs;

// === Modpack Info (for UI) ===

#[derive(Debug, Serialize, Deserialize)]
pub struct ModpackInfo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub downloads: u64,
    pub follows: u64,
    pub icon_url: Option<String>,
    pub author: String,
    pub categories: Vec<String>,
    pub versions: Vec<String>, // supported mc versions
    pub date_modified: String,
}

// === Search Modpacks (Modrinth) ===

pub async fn search_modrinth_modpacks(
    query: &str,
    loader: Option<&str>,
    page: u32,
) -> Result<Vec<ModpackInfo>, String> {
    let mut url = format!("{}/search?limit=20&offset={}", MODRINTH_API, page * 20);

    if !query.is_empty() {
        url = format!("{}&query={}", url, urlencoding::encode(query));
    }

    let mut facets = vec![r#"["project_type:modpack"]"#.to_string()];
    if let Some(l) = loader {
        if !l.is_empty() {
            facets.push(format!(r#"["categories:{}"]"#, l));
        }
    }
    url = format!("{}&facets=[{}]", url, facets.join(","));

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("User-Agent", "MyL/1.0")
        .send()
        .await
        .map_err(|e| format!("Modrinth API error: {}", e))?;

    let result: ModrinthSearchResult = resp.json().await.map_err(|e| format!("Parse error: {}", e))?;

    Ok(result.hits.into_iter().map(|hit| ModpackInfo {
        id: hit.slug.clone(),
        title: hit.title,
        description: hit.description,
        downloads: hit.downloads,
        follows: 0,
        icon_url: hit.icon_url,
        author: hit.author,
        categories: hit.categories.unwrap_or_default(),
        versions: hit.versions.unwrap_or_default(),
        date_modified: hit.date_modified,
    }).collect())
}

// === Get Modpack Versions (Modrinth) ===
// Returns same ModVersionInfo format — the download_url points to .mrpack

pub async fn get_modrinth_modpack_versions(slug: &str) -> Result<Vec<ModVersionInfo>, String> {
    let url = format!("{}/project/{}/version", MODRINTH_API, slug);

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("User-Agent", "MyL/1.0")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let versions: Vec<ModrinthVersion> = resp.json().await.map_err(|e| e.to_string())?;

    Ok(versions.into_iter().map(|v| {
        let primary_file = v.files.iter().find(|f| f.primary).or(v.files.first());
        ModVersionInfo {
            id: v.id,
            name: v.name,
            version: v.version_number,
            mc_versions: v.game_versions,
            loaders: v.loaders,
            download_url: primary_file.map(|f| f.url.clone()).unwrap_or_default(),
            filename: primary_file.map(|f| f.filename.clone()).unwrap_or_default(),
            size: primary_file.map(|f| f.size).unwrap_or(0),
        }
    }).collect())
}

// === Download .mrpack to temp dir and return path ===

pub async fn download_mrpack_to_temp(url: &str, filename: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("User-Agent", "MyL/1.0")
        .send()
        .await
        .map_err(|e| format!("Download error: {}", e))?;

    let bytes = resp.bytes().await.map_err(|e| e.to_string())?;

    let tmp_dir = std::env::temp_dir().join("myl_import");
    fs::create_dir_all(&tmp_dir).map_err(|e| e.to_string())?;

    let safe_name = filename.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
    let out_path = tmp_dir.join(&safe_name);
    fs::write(&out_path, &bytes).map_err(|e| e.to_string())?;

    Ok(out_path.to_string_lossy().to_string())
}
