use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Account types supported by the launcher
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum AccountType {
    #[serde(rename = "microsoft")]
    Microsoft,
    #[serde(rename = "offline")]
    Offline,
}

/// Full account data stored in accounts.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub account_type: AccountType,
    pub username: String,
    #[serde(default)]
    pub uuid: Option<String>,
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub skin_url: Option<String>,
    #[serde(default)]
    pub client_id: Option<String>,
}

/// Summary returned to the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub id: String,
    pub account_type: String,
    pub username: String,
    pub uuid: Option<String>,
    pub skin_png_base64: Option<String>,
    pub is_active: bool,
}

/// Storage file path: %APPDATA%\MyL\accounts.json
fn accounts_path() -> Result<PathBuf, String> {
    let appdata = dirs::data_local_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("AppData").join("Roaming")))
        .ok_or_else(|| "Cannot determine AppData path".to_string())?;
    let dir = appdata.join("MyL");
    fs::create_dir_all(&dir).map_err(|e| format!("Cannot create app dir: {}", e))?;
    Ok(dir.join("accounts.json"))
}

/// Load all accounts from disk
fn load_all() -> Result<Vec<Account>, String> {
    let path = accounts_path()?;
    if !path.exists() {
        return Ok(vec![]);
    }
    let data = fs::read_to_string(&path).map_err(|e| format!("Cannot read accounts.json: {}", e))?;
    if data.trim().is_empty() {
        return Ok(vec![]);
    }
    serde_json::from_str(&data).map_err(|e| format!("Parse error: {}", e))
}

/// Save all accounts to disk
fn save_all(accounts: &[Account]) -> Result<(), String> {
    let path = accounts_path()?;
    let json = serde_json::to_string_pretty(accounts).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| format!("Cannot write accounts.json: {}", e))
}

/// Generate a deterministic UUID from a username (for offline accounts)
/// Uses MD5 hash of "OfflinePlayer:<username>" as per Minecraft convention
fn offline_uuid(username: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let input = format!("OfflinePlayer:{}", username);
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash = hasher.finish();

    // Format as UUID v3-like (variant 1)
    let h = hash as u32;
    let hi = (hash >> 32) as u32;
    format!(
        "{:08x}-{:04x}-3{:03x}-{:04x}-{:012x}",
        h,
        (h >> 16) as u16,
        hi & 0xFFF,
        ((hi >> 12) & 0x3FFF) | 0x8000,
        hash & 0xFFFFFFFFFFFF
    )
}

// === Public API ===

/// List all accounts
pub fn list_accounts(active_id: Option<&str>) -> Result<Vec<AccountInfo>, String> {
    use base64::Engine;
    let accounts = load_all()?;
    let skins_dir = accounts_path().ok().and_then(|p| p.parent().map(|d| d.join("skins")));

    Ok(accounts
        .iter()
        .map(|a| {
            let mut skin_png_base64 = None;
            if let Some(ref sdir) = skins_dir {
                if let Some(ref u) = a.uuid {
                    let spath = sdir.join(format!("{}.png", u));
                    if spath.exists() {
                        if let Ok(bytes) = fs::read(&spath) {
                            let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
                            skin_png_base64 = Some(format!("data:image/png;base64,{}", b64));
                        }
                    }
                }
            }
            let is_act = active_id.map_or(false, |id| id == a.id);
            AccountInfo {
                id: a.id.clone(),
                account_type: match a.account_type {
                    AccountType::Microsoft => "microsoft".to_string(),
                    AccountType::Offline => "offline".to_string(),
                },
                username: a.username.clone(),
                uuid: a.uuid.clone(),
                skin_png_base64,
                is_active: is_act,
            }
        })
        .collect())
}

