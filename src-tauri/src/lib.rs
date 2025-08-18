mod character;
mod database;
mod error;
mod import;
mod metatype;
mod shared;

use crate::database::AppState;
use std::fs::{create_dir_all, remove_file};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Setup
            let app_dir = app.path().app_data_dir()?;
            create_dir_all(&app_dir)?;

            // Initialize database
            let db_path = app_dir.join("chumper.db3");
            if db_path.exists() {
                remove_file(&db_path)?;
            }
            let state = AppState::new(&db_path)?;
            app.manage(state);

            // Setup services
            // let character_service = CharacterService::new(app.handle().clone());

            // Initialize Services
            // app.manage(character_service);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            character::commands::list_characters,
            character::commands::get_character,
            character::commands::create_character,
            character::commands::import_character,
            character::commands::export_character,
            metatype::commands::list_metatypes,
            metatype::commands::get_metatype,
            metatype::commands::create_metatype,
            database::commands::initialize_database
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
