use std::path::PathBuf;
use crate::models::wallpaper::CatalogManifest;
use crate::providers::github::GitHubWallpaperProvider;
use crate::providers::traits::{ManifestFetchStatus, WallpaperProvider};
use crate::setters::get_platform_setter;

// Replace with your actual GitHub username, repo name, and branch
const GITHUB_MANIFEST_URL: &str = "https://raw.githubusercontent.com/kraken-503/dayne-src/main/manifest.json";

#[tauri::command]
pub async fn fetch_catalog() -> Result<CatalogManifest, String> {
    let provider = GitHubWallpaperProvider::new(GITHUB_MANIFEST_URL.to_string());

    match provider.fetch_manifest(None).await? {
        ManifestFetchStatus::Updated(manifest, _etag) => Ok(manifest),
        ManifestFetchStatus::NotModified => Err("Manifest not modified".to_string()),
    }
}

#[tauri::command]
pub async fn apply_wallpaper(file_path: String) -> Result<(), String> {
    let path = PathBuf::from(&file_path);
    let setter = get_platform_setter();

    if !setter.is_supported() {
        return Err("Setting wallpaper is not natively supported on this system environment.".into());
    }

    setter.set_wallpaper(&path)
}
