mod commands;
mod controllers;
mod models;
mod utils;

use commands::*;
use controllers::character_controller::CharacterController;
use controllers::database_controller::DatabaseController;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let character_controller = CharacterController::new().unwrap();
    let database_controller = DatabaseController::new().unwrap();

    tauri::Builder::default()
        .manage(character_controller)
        .manage(database_controller)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_characters,
            create_character,
            import_character,
            init_database
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
