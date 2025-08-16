mod commands;
mod controllers;
mod models;
mod traits;
mod utils;

use commands::*;
use controllers::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let character_controller = CharacterController::new().unwrap();
    let database_controller = DatabaseController::new().unwrap();
    let metatype_controller = MetatypeController::new().unwrap();

    tauri::Builder::default()
        .manage(character_controller)
        .manage(database_controller)
        .manage(metatype_controller)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_characters,
            get_character,
            create_character,
            import_character,
            export_character,
            init_database,
            get_metatype,
            list_metatypes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
