use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;

use crate::models::wallpaper::CatalogManifest;
use crate::providers::traits::{ManifestFetchStatus, WallpaperProvider};

pub struct GitHubWallpaperProvider {
    client: reqwest::Client,
    manifest_url: String,
}

impl GitHubWallpaperProvider {
    pub fn new(manifest_url: String) -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("Dayne-Wallpaper-App/1.0")
                .build()
                .unwrap_or_default(),
            manifest_url,
        }
    }
}

#[async_trait]
impl WallpaperProvider for GitHubWallpaperProvider {
    async fn fetch_manifest(&self, current_etag: Option<&str>) -> Result<ManifestFetchStatus, String> {
        let mut req = self.client.get(&self.manifest_url);
        if let Some(etag) = current_etag {
            req = req.header("If-None-Match", etag);
        }

        let resp = req.send().await.map_err(|e| format!("Network error: {}", e))?;

        if resp.status() == reqwest::StatusCode::NOT_MODIFIED {
            return Ok(ManifestFetchStatus::NotModified);
        }

        if !resp.status().is_success() {
            return Err(format!("GitHub server returned HTTP status {}", resp.status()));
        }

        let etag = resp.headers()
            .get("etag")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("")
            .to_string();

        let manifest: CatalogManifest = resp.json().await
            .map_err(|e| format!("Failed to parse GitHub JSON manifest: {}", e))?;

        Ok(ManifestFetchStatus::Updated(manifest, etag))
    }

    async fn download_image(&self, url: &str, destination: PathBuf) -> Result<PathBuf, String> {
        if destination.exists() {
            return Ok(destination);
        }

        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).await.map_err(|e| e.to_string())?;
        }

        let resp = self.client.get(url).send().await.map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Err(format!("Image download error HTTP {}", resp.status()));
        }

        let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
        fs::write(&destination, &bytes).await.map_err(|e| e.to_string())?;

        Ok(destination)
    }
}
