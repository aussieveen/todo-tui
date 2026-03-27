use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/auth";
const DRIVE_SCOPE: &str = "https://www.googleapis.com/auth/drive.file";

#[derive(Debug, Deserialize)]
struct CredentialsFile {
    installed: InstalledCredentials,
}

#[derive(Debug, Deserialize)]
struct InstalledCredentials {
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenData {
    access_token: String,
    refresh_token: Option<String>,
    /// Unix timestamp when the access token expires.
    expires_at: i64,
}

#[derive(Debug, Clone)]
pub struct TokenStore {
    token_path: PathBuf,
    #[allow(dead_code)]
    credentials_path: PathBuf,
    token: Option<TokenData>,
    client_id: String,
    client_secret: String,
}

impl TokenStore {
    /// Load credentials and existing token (if any).
    pub fn load(credentials_path: &Path, token_path: &Path) -> Result<Self, String> {
        let creds_content = fs::read_to_string(credentials_path)
            .map_err(|e| format!("Cannot read credentials.json: {e}"))?;
        let creds: CredentialsFile = serde_json::from_str(&creds_content)
            .map_err(|e| format!("Invalid credentials.json: {e}"))?;

        let token = fs::read_to_string(token_path)
            .ok()
            .and_then(|c| serde_json::from_str::<TokenData>(&c).ok());

        Ok(Self {
            token_path: token_path.to_path_buf(),
            credentials_path: credentials_path.to_path_buf(),
            token,
            client_id: creds.installed.client_id,
            client_secret: creds.installed.client_secret,
        })
    }

    /// Return a valid Bearer token, refreshing or prompting for auth as needed.
    pub async fn get_access_token(&mut self, http: &reqwest::Client) -> Result<String, String> {
        if let Some(tok) = &self.token {
            let now = unix_now();
            if tok.expires_at > now + 60 {
                return Ok(tok.access_token.clone());
            }
            // Try refresh
            if let Some(refresh_token) = tok.refresh_token.clone() {
                match self.refresh(http, &refresh_token).await {
                    Ok(access) => return Ok(access),
                    Err(e) => eprintln!("Token refresh failed ({e}), re-authorising…"),
                }
            }
        }

        // First-run or refresh failed: prompt user
        self.authorize(http).await
    }

    async fn refresh(
        &mut self,
        http: &reqwest::Client,
        refresh_token: &str,
    ) -> Result<String, String> {
        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ];
        let resp: serde_json::Value = http
            .post(TOKEN_URL)
            .form(&params)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())?;

        let access_token = resp["access_token"]
            .as_str()
            .ok_or("missing access_token")?
            .to_string();
        let expires_in = resp["expires_in"].as_i64().unwrap_or(3600);
        let expires_at = unix_now() + expires_in;

        let new_tok = TokenData {
            access_token: access_token.clone(),
            refresh_token: Some(refresh_token.to_string()),
            expires_at,
        };
        self.token = Some(new_tok.clone());
        self.persist(&new_tok)?;
        Ok(access_token)
    }

    async fn authorize(&mut self, http: &reqwest::Client) -> Result<String, String> {
        let auth_url = format!(
            "{AUTH_URL}?client_id={}&redirect_uri=urn:ietf:wg:oauth:2.0:oob\
             &response_type=code&scope={}&access_type=offline&prompt=consent",
            urlencoded(&self.client_id),
            urlencoded(DRIVE_SCOPE),
        );

        eprintln!("\n=== Google Drive authorisation required ===");
        eprintln!("Open this URL in your browser:\n\n  {auth_url}\n");
        eprint!("Paste the authorisation code here: ");
        io::stdout().flush().ok();

        let mut code = String::new();
        io::stdin()
            .read_line(&mut code)
            .map_err(|e| e.to_string())?;
        let code = code.trim().to_string();

        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("code", code.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", "urn:ietf:wg:oauth:2.0:oob"),
        ];
        let resp: serde_json::Value = http
            .post(TOKEN_URL)
            .form(&params)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())?;

        let access_token = resp["access_token"]
            .as_str()
            .ok_or("missing access_token in response")?
            .to_string();
        let refresh_token = resp["refresh_token"].as_str().map(str::to_string);
        let expires_in = resp["expires_in"].as_i64().unwrap_or(3600);

        let tok = TokenData {
            access_token: access_token.clone(),
            refresh_token,
            expires_at: unix_now() + expires_in,
        };
        self.token = Some(tok.clone());
        self.persist(&tok)?;
        Ok(access_token)
    }

    fn persist(&self, tok: &TokenData) -> Result<(), String> {
        let content = serde_json::to_string_pretty(tok).map_err(|e| e.to_string())?;
        fs::write(&self.token_path, content).map_err(|e| e.to_string())
    }
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

fn urlencoded(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}
