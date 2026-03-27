use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::auth::TokenStore;

const DRIVE_FILES_URL: &str = "https://www.googleapis.com/drive/v3/files";
const DRIVE_UPLOAD_URL: &str = "https://www.googleapis.com/upload/drive/v3/files";
const TODO_FILE_NAME: &str = "todo-tui.txt";

#[derive(Debug, Deserialize)]
struct FileListResponse {
    files: Vec<FileMeta>,
}

#[derive(Debug, Deserialize)]
struct FileMeta {
    id: String,
    #[serde(rename = "modifiedTime")]
    modified_time: Option<DateTime<Utc>>,
}

#[derive(Clone)]
pub struct DriveClient {
    pub(crate) http: reqwest::Client,
    pub(crate) tokens: TokenStore,
}

impl DriveClient {
    pub fn new(tokens: TokenStore) -> Self {
        Self {
            http: reqwest::Client::new(),
            tokens,
        }
    }

    /// Obtain (and if necessary interactively request) a valid access token.
    /// Call this before ratatui initialises to ensure any browser-based auth
    /// prompt happens while the terminal is still in normal mode.
    pub async fn ensure_authenticated(&mut self) -> Result<(), String> {
        self.tokens.get_access_token(&self.http).await?;
        Ok(())
    }

    /// Find the todo-tui.txt file in Drive, creating it if it doesn't exist.
    /// Returns the file ID.
    pub async fn find_or_create_file(&mut self) -> Result<String, String> {
        let token = self.tokens.get_access_token(&self.http).await?;

        // Search for existing file
        let resp = self
            .http
            .get(DRIVE_FILES_URL)
            .bearer_auth(&token)
            .query(&[
                ("q", format!("name='{TODO_FILE_NAME}' and trashed=false")),
                ("fields", "files(id,modifiedTime)".to_string()),
                ("spaces", "drive".to_string()),
            ])
            .send()
            .await
            .map_err(|e| format!("Drive list error: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Drive list failed ({status}): {body}"));
        }

        let list: FileListResponse = resp.json().await.map_err(|e| e.to_string())?;

        if let Some(f) = list.files.into_iter().next() {
            return Ok(f.id);
        }

        // Create the file
        let token = self.tokens.get_access_token(&self.http).await?;
        let meta = serde_json::json!({ "name": TODO_FILE_NAME });
        let resp = self
            .http
            .post(format!("{DRIVE_UPLOAD_URL}?uploadType=multipart"))
            .bearer_auth(&token)
            .multipart(
                reqwest::multipart::Form::new()
                    .part(
                        "metadata",
                        reqwest::multipart::Part::text(meta.to_string())
                            .mime_str("application/json")
                            .map_err(|e| e.to_string())?,
                    )
                    .part(
                        "media",
                        reqwest::multipart::Part::text("")
                            .mime_str("text/plain")
                            .map_err(|e| e.to_string())?,
                    ),
            )
            .send()
            .await
            .map_err(|e| format!("Drive create error: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Drive create failed ({status}): {body}"));
        }

        let meta: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        meta["id"]
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| "Drive create: no id in response".to_string())
    }

    /// Get the last-modified time of a file.
    pub async fn get_modified_time(&mut self, file_id: &str) -> Result<DateTime<Utc>, String> {
        let token = self.tokens.get_access_token(&self.http).await?;
        let resp = self
            .http
            .get(format!("{DRIVE_FILES_URL}/{file_id}"))
            .bearer_auth(&token)
            .query(&[("fields", "modifiedTime")])
            .send()
            .await
            .map_err(|e| format!("Drive metadata error: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Drive metadata failed ({status}): {body}"));
        }

        let meta: FileMeta = resp.json().await.map_err(|e| e.to_string())?;
        meta.modified_time
            .ok_or_else(|| "Drive metadata: missing modifiedTime".to_string())
    }

    /// Download the full text content of a file.
    pub async fn download(&mut self, file_id: &str) -> Result<String, String> {
        let token = self.tokens.get_access_token(&self.http).await?;
        let resp = self
            .http
            .get(format!("{DRIVE_FILES_URL}/{file_id}"))
            .bearer_auth(&token)
            .query(&[("alt", "media")])
            .send()
            .await
            .map_err(|e| format!("Drive download error: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Drive download failed ({status}): {body}"));
        }

        resp.text().await.map_err(|e| e.to_string())
    }

    /// Upload (overwrite) the content of an existing file.
    pub async fn upload(&mut self, file_id: &str, content: &str) -> Result<(), String> {
        let token = self.tokens.get_access_token(&self.http).await?;
        let resp = self
            .http
            .patch(format!("{DRIVE_UPLOAD_URL}/{file_id}?uploadType=media"))
            .bearer_auth(&token)
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(content.to_string())
            .send()
            .await
            .map_err(|e| format!("Drive upload error: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Drive upload failed ({status}): {body}"));
        }

        Ok(())
    }
}
