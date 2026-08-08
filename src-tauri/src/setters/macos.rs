use std::path::Path;
use std::process::Command;
use crate::setters::traits::WallpaperSetter;

pub struct MacOSWallpaperSetter;

impl WallpaperSetter for MacOSWallpaperSetter {
    fn is_supported(&self) -> bool {
        cfg!(target_os = "macos")
    }

    fn set_wallpaper(&self, path: &Path) -> Result<(), String> {
        let script = format!(
            "tell application \"System Events\" to set picture of every desktop to \"{}\"",
            path.display()
        );

        let status = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .status()
            .map_err(|e| e.to_string())?;

        if status.success() {
            Ok(())
        } else {
            Err("AppleScript failed to change desktop wallpaper.".into())
        }
    }
}
