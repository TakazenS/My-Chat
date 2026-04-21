mod db;
mod auth;

use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let conn = db::init_db(&app.handle())?;
            app.manage(db::DbState(Mutex::new(conn)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            auth::register,
            auth::login,
            auth::logout,
            auth::get_current_session
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
