mod character;
mod database;
mod error;
mod import;
mod metatype;
mod modifier;
mod shared;

use crate::database::AppState;
use std::fs::{create_dir_all, remove_file};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    log::info!("Starting Chumper...");
    tauri::Builder::default()
        .setup(|app| {
            log::info!("Initializing database 'chumper.db3'...");
            // Setup
            let app_dir = app.path().app_data_dir()?;
            create_dir_all(&app_dir)?;

            // Initialize database
            let db_path = app_dir.join("chumper.db3");
            if db_path.exists() {
                remove_file(&db_path)?;
            }

            let state = AppState::new(&db_path, &app)?;
            app.manage(state);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            character::commands::list_characters,
            character::commands::get_character,
            character::commands::create_character,
            character::commands::import_character,
            character::commands::export_character,
            character::commands::delete_character,
            character::commands::archive_character,
            character::resource::commands::list_character_resources,
            modifier::commands::list_modifiers,
            metatype::commands::list_metatypes,
            metatype::commands::get_metatype,
            metatype::commands::create_metatype,
            metatype::commands::import_metatype,
            database::commands::initialize_database,
            import::commands::import_characters,
            import::commands::import_metatypes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
