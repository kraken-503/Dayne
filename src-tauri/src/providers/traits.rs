use async_trait::async_trait;
use std::path::PathBuf;
use crate::models::wallpaper::CatalogManifest;

#[derive(Debug, PartialEq)]
pub enum ManifestFetchStatus {
    Updated(CatalogManifest, String),
    NotModified,
}

#[async_trait]
pub trait WallpaperProvider: Send + Sync {
    async fn fetch_manifest(&self, current_etag: Option<&str>) -> Result<ManifestFetchStatus, String>;
    async fn download_image(&self, url: &str, destination: PathBuf) -> Result<PathBuf, String>;
}
