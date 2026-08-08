use std::path::Path;
use crate::setters::traits::WallpaperSetter;

pub struct WindowsWallpaperSetter;

impl WallpaperSetter for WindowsWallpaperSetter {
    fn is_supported(&self) -> bool {
        cfg!(target_os = "windows")
    }

    fn set_wallpaper(&self, path: &Path) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            use std::ffi::OsStr;
            use std::os::windows::ffi::OsStrExt;
            use winapi::um::winuser::{SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER};

            let wide_path: Vec<u16> = OsStr::new(path)
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();

            let res = unsafe {
                SystemParametersInfoW(
                    SPI_SETDESKWALLPAPER,
                    0,
                    wide_path.as_ptr() as *mut _,
                    SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
                )
            };

            if res == 0 {
                return Err("Failed to set Windows desktop wallpaper via SystemParametersInfoW API.".into());
            }
            Ok(())
        }

        #[cfg(not(target_os = "windows"))]
        {
            let _ = path;
            Err("Windows wallpaper setter invoked on a non-Windows target OS.".into())
        }
    }
}
