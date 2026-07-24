use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::Emitter;

use crate::instances;

static GAME_PROCESS: Mutex<Option<Child>> = Mutex::new(None);

// === Version manifest structures ===

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct VersionJson {
    #[serde(rename = "mainClass")]
    pub main_class: Option<String>,
    pub arguments: Option<VersionArguments>,
    pub libraries: Option<Vec<Library>>,
    #[serde(rename = "assetIndex")]
    pub asset_index: Option<AssetIndex>,
    pub assets: Option<String>,
    #[serde(rename = "minecraftArguments")]
    pub minecraft_arguments: Option<String>,
    pub downloads: Option<VersionDownloads>,
    #[serde(rename = "javaVersion")]
    pub java_version: Option<JavaVersionInfo>,
}

/// e.g. {"component": "java-runtime-epsilon", "majorVersion": 25}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaVersionInfo {
    pub component: String,
    #[serde(rename = "majorVersion")]
    pub major_version: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionDownloads {
    pub client: Option<ClientDownload>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientDownload {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionArguments {
    pub game: Option<Vec<serde_json::Value>>,
    pub jvm: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Library {
    pub name: String,
    pub url: Option<String>,
    #[serde(rename = "downloadUrl")]
    pub download_url: Option<String>,
    pub rules: Option<Vec<Rule>>,
    pub natives: Option<serde_json::Value>,
    #[serde(rename = "extract")]
    pub Extract: Option<ExtractInfo>,
    pub downloads: Option<LibraryDownloads>,
    /// Not from JSON. Marks a library that is only needed on disk for the Forge installer's
    /// processor step (installertools, srgutils, etc.), NOT on the game's runtime classpath.
    /// Keeping these off the classpath avoids module-path conflicts at launch.
    #[serde(default, skip_serializing)]
    pub processor_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryDownloads {
    pub artifact: Option<LibraryArtifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryArtifact {
    pub path: Option<String>,
    pub url: String,
    pub sha1: Option<String>,
    pub size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub action: String,
    pub os: Option<RuleOs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleOs {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractInfo {
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetIndex {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetIndexJson {
    pub objects: std::collections::HashMap<String, AssetObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetObject {
    pub hash: String,
    pub size: u64,
}

#[derive(Debug, Serialize)]
pub struct LaunchResult {
    pub success: bool,
    pub message: String,
    pub pid: Option<u32>,
}

// === Helpers ===

fn mojang_meta_dir() -> Result<PathBuf, String> {
    let appdata = dirs::data_local_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("AppData").join("Roaming")))
        .ok_or("Cannot determine AppData")?;
    Ok(appdata.join("MyL").join("meta"))
}

fn instances_root() -> Result<PathBuf, String> {
    let appdata = dirs::data_local_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("AppData").join("Roaming")))
        .ok_or("Cannot determine AppData")?;
    Ok(appdata.join("MyL").join("instances"))
}

fn maven_to_path(name: &str) -> String {
    let parts: Vec<&str> = name.splitn(3, ':').collect();
    if parts.len() < 3 { return name.to_string(); }
    let group = parts[0].replace('.', "/");
    let artifact = parts[1];
    let version = parts[2];
    format!("{}/{}/{}/{}-{}.jar", group, artifact, version, artifact, version)
}

/// Dedup key for a library: "group:artifact" (+ ":classifier" when present), WITHOUT the version.
/// Lets us collapse e.g. Mojang's guava 33.3.1 and Forge's guava 32.1.2 to one entry, while keeping
/// the main jar distinct from its `natives-windows` classifier sibling.
fn ga_key(name: &str) -> String {
    let parts: Vec<&str> = name.split(':').collect();
    match parts.len() {
        n if n >= 4 => format!("{}:{}:{}", parts[0], parts[1], parts[3]),
        3 => format!("{}:{}", parts[0], parts[1]),
        _ => name.to_string(),
    }
}

fn rules_satisfied(rules: &Option<Vec<Rule>>) -> bool {
    match rules {
        None => true,
        Some(rules) => {
            let mut allowed = true;
            for rule in rules {
                match rule.action.as_str() {
                    "allow" => {
                        if let Some(ref os) = rule.os {
                            if let Some(ref name) = os.name {
                                if name != "windows" { allowed = false; }
                            }
                        }
                    }
                    "disallow" => {
                        if let Some(ref os) = rule.os {
                            if let Some(ref name) = os.name {
                                if name == "windows" { allowed = false; }
                            }
                        }
                    }
                    _ => {}
                }
            }
            allowed
        }
    }
}

/// Extract ALL string values from a JSON argument entry (can be a plain string, or an
/// object with "value" being either a single string or an array of strings, e.g.
/// {"value": ["--width", "${resolution_width}", "--height", "${resolution_height}"]}).
/// Returning only the Vec's first element (as the old code did) silently drops the rest
/// of a multi-part flag, which shifts every argument after it out of position.
fn extract_arg_values(val: &serde_json::Value) -> Vec<String> {
    match val {
        serde_json::Value::String(s) => vec![s.clone()],
        serde_json::Value::Object(map) => match map.get("value") {
            Some(serde_json::Value::String(v)) => vec![v.clone()],
            Some(serde_json::Value::Array(arr)) => {
                arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()
            }
            _ => vec![],
        },
        _ => vec![],
    }
}

/// Whether a manifest argument's conditional "rules" are satisfied by this launcher.
///
/// Mojang semantics: no `rules` field => the entry is always included. If `rules` IS
/// present, the entry is EXCLUDED by default, and each rule that matches the current
/// environment (os name + any required `features`) sets allow/disallow, with the last
/// matching rule winning. This launcher doesn't implement demo mode, custom resolution,
/// or quick-play, so any rule with a non-empty `features` requirement never matches —
/// which correctly keeps args like --demo / --width / --quickPlayPath out entirely,
/// instead of the old code including them unconditionally (since it only ever looked at
/// "os" and defaulted to "allowed = true").
fn arg_rules_allowed(arg: &serde_json::Value) -> bool {
    let rules = match arg.get("rules").and_then(|v| v.as_array()) {
        Some(r) => r,
        None => return true,
    };

    let mut allowed = false;
    for rule in rules {
        let os_ok = match rule.get("os").and_then(|v| v.as_object()) {
            Some(os) => os.get("name").and_then(|v| v.as_str()).map_or(true, |n| n == "windows"),
            None => true,
        };
        let features_ok = match rule.get("features").and_then(|v| v.as_object()) {
            Some(features) => features.is_empty(),
            None => true,
        };
        if os_ok && features_ok {
            let action = rule.get("action").and_then(|v| v.as_str()).unwrap_or("allow");
            allowed = action == "allow";
        }
    }
    allowed
}

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::Semaphore;

/// Build classpath, downloading missing libraries in parallel.
///
/// Two things happen here beyond plain downloading:
///  * `processor_only` libraries (from Forge's install_profile.json) are downloaded to disk but
///    kept OFF the classpath — they are tools the installer's processors need, not runtime deps.
///  * Runtime libraries are deduplicated by `group:artifact[:classifier]` keeping the LAST match,
///    so a loader's copy of a shared dependency (e.g. Forge guava 32.1.2 / failureaccess 1.0.1)
///    wins over Mojang's (guava 33.3.1 / failureaccess 1.0.2). Two versions of the same module on
///    the module path is what triggered `java.lang.module.ResolutionException` (split package).
async fn build_classpath(app: &AppHandle, libraries: &[Library], libraries_dir: &Path) -> String {
    if libraries.is_empty() { return String::new(); }

    // Split into runtime (deduped, order-preserving) vs processor-only (download but don't add to cp).
    let mut order: Vec<String> = Vec::new();
    let mut cp_map: std::collections::HashMap<String, Library> = std::collections::HashMap::new();
    let mut proc_libs: Vec<Library> = Vec::new();
    for lib in libraries.iter().filter(|lib| rules_satisfied(&lib.rules)) {
        if lib.processor_only {
            proc_libs.push(lib.clone());
            continue;
        }
        let key = ga_key(&lib.name);
        if !cp_map.contains_key(&key) { order.push(key.clone()); }
        cp_map.insert(key, lib.clone()); // last occurrence wins
    }
    let cp_libs: Vec<Library> = order.iter().filter_map(|k| cp_map.get(k).cloned()).collect();

    let client = reqwest::Client::new();
    let semaphore = Arc::new(Semaphore::new(16)); // 16 concurrent downloads
    let done_counter = Arc::new(AtomicUsize::new(0));
    let total = cp_libs.len() + proc_libs.len();

    // Resolve a library's on-disk path + download URL, spawning a fetch task if the jar is missing.
    let spawn_dl = |lib: &Library| -> (PathBuf, Option<tokio::task::JoinHandle<()>>) {
        let (rel_path, download_url) = match lib.downloads.as_ref().and_then(|d| d.artifact.as_ref()) {
            Some(artifact) => (
                artifact.path.clone().unwrap_or_else(|| maven_to_path(&lib.name)),
                Some(artifact.url.clone()),
            ),
            None => {
                let path = maven_to_path(&lib.name);
                let url = lib.download_url.clone()
                    .or_else(|| lib.url.as_ref().map(|base| format!("{}/{}", base.trim_end_matches('/'), path)))
                    .or_else(|| Some(format!("https://libraries.minecraft.net/{}", path)));
                (path, url)
            }
        };

        let jar_path = libraries_dir.join(&rel_path);
        if jar_path.exists() { return (jar_path, None); }
        let url = match download_url {
            Some(u) if !u.is_empty() => u,
            _ => return (jar_path, None),
        };

        let sem = semaphore.clone();
        let client_clone = client.clone();
        let target_path = jar_path.clone();
        let app_clone = app.clone();
        let done_clone = done_counter.clone();
        let total = total;

        let task = tokio::spawn(async move {
            let _permit = sem.acquire().await;
            if let Some(parent) = target_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(resp) = client_clone.get(&url).send().await {
                if resp.status().is_success() {
                    if let Ok(bytes) = resp.bytes().await {
                        let _ = fs::write(&target_path, &bytes);
                    }
                } else {
                    // Fallback for Forge/NeoForge variant JARs (-universal, -client, -installer)
                    let alt_urls = vec![
                        url.replace(".jar", "-universal.jar"),
                        url.replace(".jar", "-client.jar"),
                        url.replace(".jar", "-installer.jar"),
                    ];
                    for alt_url in alt_urls {
                        if let Ok(resp2) = client_clone.get(&alt_url).send().await {
                            if resp2.status().is_success() {
                                if let Ok(bytes2) = resp2.bytes().await {
                                    let _ = fs::write(&target_path, &bytes2);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            let current = done_clone.fetch_add(1, Ordering::Relaxed) + 1;
            if current % 5 == 0 || current == total {
                let _ = app_clone.emit("download-progress", serde_json::json!({
                    "task_id": "libraries",
                    "done": current,
                    "total": total
                }));
            }
        });
        (jar_path, Some(task))
    };

    // Spawn all downloads (both sets run concurrently, capped by the semaphore).
    let cp_tasks: Vec<(PathBuf, Option<tokio::task::JoinHandle<()>>)> =
        cp_libs.iter().map(|l| spawn_dl(l)).collect();
    let proc_tasks: Vec<(PathBuf, Option<tokio::task::JoinHandle<()>>)> =
        proc_libs.iter().map(|l| spawn_dl(l)).collect();

    // Processor libs: just wait for them to land on disk; they never go on the classpath.
    for (_jar, task) in proc_tasks {
        if let Some(t) = task { let _ = t.await; }
    }

    // Runtime libs: wait, then add existing jars to the classpath in dedup order.
    let mut entries: Vec<String> = Vec::new();
    for (jar_path, task) in cp_tasks {
        if let Some(t) = task { let _ = t.await; }
        if jar_path.exists() {
            entries.push(jar_path.to_string_lossy().to_string());
        }
    }

    entries.join(";")
}

/// Extract native libraries from JARs that have "natives" field
async fn extract_natives(libraries: &[Library], libraries_dir: &Path, natives_dir: &Path) {
    let _ = fs::create_dir_all(natives_dir);

    for lib in libraries.iter().filter(|lib| rules_satisfied(&lib.rules)) {
        if lib.natives.is_none() { continue; }

        // Get the native classifier for windows
        let classifier = match &lib.natives {
            Some(serde_json::Value::Object(map)) => {
                map.get("windows").and_then(|v| v.as_str()).unwrap_or("natives-windows")
            }
            _ => "natives-windows",
        };

        // Find the classifier JAR in libraries
        let name_parts: Vec<&str> = lib.name.splitn(3, ':').collect();
        if name_parts.len() < 3 { continue; }

        let classifier_jar = format!("{}-{}-{}.jar", name_parts[1], name_parts[2], classifier);
        let group_path = name_parts[0].replace('.', "/");
        let jar_rel = format!("{}/{}/{}/{}", group_path, name_parts[1], name_parts[2], classifier_jar);
        let jar_path = libraries_dir.join(&jar_rel);

        if !jar_path.exists() {
            // Try downloading the classifier JAR
            let base_url = lib.download_url.clone()
                .or_else(|| lib.url.as_ref().map(|u| format!("{}/{}", u.trim_end_matches('/'), jar_rel)))
                .or_else(|| Some(format!("https://libraries.minecraft.net/{}", jar_rel)));

            if let Some(url) = base_url {
                if let Some(parent) = jar_path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                eprintln!("Downloading native: {} -> {}", lib.name, url);
                if let Ok(resp) = reqwest::get(&url).await {
                    if let Ok(bytes) = resp.bytes().await {
                        let _ = fs::write(&jar_path, &bytes);
                    }
                }
            }
        }

        if jar_path.exists() {
            // Extract the native JAR into natives directory
            eprintln!("Extracting native: {:?}", jar_path);
            if let Ok(zip_file) = fs::File::open(&jar_path) {
                if let Ok(mut archive) = zip::ZipArchive::new(zip_file) {
                    for i in 0..archive.len() {
                        if let Ok(mut file) = archive.by_index(i) {
                            let name = file.name().to_string();
                            // Skip META-INF and excluded files
                            if name.starts_with("META-INF/") { continue; }
                            if let Some(ref excl) = lib.Extract {
                                if let Some(ref excludes) = excl.exclude {
                                    if excludes.iter().any(|e| name.contains(e.as_str())) { continue; }
                                }
                            }
                            let out_path = natives_dir.join(&name);
                            if let Some(parent) = out_path.parent() {
                                let _ = fs::create_dir_all(parent);
                            }
                            let _ = std::io::copy(&mut file, &mut fs::File::create(&out_path).unwrap_or_else(|_| fs::File::create(natives_dir.join("dummy")).unwrap()));
                        }
                    }
                }
            }
        }
    }
}

/// Download asset index and missing asset objects in parallel (multi-threaded pool)
async fn download_assets(app: &AppHandle, asset_index: &AssetIndex, game_dir: &Path) {
    let assets_dir = game_dir.join("assets");
    let objects_dir = assets_dir.join("objects");
    let index_dir = assets_dir.join("indexes");
    let _ = fs::create_dir_all(&index_dir);
    let _ = fs::create_dir_all(&objects_dir);

    // Download asset index JSON
    let index_path = index_dir.join(format!("{}.json", asset_index.id));
    if !index_path.exists() {
        eprintln!("[Assets] Downloading asset index: {}", asset_index.id);
        if let Ok(resp) = reqwest::get(&asset_index.url).await {
            if let Ok(bytes) = resp.bytes().await {
                let _ = fs::write(&index_path, &bytes);
            }
        }
    }

    let data = match fs::read_to_string(&index_path) {
        Ok(d) => d,
        Err(_) => return,
    };

    let index: AssetIndexJson = match serde_json::from_str(&data) {
        Ok(i) => i,
        Err(_) => return,
    };

    let total = index.objects.len();
    if total == 0 { return; }

    eprintln!("[Assets] Starting parallel multi-threaded download of {} assets...", total);

    let client = reqwest::Client::new();
    let semaphore = Arc::new(Semaphore::new(24)); // 24 concurrent worker threads
    let done_counter = Arc::new(AtomicUsize::new(0));

    let mut tasks = Vec::new();

    for (_name, obj) in index.objects {
        let hash = obj.hash.clone();
        if hash.len() < 2 { continue; }
        let hash_prefix = hash[..2].to_string();
        let obj_path = objects_dir.join(&hash_prefix).join(&hash);

        if obj_path.exists() {
            done_counter.fetch_add(1, Ordering::Relaxed);
            continue;
        }

        let sem = semaphore.clone();
        let client_clone = client.clone();
        let app_clone = app.clone();
        let done_clone = done_counter.clone();

        let task = tokio::spawn(async move {
            let _permit = sem.acquire().await;
            if let Some(parent) = obj_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let url = format!("https://resources.download.minecraft.net/{}/{}", hash_prefix, hash);
            if let Ok(resp) = client_clone.get(&url).send().await {
                if let Ok(bytes) = resp.bytes().await {
                    let _ = fs::write(&obj_path, &bytes);
                }
            }
            let current = done_clone.fetch_add(1, Ordering::Relaxed) + 1;
            if current % 30 == 0 || current == total {
                let _ = app_clone.emit("download-progress", serde_json::json!({
                    "task_id": "assets",
                    "done": current,
                    "total": total
                }));
            }
        });
        tasks.push(task);
    }

    for task in tasks {
        let _ = task.await;
    }

    let final_done = done_counter.load(Ordering::Relaxed);
    eprintln!("[Assets] Parallel assets check complete: {} / {}", final_done, total);
}

/// Build a launch profile from the bytes of a Forge/NeoForge *installer* jar.
///
/// Both Forge (1.13+) and NeoForge ship their runtime manifest INSIDE the installer jar as
/// `version.json` (runtime libs + `BootstrapLauncher` mainClass + the `--fml.*Version` args),
/// alongside `install_profile.json` (the processor tool libs). Neither publishes a standalone
/// version.json on Maven, so we extract both here and assemble a ForgeWrapper-driven profile:
///  * inject ForgeWrapper (its `Main` runs the installer's processors at launch, then delegates
///    to BootstrapLauncher — and 1.6.0 auto-detects NeoForge via the `--fml.neoForgeVersion` arg
///    that's already present in NeoForge's version.json);
///  * append install_profile's processor libs as `processor_only` (downloaded, kept off classpath);
///  * force mainClass to ForgeWrapper's `Main`.
fn profile_from_installer(bytes: &[u8], loader_tag: &str) -> Option<VersionJson> {
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes)).ok()?;

    // Read both entries into owned Strings first, so each mutable borrow of `archive` is
    // released before the next `by_name` call.
    let mut version_contents = String::new();
    if let Ok(mut file) = archive.by_name("version.json") {
        let _ = std::io::Read::read_to_string(&mut file, &mut version_contents);
    }
    let mut install_contents = String::new();
    if let Ok(mut file) = archive.by_name("install_profile.json") {
        let _ = std::io::Read::read_to_string(&mut file, &mut install_contents);
    }

    let mut profile = serde_json::from_str::<VersionJson>(&version_contents).ok()?;
    eprintln!("[{}] Extracted version.json from installer. Runtime libraries: {}", loader_tag,
        profile.libraries.as_ref().map(|l| l.len()).unwrap_or(0));

    // ForgeWrapper is NOT on Maven Central — only a GitHub release asset (capital F filename).
    let fw_lib = Library {
        name: "io.github.zekerzhayard:forgewrapper:1.6.0".to_string(),
        url: Some("https://repo1.maven.org/maven2/".to_string()),
        download_url: Some("https://github.com/ZekerZhayard/ForgeWrapper/releases/download/1.6.0/ForgeWrapper-1.6.0.jar".to_string()),
        rules: None,
        natives: None,
        Extract: None,
        downloads: None,
        processor_only: false,
    };
    if let Some(ref mut libs) = profile.libraries { libs.push(fw_lib); } else { profile.libraries = Some(vec![fw_lib]); }

    // Append the installer's PROCESSOR libraries (installertools, jarsplitter, binarypatcher,
    // ForgeAutoRenamingTool, ...) so build_classpath fetches them to disk before the processors
    // run. Marked processor_only so they stay OFF the runtime classpath/module path.
    if let Ok(install_json) = serde_json::from_str::<serde_json::Value>(&install_contents) {
        if let Some(proc_libs) = install_json.get("libraries").and_then(|v| v.as_array()) {
            if let Ok(mut parsed) = serde_json::from_value::<Vec<Library>>(serde_json::Value::Array(proc_libs.clone())) {
                eprintln!("[{}] Adding {} processor libraries from install_profile.json (download-only, off classpath)", loader_tag, parsed.len());
                for l in parsed.iter_mut() { l.processor_only = true; }
                if let Some(ref mut libs) = profile.libraries { libs.extend(parsed); } else { profile.libraries = Some(parsed); }
            }
        }
    }

    profile.main_class = Some("io.github.zekerzhayard.forgewrapper.installer.Main".to_string());
    Some(profile)
}

/// The NeoForge version prefix that targets a given Minecraft version.
/// NeoForge `X.Y.Z` targets MC `1.X.Y` (patchless `1.X` maps to the `X.0.` line). So MC 1.21.5
/// -> "21.5.", MC 1.21 -> "21.0.". Returns None for versions that aren't `1.x[.y]` (snapshots etc.).
fn neoforge_prefix(mc_version: &str) -> Option<String> {
    let parts: Vec<&str> = mc_version.split('.').collect();
    if parts.len() < 2 || parts[0] != "1" { return None; }
    let major = parts[1];
    let minor = parts.get(2).copied().unwrap_or("0");
    Some(format!("{}.{}.", major, minor))
}

/// Candidate NeoForge versions to try for an instance, in priority order.
///
/// The instance's MC version is authoritative. A user-requested version is honored ONLY if it
/// targets this MC line (same `X.Y.` prefix) — otherwise it's silently dropped and replaced with
/// the newest NeoForge that actually targets this MC. This prevents the classic
/// "Patch expected <class> checksum A but it was B" installer failure, which happens when e.g.
/// NeoForge 21.3.x (built against MC 1.21.3 classes) is fed a 1.21.2 client jar. Empty result =>
/// NeoForge has no build for this MC version.
async fn neoforge_version_candidates(mc_version: &str, requested: &str) -> Vec<String> {
    let want = neoforge_prefix(mc_version);
    let req = requested.split_whitespace().next().unwrap_or("").trim().to_string();

    let mut out: Vec<String> = Vec::new();
    if let Some(ref w) = want {
        if !req.is_empty() && req.starts_with(w) { out.push(req.clone()); }
    } else if !req.is_empty() {
        // Unknown MC shape — can't validate, so just trust what was asked for.
        out.push(req.clone());
    }

    // Resolve the newest build targeting this MC line from the official version list.
    if let Some(ref w) = want {
        if let Ok(resp) = reqwest::get("https://maven.neoforged.net/api/maven/versions/releases/net%2Fneoforged%2Fneoforge").await {
            if let Ok(text) = resp.text().await {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(arr) = json.get("versions").and_then(|v| v.as_array()) {
                        // API returns ascending order; the last match on this line is newest.
                        let mut latest: Option<String> = None;
                        for v in arr.iter().filter_map(|v| v.as_str()) {
                            if v.starts_with(w) { latest = Some(v.to_string()); }
                        }
                        if let Some(l) = latest {
                            if !out.contains(&l) { out.push(l); }
                        }
                    }
                }
            }
        }
    }
    out
}

/// Download/Fetch mod loader manifest (Fabric, Quilt, Forge, NeoForge) with guaranteed fallback
async fn fetch_loader_profile(mc_version: &str, loader: &str, loader_ver: &str) -> VersionJson {
    let loader_lower = loader.to_lowercase();
    let clean_ver = loader_ver.split_whitespace().next().unwrap_or(loader_ver).to_string();
    let full_ver = if clean_ver.contains('-') {
        clean_ver.clone()
    } else {
        format!("{}-{}", mc_version, clean_ver)
    };

    let mut urls = Vec::new();
    match loader_lower.as_str() {
        "fabric" => {
            let ver = if clean_ver.is_empty() {
                let meta_url = format!("https://meta.fabricmc.net/v2/versions/loader/{}", mc_version);
                if let Ok(resp) = reqwest::get(&meta_url).await {
                    if let Ok(text) = resp.text().await {
                        if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(&text) {
                            arr.first().and_then(|item| item.get("loader")?.get("version")?.as_str()).map(|s| s.to_string()).unwrap_or_default()
                        } else { String::new() }
                    } else { String::new() }
                } else { String::new() }
            } else { clean_ver.clone() };
            if !ver.is_empty() {
                urls.push(format!("https://meta.fabricmc.net/v2/versions/loader/{}/{}/profile/json", mc_version, ver));
            }
            urls.push(format!("https://meta.fabricmc.net/v2/versions/loader/{}/profile/json", mc_version));
        }
        "quilt" => {
            let ver = if clean_ver.is_empty() {
                let meta_url = format!("https://meta.quiltmc.org/v3/versions/loader/{}", mc_version);
                if let Ok(resp) = reqwest::get(&meta_url).await {
                    if let Ok(text) = resp.text().await {
                        if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(&text) {
                            arr.first().and_then(|item| item.get("loader")?.get("version")?.as_str()).map(|s| s.to_string()).unwrap_or_default()
                        } else { String::new() }
                    } else { String::new() }
                } else { String::new() }
            } else { clean_ver.clone() };
            if !ver.is_empty() {
                urls.push(format!("https://meta.quiltmc.org/v3/versions/loader/{}/{}/profile/json", mc_version, ver));
            }
            urls.push(format!("https://meta.quiltmc.org/v3/versions/loader/{}/profile/json", mc_version));
        }
        "forge" => {
            let mut try_vers = vec![full_ver.clone(), clean_ver.clone()];
            
            // Auto-query promotions_slim.json if version is obsolete/mismatched
            let promo_url = "https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json";
            if let Ok(resp_p) = reqwest::get(promo_url).await {
                if let Ok(text_p) = resp_p.text().await {
                    if let Ok(json_p) = serde_json::from_str::<serde_json::Value>(&text_p) {
                        if let Some(promos) = json_p.get("promos").and_then(|p| p.as_object()) {
                            if let Some(rec) = promos.get(&format!("{}-recommended", mc_version)).and_then(|v| v.as_str()) {
                                try_vers.push(format!("{}-{}", mc_version, rec));
                            }
                            if let Some(lat) = promos.get(&format!("{}-latest", mc_version)).and_then(|v| v.as_str()) {
                                try_vers.push(format!("{}-{}", mc_version, lat));
                            }
                        }
                    }
                }
            }

            for test_ver in &try_vers {
                let installer_url = format!("https://maven.minecraftforge.net/net/minecraftforge/forge/{}/forge-{}-installer.jar", test_ver, test_ver);
                eprintln!("[Forge] Attempting installer fetch from: {}", installer_url);
                if let Ok(resp) = reqwest::get(&installer_url).await {
                    if resp.status().is_success() {
                        if let Ok(bytes) = resp.bytes().await {
                            if let Some(profile) = profile_from_installer(bytes.as_ref(), "Forge") {
                                eprintln!("[Forge] Built launch profile from installer ({})", test_ver);
                                return profile;
                            }
                        }
                    }
                }
            }
        }
        "neoforge" => {
            // NeoForge ships version.json + install_profile.json INSIDE its installer jar (exactly
            // like Forge 1.13+) — there is NO standalone -version.json on Maven, so extract it.
            // ForgeWrapper 1.6.0 supports NeoForge (>20.2.x): it detects it from the
            // --fml.neoForgeVersion arg already present in NeoForge's version.json.
            //
            // Candidates are validated against the instance's MC version (see helper): a
            // mismatched requested version is dropped in favour of the newest build that actually
            // targets this MC, avoiding the binary-patch checksum failure.
            let try_vers = neoforge_version_candidates(mc_version, &clean_ver).await;

            for test_ver in &try_vers {
                if test_ver.is_empty() { continue; }
                let installer_url = format!("https://maven.neoforged.net/releases/net/neoforged/neoforge/{}/neoforge-{}-installer.jar", test_ver, test_ver);
                eprintln!("[NeoForge] Attempting installer fetch from: {}", installer_url);
                if let Ok(resp) = reqwest::get(&installer_url).await {
                    if resp.status().is_success() {
                        if let Ok(bytes) = resp.bytes().await {
                            if let Some(profile) = profile_from_installer(bytes.as_ref(), "NeoForge") {
                                eprintln!("[NeoForge] Built launch profile from installer ({})", test_ver);
                                return profile;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }

    for url in &urls {
        eprintln!("[Loader] Attempting profile fetch from: {}", url);
        if let Ok(resp) = reqwest::get(url).await {
            if let Ok(text) = resp.text().await {
                if let Ok(mut profile) = serde_json::from_str::<VersionJson>(&text) {
                    if profile.main_class.is_some() || profile.libraries.is_some() {
                        if let Some(ref mc) = profile.main_class {
                            if mc.contains("zekerzhayard") {
                                profile.main_class = Some("cpw.mods.bootstraplauncher.BootstrapLauncher".to_string());
                            }
                        }
                        eprintln!("[Loader] Successfully loaded profile from {}", url);
                        return profile;
                    }
                } else if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                    let main_class = v.get("mainClass").and_then(|s| s.as_str()).map(|s| s.to_string());
                    let libraries: Option<Vec<Library>> = v.get("libraries").and_then(|libs| serde_json::from_value(libs.clone()).ok());
                    let arguments: Option<VersionArguments> = v.get("arguments").and_then(|args| serde_json::from_value(args.clone()).ok());
                    let minecraft_arguments = v.get("minecraftArguments").and_then(|s| s.as_str()).map(|s| s.to_string());
                    if main_class.is_some() || libraries.is_some() {
                        eprintln!("[Loader] Successfully parsed fallback profile from {}", url);
                        return VersionJson {
                            main_class,
                            arguments,
                            libraries,
                            asset_index: None,
                            assets: None,
                            minecraft_arguments,
                            downloads: None,
                            java_version: None,
                        };
                    }
                }
            }
        }
    }

    // === GUARANTEED DYNAMIC FALLBACK (Never launch Vanilla for modded instances) ===
    eprintln!("[Loader] Network profile fetch failed. Constructing dynamic loader profile for {} {}...", loader, clean_ver);

    let (main_class, lib_name, lib_url) = match loader_lower.as_str() {
        "fabric" => (
            "net.fabricmc.loader.impl.launch.knot.KnotClient".to_string(),
            format!("net.fabricmc:fabric-loader:{}", if clean_ver.is_empty() { "0.15.11".to_string() } else { clean_ver.clone() }),
            Some("https://maven.fabricmc.net/".to_string()),
        ),
        "quilt" => (
            "org.quiltmc.loader.impl.launch.knot.KnotClient".to_string(),
            format!("org.quiltmc:quilt-loader:{}", if clean_ver.is_empty() { "0.24.0".to_string() } else { clean_ver.clone() }),
            Some("https://maven.quiltmc.org/repository/release/".to_string()),
        ),
        "neoforge" => (
            "net.neoforged.neoforgeserver.ServerMain".to_string(),
            format!("net.neoforged:neoforge:{}", clean_ver),
            Some("https://maven.neoforged.net/releases/".to_string()),
        ),
        _ => {
            // Forge
            let forge_main = if mc_version.starts_with("1.18") || mc_version.starts_with("1.19") || mc_version.starts_with("1.20") || mc_version.starts_with("1.21") {
                "cpw.mods.bootstraplauncher.BootstrapLauncher".to_string()
            } else if mc_version.starts_with("1.13") || mc_version.starts_with("1.14") || mc_version.starts_with("1.15") || mc_version.starts_with("1.16") || mc_version.starts_with("1.17") {
                "cpw.mods.modlauncher.Launcher".to_string()
            } else {
                "net.minecraft.launchwrapper.Launch".to_string()
            };
            (
                forge_main,
                format!("net.minecraftforge:forge:{}", full_ver),
                Some("https://maven.minecraftforge.net/".to_string()),
            )
        }
    };

    let loader_lib = Library {
        name: lib_name,
        url: lib_url,
        download_url: None,
        rules: None,
        natives: None,
        Extract: None,
        downloads: None,
        processor_only: false,
    };

    let mut loader_libs = vec![loader_lib];
    if loader_lower == "forge" {
        loader_libs.push(Library {
            name: "cpw.mods:bootstraplauncher:1.1.2".to_string(),
            url: Some("https://maven.minecraftforge.net/".to_string()),
            download_url: None,
            rules: None,
            natives: None,
            Extract: None,
            downloads: None,
            processor_only: false,
        });
        loader_libs.push(Library {
            name: "cpw.mods:securejarhandler:2.1.10".to_string(),
            url: Some("https://maven.minecraftforge.net/".to_string()),
            download_url: None,
            rules: None,
            natives: None,
            Extract: None,
            downloads: None,
            processor_only: false,
        });
        loader_libs.push(Library {
            name: "org.ow2.asm:asm:9.5".to_string(),
            url: Some("https://repo1.maven.org/maven2/".to_string()),
            download_url: None,
            rules: None,
            natives: None,
            Extract: None,
            downloads: None,
            processor_only: false,
        });
    }

    let extra_arguments = if loader_lower == "forge" {
        if mc_version.starts_with("1.20") || mc_version.starts_with("1.21") || mc_version.starts_with("1.19") || mc_version.starts_with("1.18") || mc_version.starts_with("1.17") || mc_version.starts_with("1.16") || mc_version.starts_with("1.15") || mc_version.starts_with("1.14") || mc_version.starts_with("1.13") {
            Some(VersionArguments {
                game: Some(vec![
                    serde_json::json!("--launchTarget"),
                    serde_json::json!("forgeclient"),
                    serde_json::json!("--fml.forgeVersion"),
                    serde_json::json!(clean_ver),
                    serde_json::json!("--fml.mcVersion"),
                    serde_json::json!(mc_version),
                    serde_json::json!("--fml.forgeGroup"),
                    serde_json::json!("net.minecraftforge"),
                    serde_json::json!("--fml.mcpVersion"),
                    serde_json::json!(mc_version),
                ]),
                jvm: Some(vec![
                    serde_json::json!("-Dforgesvc.net=true"),
                    serde_json::json!("-DignoreList=bootstraplauncher,securejarhandler,asm-all,asm-debug-all,java-objc-bridge"),
                ]),
            })
        } else { None }
    } else if loader_lower == "neoforge" {
        Some(VersionArguments {
            game: Some(vec![
                serde_json::json!("--launchTarget"),
                serde_json::json!("neoforgeclient"),
                serde_json::json!("--fml.neoForgeVersion"),
                serde_json::json!(clean_ver),
                serde_json::json!("--fml.mcVersion"),
                serde_json::json!(mc_version),
            ]),
            jvm: Some(vec![
                serde_json::json!("-Dneoforge.net=true"),
            ]),
        })
    } else { None };

    VersionJson {
        main_class: Some(main_class),
        arguments: extra_arguments,
        libraries: Some(loader_libs),
        asset_index: None,
        assets: None,
        minecraft_arguments: if loader_lower == "forge" && (mc_version.starts_with("1.7") || mc_version.starts_with("1.12")) {
            Some("--tweakClass net.minecraftforge.fml.common.launcher.FMLTweaker".to_string())
        } else { None },
        downloads: None,
        java_version: None,
    }
}

// === Public API ===

pub async fn launch_game(
    app: AppHandle,
    instance_id: &str,
    account_username: &str,
    account_uuid: &str,
) -> Result<LaunchResult, String> {
    let instances_dir = instances_root()?;
    let instance_dir = instances_dir.join(instance_id);
    let cfg = instances::get_instance_config(instance_id)?;

    // 1. Download version manifest (needed first: it tells us which Java to use)
    eprintln!("[Launch] Downloading version manifest for {}...", cfg.mc_version);
    let version_manifest = download_version_manifest(&cfg.mc_version).await?;
    eprintln!("[Launch] Vanilla manifest downloaded. Main class: {:?}", version_manifest.main_class);

    // 1b. Download Mod Loader Profile (Fabric, Quilt, Forge, NeoForge) if specified
    let loader_name = cfg.loader.clone().unwrap_or_default();
    let loader_ver = cfg.loader_version.clone().unwrap_or_default();

    let loader_profile: Option<VersionJson> = if !loader_name.is_empty() && loader_name.to_lowercase() != "vanilla" {
        eprintln!("[Launch] Fetching loader profile for {} (ver: {})...", loader_name, loader_ver);
        Some(fetch_loader_profile(&cfg.mc_version, &loader_name, &loader_ver).await)
    } else {
        None
    };

    // BUGFIX: For Forge AND NeoForge, everything downstream assumes ForgeWrapper is on the
    // classpath (mainClass gets forced to ForgeWrapper's Main a bit further below). If
    // fetch_loader_profile couldn't download/parse the real installer (network issue, wrong/
    // nonexistent version, Maven rate-limit, etc.) it silently falls back to a synthetic profile
    // that does NOT include ForgeWrapper. Launching in that state used to just spawn Java anyway,
    // which crashes almost instantly with a NoClassDefFoundError/ClassNotFoundException and exit
    // code 1, with no clear reason surfaced to the user. Fail fast here instead.
    let loader_l = loader_name.to_lowercase();
    if loader_l == "forge" || loader_l == "neoforge" {
        let has_forgewrapper = loader_profile
            .as_ref()
            .and_then(|lp| lp.libraries.as_ref())
            .map(|libs| libs.iter().any(|l| l.name.contains("forgewrapper")))
            .unwrap_or(false);
        if !has_forgewrapper {
            let (maven, human) = if loader_l == "neoforge" {
                ("maven.neoforged.net", "NeoForge")
            } else {
                ("maven.minecraftforge.net / files.minecraftforge.net", "Forge")
            };
            return Err(format!(
                "Не удалось получить {} installer для версии {} ({} {}). \
                 Проверьте подключение к {}, а также что указанная версия существует для MC {}. \
                 Запуск отменён, чтобы не крашить Java с непонятной ошибкой (exit code 1).",
                human, cfg.mc_version, human, loader_ver, maven, cfg.mc_version
            ));
        }
    }

    // 2. Find/Download the Java runtime THIS version requires
    let java_path = ensure_java_for_manifest(&version_manifest).await?;
    eprintln!("[Launch] Using Java: {}", java_path);

    // 3. Game directory
    let game_dir = instance_dir.join(".minecraft");
    let assets_dir = game_dir.join("assets");
    let libraries_dir = game_dir.join("libraries");
    let natives_dir = game_dir.join("natives");

    // 4. Combine vanilla libraries + loader libraries
    eprintln!("[Launch] Building classpath...");
    let mut libs = version_manifest.libraries.clone().unwrap_or_default();
    if let Some(ref l_profile) = loader_profile {
        if let Some(ref l_libs) = l_profile.libraries {
            eprintln!("[Launch] Adding {} loader libraries from {} profile...", l_libs.len(), loader_name);
            for lib in l_libs {
                libs.push(lib.clone());
            }
        }
    }

    eprintln!("[Launch] Total combined libraries count: {}", libs.len());
    let mut classpath = build_classpath(&app, &libs, &libraries_dir).await;

    // 4b. Download client JAR and add to classpath
    if let Some(ref downloads) = version_manifest.downloads {
        if let Some(ref client) = downloads.client {
            let client_jar = game_dir.join("minecraft-client.jar");
            if !client_jar.exists() {
                eprintln!("[Launch] Downloading client JAR: {}", client.url);
                if let Ok(resp) = reqwest::get(&client.url).await {
                    if let Ok(bytes) = resp.bytes().await {
                        let _ = fs::write(&client_jar, &bytes);
                        eprintln!("[Launch] Client JAR downloaded: {} bytes", bytes.len());
                    }
                }
            }
            if client_jar.exists() {
                if !classpath.is_empty() {
                    classpath.push(';');
                }
                classpath.push_str(&client_jar.to_string_lossy());
                eprintln!("[Launch] Client JAR added to classpath");
            }
        }
    }

    eprintln!("[Launch] Classpath length: {} chars", classpath.len());

    // 5. Extract natives
    eprintln!("[Launch] Extracting natives...");
    extract_natives(&libs, &libraries_dir, &natives_dir).await;
    eprintln!("[Launch] Natives extracted.");

    // 6. Download assets in parallel
    if let Some(ref asset_index) = version_manifest.asset_index {
        eprintln!("[Launch] Downloading assets for index: {}", asset_index.id);
        download_assets(&app, asset_index, &game_dir).await;
        eprintln!("[Launch] Assets done.");
    } else {
        eprintln!("[Launch] No asset index found.");
    }

    // 7. Determine main class (loader mainClass takes precedence over vanilla mainClass!)
    let mut main_class = loader_profile.as_ref().and_then(|lp| lp.main_class.clone())
        .or_else(|| version_manifest.main_class.clone())
        .unwrap_or_else(|| "net.minecraft.client.main.Main".to_string());

    if loader_name.to_lowercase() == "forge" || loader_name.to_lowercase() == "neoforge" {
        main_class = "io.github.zekerzhayard.forgewrapper.installer.Main".to_string();
        eprintln!("[Launch] Using ForgeWrapper mainClass: {}", main_class);
    }

    eprintln!("[Launch] Target mainClass: {}", main_class);

    // 8. Asset index ID
    let asset_id = version_manifest.asset_index.as_ref()
        .map(|ai| ai.id.clone())
        .unwrap_or_else(|| cfg.mc_version.clone());

    // 9. Build JVM arguments from manifest & loader profile
    let mut jvm_args_list: Vec<String> = Vec::new();
    let mut game_args_list: Vec<String> = Vec::new();

    // Combine JVM arguments from version manifest & loader profile
    let mut jvm_sources = Vec::new();
    if let Some(ref v_args) = version_manifest.arguments {
        if let Some(ref jvm) = v_args.jvm { jvm_sources.push(jvm); }
    }
    if let Some(ref l_profile) = loader_profile {
        if let Some(ref l_args) = l_profile.arguments {
            if let Some(ref jvm) = l_args.jvm { jvm_sources.push(jvm); }
        }
    }

    for jvm_src in jvm_sources {
        for arg in jvm_src {
            if !arg_rules_allowed(arg) { continue; }
            for val in extract_arg_values(arg) {
                let resolved = val
                    .replace("${natives_directory}", &natives_dir.to_string_lossy())
                    .replace("${launcher_name}", "MyL")
                    .replace("${launcher_version}", "0.1.0")
                    .replace("${classpath}", &classpath)
                    .replace("${library_directory}", &libraries_dir.to_string_lossy())
                    .replace("${classpath_separator}", ";")
                    .replace("${version_name}", &cfg.mc_version);
                jvm_args_list.push(resolved);
            }
        }
    }

    // Add ForgeWrapper System Properties for Forge / NeoForge instances.
    // ForgeWrapper needs -Dforgewrapper.installer pointing at the ORIGINAL installer jar (it runs
    // the installer's processors from there at launch). Forge and NeoForge live under different
    // Maven coordinates, so resolve the on-disk installer path per loader.
    if loader_name.to_lowercase() == "forge" || loader_name.to_lowercase() == "neoforge" || main_class.contains("zekerzhayard") {
        let is_neo = loader_name.to_lowercase() == "neoforge";
        let clean_ver = loader_ver.split_whitespace().next().unwrap_or(&loader_ver);
        let mut actual_installer_path = PathBuf::new();

        if is_neo {
            // NeoForge: net/neoforged/neoforge/<ver>/neoforge-<ver>-installer.jar
            // Must resolve the SAME version as fetch_loader_profile did (so -Dforgewrapper.installer
            // matches the --fml.neoForgeVersion baked into the extracted version.json).
            let try_vers = neoforge_version_candidates(&cfg.mc_version, clean_ver).await;
            for test_ver in &try_vers {
                if test_ver.is_empty() { continue; }
                let target_path = libraries_dir.join("net").join("neoforged").join("neoforge").join(test_ver).join(format!("neoforge-{}-installer.jar", test_ver));
                if target_path.exists() && fs::metadata(&target_path).map(|m| m.len() > 1000000).unwrap_or(false) {
                    actual_installer_path = target_path;
                    break;
                }
                let url = format!("https://maven.neoforged.net/releases/net/neoforged/neoforge/{}/neoforge-{}-installer.jar", test_ver, test_ver);
                eprintln!("[NeoForge] Attempting installer download: {}", url);
                if let Ok(resp) = reqwest::get(&url).await {
                    if resp.status().is_success() {
                        if let Ok(bytes) = resp.bytes().await {
                            if bytes.len() > 1000000 {
                                if let Some(parent) = target_path.parent() { let _ = fs::create_dir_all(parent); }
                                if fs::write(&target_path, &bytes).is_ok() {
                                    actual_installer_path = target_path;
                                    eprintln!("[NeoForge] Saved installer JAR ({}, {} bytes)", test_ver, bytes.len());
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            // Forge: net/minecraftforge/forge/<mc>-<ver>/forge-<mc>-<ver>-installer.jar
            let full_ver = if clean_ver.contains('-') { clean_ver.to_string() } else { format!("{}-{}", cfg.mc_version, clean_ver) };
            let mut try_vers = vec![full_ver.clone(), clean_ver.to_string()];

            let promo_url = "https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json";
            if let Ok(resp_p) = reqwest::get(promo_url).await {
                if let Ok(text_p) = resp_p.text().await {
                    if let Ok(json_p) = serde_json::from_str::<serde_json::Value>(&text_p) {
                        if let Some(promos) = json_p.get("promos").and_then(|p| p.as_object()) {
                            if let Some(rec) = promos.get(&format!("{}-recommended", cfg.mc_version)).and_then(|v| v.as_str()) {
                                try_vers.push(format!("{}-{}", cfg.mc_version, rec));
                            }
                            if let Some(lat) = promos.get(&format!("{}-latest", cfg.mc_version)).and_then(|v| v.as_str()) {
                                try_vers.push(format!("{}-{}", cfg.mc_version, lat));
                            }
                        }
                    }
                }
            }

            for test_ver in &try_vers {
                let target_path = libraries_dir.join("net").join("minecraftforge").join("forge").join(test_ver).join(format!("forge-{}-installer.jar", test_ver));
                if target_path.exists() && fs::metadata(&target_path).map(|m| m.len() > 1000000).unwrap_or(false) {
                    actual_installer_path = target_path;
                    break;
                }
                let url = format!("https://maven.minecraftforge.net/net/minecraftforge/forge/{}/forge-{}-installer.jar", test_ver, test_ver);
                eprintln!("[Forge] Attempting installer download: {}", url);
                if let Ok(resp) = reqwest::get(&url).await {
                    if resp.status().is_success() {
                        if let Ok(bytes) = resp.bytes().await {
                            if bytes.len() > 1000000 {
                                if let Some(parent) = target_path.parent() {
                                    let _ = fs::create_dir_all(parent);
                                }
                                if fs::write(&target_path, &bytes).is_ok() {
                                    actual_installer_path = target_path;
                                    eprintln!("[Forge] Saved installer JAR ({}, {} bytes)", test_ver, bytes.len());
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        // BUGFIX: previously, if none of `try_vers` resolved to a downloadable installer jar,
        // `actual_installer_path` stayed as an empty PathBuf and we'd still push
        // "-Dforgewrapper.installer=" (empty) into the JVM args. ForgeWrapper then throws
        // trying to open a nonexistent/empty path and Java exits with code 1. Fail fast
        // with a clear message instead of spawning a doomed process.
        if actual_installer_path.as_os_str().is_empty() || !actual_installer_path.exists() {
            let human = if is_neo { "NeoForge" } else { "Forge" };
            return Err(format!(
                "Не удалось скачать/найти installer jar для {} {} (MC {}). \
                 -Dforgewrapper.installer оказался бы пустым, и ForgeWrapper гарантированно упал бы \
                 с exit code 1. Проверьте интернет-соединение и версию загрузчика.",
                human, loader_ver, cfg.mc_version
            ));
        }

        let client_jar = game_dir.join("minecraft-client.jar");
        jvm_args_list.push(format!("-Dforgewrapper.installer={}", actual_installer_path.to_string_lossy()));
        jvm_args_list.push(format!("-Dforgewrapper.minecraft={}", client_jar.to_string_lossy()));
        jvm_args_list.push(format!("-Dforgewrapper.librariesDir={}", libraries_dir.to_string_lossy()));
        eprintln!("[Launch] Added ForgeWrapper properties:\n  installer: {:?}\n  minecraft: {:?}\n  librariesDir: {:?}", actual_installer_path, client_jar, libraries_dir);
    }

    // Combine Game arguments
    let mut game_sources = Vec::new();
    if let Some(ref v_args) = version_manifest.arguments {
        if let Some(ref game) = v_args.game { game_sources.push(game); }
    }
    if let Some(ref l_profile) = loader_profile {
        if let Some(ref l_args) = l_profile.arguments {
            if let Some(ref game) = l_args.game { game_sources.push(game); }
        }
    }

    if !game_sources.is_empty() {
        for game_src in game_sources {
            for arg in game_src {
                if !arg_rules_allowed(arg) { continue; }
                for val in extract_arg_values(arg) {
                    let resolved = val
                        .replace("${auth_player_name}", account_username)
                        .replace("${auth_uuid}", account_uuid)
                        .replace("${auth_access_token}", "0")
                        .replace("${clientid}", "00000000-0000-0000-0000-000000000000")
                        .replace("${auth_xuid}", "0")
                        .replace("${version_name}", &cfg.mc_version)
                        .replace("${game_directory}", &game_dir.to_string_lossy())
                        .replace("${assets_root}", &assets_dir.to_string_lossy())
                        .replace("${assets_index_name}", &asset_id)
                        .replace("${user_type}", "offline")
                        .replace("${version_type}", "MyL")
                        .replace("${user_properties}", "{}");
                    game_args_list.push(resolved);
                }
            }
        }
    } else {
        // Fallback for old legacy arguments
        let mc_args_str = loader_profile.as_ref().and_then(|lp| lp.minecraft_arguments.clone())
            .or_else(|| version_manifest.minecraft_arguments.clone());

        if let Some(ref mc_args) = mc_args_str {
            for part in mc_args.split_whitespace() {
                let resolved = part
                    .replace("${auth_player_name}", account_username)
                    .replace("${auth_uuid}", account_uuid)
                    .replace("${auth_access_token}", "0")
                    .replace("${version_name}", &cfg.mc_version)
                    .replace("${game_directory}", &game_dir.to_string_lossy())
                    .replace("${assets_root}", &assets_dir.to_string_lossy())
                    .replace("${assets_index_name}", &asset_id)
                    .replace("${user_type}", "offline");
                game_args_list.push(resolved);
            }
        }
    }

    // Add custom JVM args from instance config if present
    if let Some(ref custom_jvm) = cfg.jvm_args {
        for arg in custom_jvm.split_whitespace() {
            if !arg.is_empty() && !jvm_args_list.contains(&arg.to_string()) {
                jvm_args_list.push(arg.to_string());
            }
        }
    }

    // Add default JVM memory args if not present
    if !jvm_args_list.iter().any(|a| a.starts_with("-Xmx")) {
        jvm_args_list.insert(0, "-Xmx2G".to_string());
    }
    if !jvm_args_list.iter().any(|a| a.starts_with("-Xms")) {
        jvm_args_list.insert(1, "-Xms512M".to_string());
    }

    // Assemble final args: JVM args + main class + game args
    let mut args = jvm_args_list;
    args.push(main_class);
    args.extend(game_args_list);

    eprintln!("[Launch] Args count: {}", args.len());
    eprintln!("[Launch] Game dir exists: {}", game_dir.exists());
    eprintln!("[Launch] Libraries dir exists: {}", libraries_dir.exists());
    eprintln!("[Launch] Spawning Java...");

    // Persist the full launch command + all game output to a log file. Crashes happen
    // inside the Java child (not this process), so without this the only record is the
    // transient in-UI console. This file lets us inspect exit-code-1 failures after the fact.
    let launch_log_path = instance_dir.join("myl-latest-launch.log");
    let log_file = {
        use std::io::Write;
        let mut f = fs::File::create(&launch_log_path).ok();
        if let Some(ref mut file) = f {
            let _ = writeln!(file, "=== MyL launch: instance {} ===", instance_id);
            let _ = writeln!(file, "Java: {}", java_path);
            let _ = writeln!(file, "\nFull command:\n\"{}\" {}\n", java_path, args.join(" "));
            let _ = writeln!(file, "=== GAME OUTPUT ===");
        }
        f.map(|file| Arc::new(Mutex::new(file)))
    };
    eprintln!("[Launch] Writing launch log to: {:?}", launch_log_path);

    // 10. Spawn process
    let mut child = Command::new(&java_path)
        .args(&args)
        .current_dir(&game_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start Java: {}", e))?;

    let pid = child.id();
    eprintln!("[Launch] Java process spawned with PID: {}", pid);

    // Stream stdout in background thread
    if let Some(stdout) = child.stdout.take() {
        let app_handle = app.clone();
        let inst_id = instance_id.to_string();
        let log = log_file.clone();
        std::thread::spawn(move || {
            use std::io::Write;
            let reader = BufReader::new(stdout);
            for line in reader.lines().flatten() {
                if let Some(ref lf) = log {
                    if let Ok(mut f) = lf.lock() { let _ = writeln!(f, "{}", line); }
                }
                let _ = app_handle.emit("game-log", serde_json::json!({
                    "instanceId": inst_id,
                    "line": line
                }));
            }
        });
    }

    // Stream stderr in background thread
    if let Some(stderr) = child.stderr.take() {
        let app_handle = app.clone();
        let inst_id = instance_id.to_string();
        let log = log_file.clone();
        std::thread::spawn(move || {
            use std::io::Write;
            let reader = BufReader::new(stderr);
            for line in reader.lines().flatten() {
                if let Some(ref lf) = log {
                    if let Ok(mut f) = lf.lock() { let _ = writeln!(f, "[stderr] {}", line); }
                }
                let _ = app_handle.emit("game-log", serde_json::json!({
                    "instanceId": inst_id,
                    "line": line
                }));
            }
        });
    }

    {
        let mut proc = GAME_PROCESS.lock().map_err(|e| e.to_string())?;
        *proc = Some(child);
    }

    // Monitor process exit in background thread
    let app_handle_exit = app.clone();
    let inst_id_exit = instance_id.to_string();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(500));
            if let Ok(mut proc) = GAME_PROCESS.lock() {
                if let Some(ref mut c) = *proc {
                    if c.id() == pid {
                        match c.try_wait() {
                            Ok(Some(status)) => {
                                eprintln!("[Launch] Process PID {} exited with status: {:?}", pid, status);
                                *proc = None;
                                let _ = app_handle_exit.emit("game-exit", serde_json::json!({
                                    "instanceId": inst_id_exit,
                                    "pid": pid,
                                    "code": status.code().unwrap_or(0)
                                }));
                                break;
                            }
                            Ok(None) => {}
                            Err(_) => {
                                *proc = None;
                                let _ = app_handle_exit.emit("game-exit", serde_json::json!({
                                    "instanceId": inst_id_exit,
                                    "pid": pid,
                                    "code": -1
                                }));
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                } else {
                    let _ = app_handle_exit.emit("game-exit", serde_json::json!({
                        "instanceId": inst_id_exit,
                        "pid": pid,
                        "code": 0
                    }));
                    break;
                }
            }
        }
    });

    Ok(LaunchResult {
        success: true,
        message: format!("Game launched (PID: {})", pid),
        pid: Some(pid),
    })
}

pub fn stop_game() -> Result<(), String> {
    let mut proc = GAME_PROCESS.lock().map_err(|e| e.to_string())?;
    if let Some(ref mut child) = *proc {
        child.kill().map_err(|e| format!("Failed to kill: {}", e))?;
        *proc = None;
        Ok(())
    } else {
        Err("No game running".to_string())
    }
}

async fn download_version_manifest(mc_version: &str) -> Result<VersionJson, String> {
    let meta_dir = mojang_meta_dir()?;
    let manifest_path = meta_dir.join(format!("version_{}.json", mc_version.replace('.', "_")));

    // Try cache
    if manifest_path.exists() {
        eprintln!("[Manifest] Cache exists: {:?}", manifest_path);
        if let Ok(data) = fs::read_to_string(&manifest_path) {
            eprintln!("[Manifest] Cache data length: {}", data.len());
            match serde_json::from_str::<VersionJson>(&data) {
                Ok(vj) => {
                    eprintln!("[Manifest] Cache parsed OK. main_class: {:?}", vj.main_class);
                    return Ok(vj);
                }
                Err(e) => {
                    eprintln!("[Manifest] Cache parse failed: {}. Re-downloading.", e);
                }
            }
        }
    }

    eprintln!("[Manifest] Downloading manifest list...");
    let resp = reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest_v2.json")
        .await.map_err(|e| e.to_string())?;
    let text = resp.text().await.map_err(|e| e.to_string())?;
    let manifest_list: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    let versions = manifest_list["versions"].as_array().ok_or("No versions array")?;
    let version_url = versions.iter()
        .find(|v| v["id"].as_str() == Some(mc_version))
        .and_then(|v| v["url"].as_str())
        .ok_or(format!("Version {} not found", mc_version))?;

    eprintln!("[Manifest] Downloading version JSON from: {}", version_url);
    let resp = reqwest::get(version_url).await.map_err(|e| e.to_string())?;
    let text = resp.text().await.map_err(|e| e.to_string())?;
    eprintln!("[Manifest] Downloaded {} bytes", text.len());

    // Debug: check mainClass in raw JSON
    if let Ok(raw) = serde_json::from_str::<serde_json::Value>(&text) {
        eprintln!("[Manifest] Raw mainClass: {:?}", raw.get("mainClass"));
    }

    let _ = fs::write(&manifest_path, &text);

    let result = serde_json::from_str::<VersionJson>(&text).map_err(|e| e.to_string())?;
    eprintln!("[Manifest] Parsed VersionJson. main_class: {:?}", result.main_class);
    Ok(result)
}

// === Java Auto-Download ===

/// Detect major Java version from a java.exe
fn detect_java_major_version(java_exe: &Path) -> Option<u32> {
    let output = Command::new(java_exe).arg("-version").output().ok()?;
    let text = String::from_utf8_lossy(&output.stderr);
    let line = text.lines().next()?;
    let start = line.find('"')? + 1;
    let rest = &line[start..];
    let end = rest.find('"')?;
    let version = &rest[..end];
    let mut parts = version.split('.');
    let first: u32 = parts.next()?.parse().ok()?;
    if first == 1 {
        parts.next()?.parse().ok()
    } else {
        Some(first)
    }
}

fn java_dir() -> Result<PathBuf, String> {
    let appdata = dirs::data_local_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("AppData").join("Roaming")))
        .ok_or("Cannot determine AppData")?;
    Ok(appdata.join("MyL").join("java"))
}

/// Download Java from Adoptium API (ZIP format)
async fn download_java_from_adoptium(major_version: u32) -> Result<String, String> {
    let java_base = java_dir()?;
    let target_dir = java_base.join(format!("jdk-{}", major_version));
    let java_exe = target_dir.join("bin").join("java.exe");

    // Check if already cached (direct path or inside subfolder like jdk-25.0.3+9/)
    if java_exe.exists() {
        eprintln!("[Java] Adoptium JDK {} cached: {:?}", major_version, java_exe);
        return Ok(java_exe.to_string_lossy().to_string());
    }
    // Check inside any subfolder
    if target_dir.exists() {
        if let Ok(entries) = fs::read_dir(&target_dir) {
            for entry in entries.flatten() {
                let exe = entry.path().join("bin").join("java.exe");
                if exe.exists() {
                    eprintln!("[Java] Adoptium JDK {} cached: {:?}", major_version, exe);
                    return Ok(exe.to_string_lossy().to_string());
                }
            }
        }
    }

    eprintln!("[Java] Downloading Adoptium JDK {}...", major_version);

    // Query Adoptium API
    let api_url = format!(
        "https://api.adoptium.net/v3/assets/latest/{}/hotspot?architecture=x64&image_type=jdk&os=windows&vendor=eclipse",
        major_version
    );
    let resp = reqwest::get(&api_url).await.map_err(|e| format!("Adoptium API error: {}", e))?;
    let assets: Vec<serde_json::Value> = resp.json().await.map_err(|e| format!("Adoptium parse error: {}", e))?;

    let asset = assets.first().ok_or("No Adoptium assets found")?;
    let zip_url = asset["binary"]["package"]["link"]
        .as_str()
        .ok_or("No download URL in Adoptium response")?;

    eprintln!("[Java] Downloading from: {}", zip_url);

    // Download ZIP
    let resp = reqwest::get(zip_url).await.map_err(|e| format!("Download error: {}", e))?;
    let bytes = resp.bytes().await.map_err(|e| format!("Download body error: {}", e))?;

    eprintln!("[Java] Downloaded {} bytes, extracting...", bytes.len());

    // Extract ZIP
    let cursor = std::io::Cursor::new(&bytes);
    let mut archive = zip::ZipArchive::new(cursor).map_err(|e| format!("ZIP parse error: {}", e))?;

    let _ = fs::create_dir_all(&target_dir);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| format!("ZIP entry error: {}", e))?;
        let out_path = target_dir.join(file.name());

        if file.is_dir() {
            let _ = fs::create_dir_all(&out_path);
        } else {
            if let Some(parent) = out_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let mut out_file = fs::File::create(&out_path).map_err(|e| format!("File create error: {}", e))?;
            std::io::copy(&mut file, &mut out_file).map_err(|e| format!("File write error: {}", e))?;
        }
    }

    // Adoptium ZIPs contain a single folder like "jdk-25.0.3+9" - find java.exe inside
    if java_exe.exists() {
        eprintln!("[Java] JDK {} installed: {:?}", major_version, java_exe);
        Ok(java_exe.to_string_lossy().to_string())
    } else {
        // Try to find java.exe in any subfolder
        if let Ok(entries) = fs::read_dir(&target_dir) {
            for entry in entries.flatten() {
                let exe = entry.path().join("bin").join("java.exe");
                if exe.exists() {
                    eprintln!("[Java] JDK {} installed: {:?}", major_version, exe);
                    return Ok(exe.to_string_lossy().to_string());
                }
            }
        }
        Err(format!("JDK {} downloaded but java.exe not found", major_version))
    }
}

/// Ensure Java runtime required by the version manifest
async fn ensure_java_for_manifest(version_manifest: &VersionJson) -> Result<String, String> {
    let major_version = version_manifest.java_version.as_ref()
        .map(|jv| jv.major_version)
        .unwrap_or(21);

    // Check JAVA_HOME first
    if let Ok(java_home) = std::env::var("JAVA_HOME") {
        let java_exe = PathBuf::from(&java_home).join("bin").join("java.exe");
        if java_exe.exists() {
            match detect_java_major_version(&java_exe) {
                Some(v) if v >= major_version => return Ok(java_exe.to_string_lossy().to_string()),
                Some(v) => eprintln!("[Java] JAVA_HOME is Java {} but need {}+, skipping", v, major_version),
                None => eprintln!("[Java] Could not detect JAVA_HOME version"),
            }
        }
    }

    // Download from Adoptium
    download_java_from_adoptium(major_version).await
}

/// Standalone entry point (keeps the old signature)
pub async fn ensure_java_for_version(mc_version: &str) -> Result<String, String> {
    let version_manifest = download_version_manifest(mc_version).await?;
    ensure_java_for_manifest(&version_manifest).await
}
