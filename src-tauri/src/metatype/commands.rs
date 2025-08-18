use tauri::State;
use crate::AppState;
use crate::metatype::{types::*, repository};

#[tauri::command]
pub async fn list_metatypes(state: State<'_, AppState>) -> Result<Vec<MetatypeSummary>, String> {
    let db = state.db.lock().unwrap();
    repository::list_metatypes(&*db).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_metatype(name: &str, state: State<'_, AppState>) -> Result<Metatype, String> {
    let db = state.db.lock().unwrap();
    repository::get_metatype(&*db, name).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_metatype(metatype: Metatype, state: State<'_, AppState>) -> Result<i64, String> {
    let db = state.db.lock().unwrap();
    repository::create_metatype(&*db, metatype).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_metatype(yaml: &str, state: State<'_, AppState>) -> Result<i64, String> {
    let db = state.db.lock().unwrap();
    repository::import_metatype(&*db, yaml).map_err(|e| e.to_string())
}
