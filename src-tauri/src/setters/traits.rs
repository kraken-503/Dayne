use std::path::Path;

pub trait WallpaperSetter: Send + Sync {
    fn is_supported(&self) -> bool;
    fn set_wallpaper(&self, path: &Path) -> Result<(), String>;
}