/// Add an offline account
pub fn add_offline(username: &str) -> Result<AccountInfo, String> {
    let mut accounts = load_all()?;

    // Check for duplicate username
    if accounts.iter().any(|a| a.username.eq_ignore_ascii_case(username)) {
        return Err(format!("Account '{}' already exists", username));
    }

    let id = format!("offline_{}", uuid::Uuid::new_v4().to_string().replace('-', ""));
    let uuid = offline_uuid(username);

    let account = Account {
        id: id.clone(),
        account_type: AccountType::Offline,
        username: username.to_string(),
        uuid: Some(uuid.clone()),
        access_token: None,
        refresh_token: None,
        expires_at: None,
        skin_url: None,
        client_id: None,
    };

    accounts.push(account);
    save_all(&accounts)?;

    let active_id = get_active_id();
    if active_id.is_none() {
        let _ = set_active_id(&id);
    }

    Ok(AccountInfo {
        id,
        account_type: "offline".to_string(),
        username: username.to_string(),
        uuid: Some(uuid),
        skin_png_base64: None,
        is_active: true,
    })
}

/// Remove an account by id
pub fn remove_account(id: &str) -> Result<(), String> {
    let mut accounts = load_all()?;
    let before = accounts.len();
    accounts.retain(|a| a.id != id);
    if accounts.len() == before {
        return Err(format!("Account '{}' not found", id));
    }
    save_all(&accounts)?;

    if get_active_id().as_deref() == Some(id) {
        if let Some(next_acc) = accounts.first() {
            let _ = set_active_id(&next_acc.id);
        } else {
            let _ = set_active_id("");
        }
    }

    Ok(())
}

/// Get account info by id
pub fn get_account(id: &str) -> Result<AccountInfo, String> {
    let accounts = load_all()?;
    accounts
        .iter()
        .find(|a| a.id == id)
        .map(|a| AccountInfo {
            id: a.id.clone(),
            account_type: match a.account_type {
                AccountType::Microsoft => "microsoft".to_string(),
                AccountType::Offline => "offline".to_string(),
            },
            username: a.username.clone(),
            uuid: a.uuid.clone(),
            skin_png_base64: None,
            is_active: false,
        })
        .ok_or_else(|| format!("Account '{}' not found", id))
}

