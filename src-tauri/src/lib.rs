pub mod models;
pub mod providers;
pub mod setters;
pub mod commands;

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::fetch_catalog,
            commands::apply_wallpaper
        ])
        .run(tauri::generate_context!())
        .expect("error while running Dayne application");
}
