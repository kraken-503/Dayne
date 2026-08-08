use std::path::Path;
use std::process::Command;
use crate::setters::traits::WallpaperSetter;

pub struct LinuxWallpaperSetter;

impl WallpaperSetter for LinuxWallpaperSetter {
    fn is_supported(&self) -> bool {
        cfg!(target_os = "linux") && std::env::var("XDG_CURRENT_DESKTOP").is_ok()
    }

    fn set_wallpaper(&self, path: &Path) -> Result<(), String> {
        let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default().to_lowercase();
        let path_str = path.to_str().ok_or("Invalid path encoding")?;

        if desktop.contains("gnome") || desktop.contains("ubuntu") {
            let uri = format!("file://{}", path_str);
            let status = Command::new("gsettings")
                .args(["set", "org.gnome.desktop.background", "picture-uri", &uri])
                .status()
                .map_err(|e| e.to_string())?;

            let _ = Command::new("gsettings")
                .args(["set", "org.gnome.desktop.background", "picture-uri-dark", &uri])
                .status();

            if status.success() {
                return Ok(());
            }
        } else if desktop.contains("kde") {
            let script = format!(
                r#"var allDesktops = desktops();
                for (i=0;i<allDesktops.length;i++) {{
                    d = allDesktops[i];
                    d.wallpaperPlugin = "org.kde.image";
                    d.currentConfigGroup = Array("Wallpaper", "org.kde.image", "General");
                    d.writeConfig("Image", "file://{}");
                }}"#,
                path_str
            );

            let status = Command::new("qdbus")
                .args(["org.kde.plasmashell", "/PlasmaShell", "org.kde.PlasmaShell.evaluateScript", &script])
                .status()
                .map_err(|e| e.to_string())?;

            if status.success() {
                return Ok(());
            }
        }

        Err("No supported Linux desktop wallpaper setter found for current environment.".into())
    }
}