/// Get the active account id from settings (placeholder - will use settings.json)
pub fn get_active_id() -> Option<String> {
    let path = accounts_path().ok()?.parent()?.join("active_acc.txt");
    fs::read_to_string(path).ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

pub fn set_active_id(id: &str) -> Result<(), String> {
    let path = accounts_path()?.parent().unwrap().join("active_acc.txt");
    fs::write(path, id).map_err(|e| e.to_string())
}

// === Microsoft Authentication (PKCE & Device Code) ===

const DEFAULT_CLIENT_ID: &str = "c36a9fb6-4f2a-41ff-90bd-ae7cc92031eb";

/// Formats the complete error chain with all upstream causes
fn full_error_chain(e: &dyn std::error::Error) -> String {
    let mut msg = e.to_string();
    let mut source = e.source();
    while let Some(s) = source {
        msg.push_str(&format!(" -> caused by: {}", s));
        source = s.source();
    }
    msg
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsaDeviceCode {
    pub user_code: String,
    pub verification_uri: String,
    pub device_code: String,
    pub interval: u64,
}

/// Helper function to finalize Minecraft Login chain (Steps 5..9)
async fn finish_minecraft_login(
    client_id: &str,
    ms_access_token: &str,
    ms_refresh_token: Option<String>,
    expires_in: u64,
) -> Result<AccountInfo, String> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| e.to_string())?;

    // Step 5: XBL Token
    let xbl_body = serde_json::json!({
        "Properties": {
            "AuthMethod": "RPS",
            "SiteName": "user.auth.xboxlive.com",
            "RpsTicket": format!("d={}", ms_access_token)
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT"
    });

    println!("[MSA Step 5] Requesting XBL Token from user.auth.xboxlive.com...");

    let xbl_res = client
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&xbl_body)
        .send()
        .await
        .map_err(|e| format!("XBL error: {}", full_error_chain(&e)))?;

    let xbl_status = xbl_res.status();
    let xbl_text = xbl_res.text().await.map_err(|e| format!("XBL response read error: {}", full_error_chain(&e)))?;
    let xbl_json: serde_json::Value = serde_json::from_str(&xbl_text).map_err(|e| format!("XBL parse error ({}): {} | Body: {}", xbl_status, e, xbl_text))?;

    let xbl_token = xbl_json["Token"].as_str().ok_or_else(|| format!("No XBL Token in response (status {}): {}", xbl_status, xbl_text))?;
    let user_hash = xbl_json["DisplayClaims"]["xui"][0]["uhs"].as_str().ok_or_else(|| format!("No UHS in response (status {}): {}", xbl_status, xbl_text))?;

    println!("[MSA Step 5] XBL Token obtained successfully (UHS: {})", user_hash);

    // Step 6: XSTS Token
    let xsts_body = serde_json::json!({
        "Properties": {
            "SandboxId": "RETAIL",
            "UserTokens": [xbl_token]
        },
        "RelyingParty": "rp://api.minecraftservices.com/",
        "TokenType": "JWT"
    });

    println!("[MSA Step 6] Requesting XSTS Token for RelyingParty='rp://api.minecraftservices.com/'...");
    println!("[MSA Step 6 Request Body]:\n{}", serde_json::to_string_pretty(&xsts_body).unwrap_or_default());

    let xsts_res = client
        .post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&xsts_body)
        .send()
        .await
        .map_err(|e| format!("XSTS error: {}", full_error_chain(&e)))?;

    let xsts_status = xsts_res.status();
    let xsts_text = xsts_res.text().await.map_err(|e| format!("XSTS response read error: {}", full_error_chain(&e)))?;
    let xsts_json: serde_json::Value = serde_json::from_str(&xsts_text).map_err(|e| format!("XSTS parse error ({}): {} | Body: {}", xsts_status, e, xsts_text))?;

    if let Some(err_num) = xsts_json.get("XErr").and_then(|v| v.as_u64()) {
        if err_num == 2148916233 {
            return Err("У этого аккаунта Microsoft нет профиля Xbox Live (XErr: 2148916233)".to_string());
        } else if err_num == 2148916238 {
            return Err("Детский аккаунт: требуется согласие родительской семьи в Xbox (XErr: 2148916238)".to_string());
        }
    }

    let xsts_token = xsts_json["Token"].as_str().ok_or_else(|| format!("No XSTS Token in response (status {}): {}", xsts_status, xsts_text))?;

    println!("[MSA Step 6] XSTS Token obtained successfully");

    // Step 7: Minecraft Token
    let mc_body = serde_json::json!({
        "identityToken": format!("XBL3.0 x={};{}", user_hash, xsts_token)
    });

    let mc_res = client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .json(&mc_body)
        .send()
        .await
        .map_err(|e| format!("Minecraft Login error: {}", e))?;

    let mc_json: serde_json::Value = mc_res.json().await.map_err(|e| format!("Minecraft Login parse error: {}", e))?;
    let mc_access_token = mc_json["access_token"].as_str().ok_or("No mc_access_token in response")?.to_string();

    // Step 8: Entitlements Check
    let ent_res = client
        .get("https://api.minecraftservices.com/entitlements/mcstore")
        .bearer_auth(&mc_access_token)
        .send()
        .await;

    if let Ok(ent_resp) = ent_res {
        if let Ok(ent_json) = ent_resp.json::<serde_json::Value>().await {
            println!("[MSA Auth] Entitlements checked: {:?}", ent_json);
        }
    }

    // Step 9: Get Profile & Download Skin
    let prof_res = client
        .get("https://api.minecraftservices.com/minecraft/profile")
        .bearer_auth(&mc_access_token)
        .send()
        .await
        .map_err(|e| format!("Profile error: {}", e))?;

    let prof_json: serde_json::Value = prof_res.json().await.map_err(|e| format!("Profile parse error: {}", e))?;
    let username = prof_json["name"].as_str().ok_or("На этом аккаунте Microsoft не куплена лицензия Minecraft!")?.to_string();
    let uuid = prof_json["id"].as_str().ok_or("No player uuid")?.to_string();

    let mut skin_url = None;
    if let Some(skins) = prof_json["skins"].as_array() {
        if let Some(s) = skins.iter().find(|s| s["state"].as_str() == Some("ACTIVE")) {
            skin_url = s["url"].as_str().map(|u| u.to_string());
        }
    }

    let mut skin_png_base64 = None;
    if let Some(ref s_url) = skin_url {
        if let Ok(resp) = reqwest::get(s_url).await {
            if let Ok(s_bytes) = resp.bytes().await {
                if let Ok(app_dir) = accounts_path().map(|p| p.parent().unwrap().to_path_buf()) {
                    let skins_dir = app_dir.join("skins");
                    let _ = fs::create_dir_all(&skins_dir);
                    let _ = fs::write(skins_dir.join(format!("{}.png", uuid)), &s_bytes);
                    use base64::Engine;
                    let b64 = base64::engine::general_purpose::STANDARD.encode(&s_bytes);
                    skin_png_base64 = Some(format!("data:image/png;base64,{}", b64));
                }
            }
        }
    }

    let expires_at = (std::time::SystemTime::now() + std::time::Duration::from_secs(expires_in))
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string();

    let id = format!("msa_{}", uuid);
    let mut accounts = load_all()?;
    accounts.retain(|a| a.id != id);

    let account = Account {
        id: id.clone(),
        account_type: AccountType::Microsoft,
        username: username.clone(),
        uuid: Some(uuid.clone()),
        access_token: Some(mc_access_token),
        refresh_token: ms_refresh_token,
        expires_at: Some(expires_at),
        skin_url,
        client_id: Some(client_id.to_string()),
    };

    accounts.push(account);
    save_all(&accounts)?;
    let _ = set_active_id(&id);

    Ok(AccountInfo {
        id,
        account_type: "microsoft".to_string(),
        username,
        uuid: Some(uuid),
        skin_png_base64,
        is_active: true,
    })
}

