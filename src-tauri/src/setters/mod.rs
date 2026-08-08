pub mod traits;
pub mod windows;
pub mod linux;
pub mod macos;

use traits::WallpaperSetter;
use windows::WindowsWallpaperSetter;
use linux::LinuxWallpaperSetter;
use macos::MacOSWallpaperSetter;

pub fn get_platform_setter() -> Box<dyn WallpaperSetter> {
    if cfg!(target_os = "windows") {
        Box::new(WindowsWallpaperSetter)
    } else if cfg!(target_os = "linux") {
        Box::new(LinuxWallpaperSetter)
    } else {
        Box::new(MacOSWallpaperSetter)
    }
}
