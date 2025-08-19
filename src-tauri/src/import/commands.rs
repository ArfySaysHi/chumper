use super::repository::import_yaml_file;
use crate::character::Character;
use crate::database::AppState;
use crate::metatype::types::Metatype;
use std::path::Path;
use std::result::Result;
use tauri::State;

#[tauri::command]
pub fn import_characters(path: &Path, state: State<'_, AppState>) -> Result<String, String> {
    let connection = state.db.lock().unwrap();
    import_yaml_file::<Character>(&connection, &path).map_err(|e| e.to_string())?;
    Ok("Characters imported successfully".to_string())
}

#[tauri::command]
pub fn import_metatypes(path: &Path, state: State<'_, AppState>) -> Result<String, String> {
    let connection = state.db.lock().unwrap();
    import_yaml_file::<Metatype>(&connection, &path).map_err(|e| e.to_string())?;
    Ok("Metatypes imported successfully".to_string())
}