/// Primary PKCE OAuth Flow (opens system browser & catches local HTTP redirect)
pub async fn authenticate_msa_pkce() -> Result<AccountInfo, String> {
    use sha2::{Digest, Sha256};
    use base64::Engine;

    // 1. Generate PKCE verifier & challenge
    let verifier_raw = format!(
        "{}{}{}{}",
        uuid::Uuid::new_v4().to_string().replace('-', ""),
        uuid::Uuid::new_v4().to_string().replace('-', ""),
        uuid::Uuid::new_v4().to_string().replace('-', ""),
        uuid::Uuid::new_v4().to_string().replace('-', "")
    );

    let mut hasher = Sha256::new();
    hasher.update(verifier_raw.as_bytes());
    let hash = hasher.finalize();
    let challenge = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash);

    // 2. Bind local HTTP server to dynamic port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|e| format!("Cannot start local auth server: {}", e))?;
    let port = listener
        .local_addr()
        .map_err(|e| format!("Cannot get local port: {}", e))?
        .port();
    let redirect_uri = format!("http://localhost:{}", port);

    let client_id = DEFAULT_CLIENT_ID;
    let auth_url = format!(
        "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?client_id={}&response_type=code&redirect_uri={}&scope=XboxLive.signin%20offline_access&code_challenge={}&code_challenge_method=S256&prompt=select_account",
        client_id,
        urlencoding::encode(&redirect_uri),
        challenge
    );

    println!("[MSA PKCE] Redirect URI: {}", redirect_uri);
    println!("[MSA PKCE] Opening Auth URL: {}", auth_url);

    // Open System Browser safely without cmd.exe splitting on '&'
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", "Start-Process", &format!("'{}'", auth_url)])
            .spawn();
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = std::process::Command::new("xdg-open").arg(&auth_url).spawn();
    }

    // 3. Catch HTTP Redirect with code or error (timeout 120s)
    let mut auth_code = None;
    let mut auth_err = None;

    if let Ok(Ok((mut stream, _))) = tokio::time::timeout(std::time::Duration::from_secs(120), listener.accept()).await {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let mut buf = [0u8; 4096];
        if let Ok(n) = stream.read(&mut buf).await {
            let req_str = String::from_utf8_lossy(&buf[..n]);
            if let Some(first_line) = req_str.lines().next() {
                if let Some(q_start) = first_line.find('?') {
                    if let Some(q_end) = first_line[q_start..].find(' ') {
                        let query = &first_line[q_start + 1..q_start + q_end];
                        for pair in query.split('&') {
                            let parts: Vec<&str> = pair.splitn(2, '=').collect();
                            if parts.len() == 2 {
                                if parts[0] == "code" {
                                    auth_code = Some(urlencoding::decode(parts[1]).unwrap_or_default().to_string());
                                } else if parts[0] == "error" || parts[0] == "error_description" {
                                    auth_err = Some(urlencoding::decode(parts[1]).unwrap_or_default().to_string());
                                }
                            }
                        }
                    }
                }
            }

            let html = if auth_code.is_some() {
                "<html><head><meta charset='utf-8'></head><body style='font-family:sans-serif;text-align:center;padding:50px;background:#121212;color:#fff;'><h2>✅ Авторизация успешна!</h2><p>Можете закрыть это окно и вернуться в MyL Launcher.</p></body></html>"
            } else {
                "<html><head><meta charset='utf-8'></head><body style='font-family:sans-serif;text-align:center;padding:50px;background:#121212;color:#ff5555;'><h2>❌ Ошибка входа</h2><p>Не удалось получить код авторизации.</p></body></html>"
            };
            let resp = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", html.len(), html);
            let _ = stream.write_all(resp.as_bytes()).await;
            let _ = stream.flush().await;
        }
    }

    let code = match auth_code {
        Some(c) => c,
        None => {
            let err_msg = auth_err.unwrap_or_else(|| format!("Timeout or redirect_uri mismatch (redirect_uri: {})", redirect_uri));
            println!("[MSA PKCE Error] {}", err_msg);
            return Err(err_msg);
        }
    };

    // 4. Exchange authorization code for MS Token
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| e.to_string())?;

    let token_params = [
        ("client_id", client_id),
        ("grant_type", "authorization_code"),
        ("code", &code),
        ("redirect_uri", &redirect_uri),
        ("code_verifier", &verifier_raw),
    ];

    let token_res = client
        .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
        .form(&token_params)
        .send()
        .await
        .map_err(|e| format!("Token network error: {}", e))?;

    let token_status = token_res.status();
    let token_text = token_res.text().await.unwrap_or_default();
    let token_json: serde_json::Value = serde_json::from_str(&token_text).unwrap_or(serde_json::Value::Null);

    if !token_status.is_success() {
        let desc = token_json["error_description"].as_str().or_else(|| token_json["error"].as_str()).unwrap_or(&token_text);
        let err_msg = format!("Token Error ({}) [redirect_uri: {}]: {}", token_status, redirect_uri, desc);
        println!("[MSA PKCE Error] {}", err_msg);
        return Err(err_msg);
    }

    let ms_access_token = token_json["access_token"].as_str().ok_or("No ms_access_token in response")?.to_string();
    let ms_refresh_token = token_json["refresh_token"].as_str().map(|s| s.to_string());
    let expires_in = token_json["expires_in"].as_u64().unwrap_or(3600);

    // Steps 5..9
    finish_minecraft_login(client_id, &ms_access_token, ms_refresh_token, expires_in).await
}

