use tauri::State;
use crate::database::AppState;
use crate::character::{repository, types::*};
use serde_yml;

#[tauri::command]
pub async fn list_characters(state: State<'_, AppState>) -> Result<Vec<CharacterSummary>, String> {
    let db = state.db.lock().unwrap();
    repository::list_characters(&*db).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_character(id: i64, state: State<'_, AppState>) -> Result<Character, String> {
    let db = state.db.lock().unwrap();
    repository::get_character_by_id(&*db, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_character(character: Character, state: State<'_, AppState>) -> Result<i64, String> {
    let db = state.db.lock().unwrap();
    repository::create_character(&*db, &character).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_character(yaml: String, state: State<'_, AppState>) -> Result<i64, String> {
    let mut character: Character = serde_yml::from_str(&yaml)
        .map_err(|e| format!("YAML parse error: {}", e))?;

    character.id = None;
    let db = state.db.lock().unwrap();
    repository::create_character(&*db, &character).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn export_character(id: i64, state: State<'_, AppState>) -> Result<String, String> {
    let db = state.db.lock().unwrap();
    let character = repository::get_character_by_id(&*db, id).map_err(|e| e.to_string())?;
    serde_yml::to_string(&character).map_err(|e| e.to_string())
}