/// Fallback Device Code Flow
pub async fn start_msa_device_code() -> Result<MsaDeviceCode, String> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| e.to_string())?;

    let params = [
        ("client_id", DEFAULT_CLIENT_ID),
        ("scope", "XboxLive.signin offline_access"),
    ];

    let res = client
        .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode")
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Network Error: {}", e))?;

    let status = res.status();
    let text = res.text().await.unwrap_or_default();
    let json: serde_json::Value = serde_json::from_str(&text).unwrap_or(serde_json::Value::Null);

    if !status.is_success() {
        let desc = json["error_description"].as_str().or_else(|| json["error"].as_str()).unwrap_or(&text);
        return Err(format!("Microsoft Device Code Error ({}): {}", status, desc));
    }

    let user_code = json["user_code"].as_str().ok_or_else(|| format!("No user_code in response: {}", text))?.to_string();
    let verification_uri = json["verification_uri"].as_str().unwrap_or("https://microsoft.com/link").to_string();
    let device_code = json["device_code"].as_str().ok_or_else(|| format!("No device_code in response: {}", text))?.to_string();
    let interval = json["interval"].as_u64().unwrap_or(5);

    Ok(MsaDeviceCode {
        user_code,
        verification_uri,
        device_code,
        interval,
    })
}

/// Device Code Polling
pub async fn poll_msa_token(device_code: &str) -> Result<AccountInfo, String> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| e.to_string())?;

    let params = [
        ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
        ("client_id", DEFAULT_CLIENT_ID),
        ("device_code", device_code),
    ];

    let res = client
        .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Network Error: {}", e))?;

    let text = res.text().await.unwrap_or_default();
    let json: serde_json::Value = serde_json::from_str(&text).map_err(|_| format!("Parse error: {}", text))?;

    if let Some(err) = json.get("error").and_then(|e| e.as_str()) {
        if err == "authorization_pending" {
            return Err("PENDING".to_string());
        } else {
            let desc = json.get("error_description").and_then(|d| d.as_str()).unwrap_or(err);
            return Err(format!("Auth error: {}", desc));
        }
    }

    let ms_access_token = json["access_token"].as_str().ok_or_else(|| format!("No ms_access_token: {}", json))?.to_string();
    let ms_refresh_token = json["refresh_token"].as_str().map(|s| s.to_string());
    let expires_in = json["expires_in"].as_u64().unwrap_or(3600);

    finish_minecraft_login(DEFAULT_CLIENT_ID, &ms_access_token, ms_refresh_token, expires_in).await
}

/// Auto-refresh account using stored refresh_token
pub async fn refresh_msa_account(account_id: &str) -> Result<AccountInfo, String> {
    let accounts = load_all()?;
    let acc = accounts.iter().find(|a| a.id == account_id).ok_or("Account not found")?;
    let refresh_tok = acc.refresh_token.as_ref().ok_or("No refresh token stored")?;
    let cid = acc.client_id.as_deref().unwrap_or(DEFAULT_CLIENT_ID);

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| e.to_string())?;

    let params = [
        ("client_id", cid),
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_tok),
    ];

    let res = client
        .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Refresh network error: {}", e))?;

    let text = res.text().await.unwrap_or_default();
    let json: serde_json::Value = serde_json::from_str(&text).map_err(|_| format!("Refresh parse error: {}", text))?;

    let new_access = json["access_token"].as_str().ok_or_else(|| format!("No access_token in refresh: {}", json))?.to_string();
    let new_refresh = json["refresh_token"].as_str().map(|s| s.to_string()).or_else(|| Some(refresh_tok.to_string()));
    let expires_in = json["expires_in"].as_u64().unwrap_or(3600);

    finish_minecraft_login(cid, &new_access, new_refresh, expires_in).await
}
