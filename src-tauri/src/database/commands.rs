use tauri::State;
use crate::database::AppState;
use crate::database::repository;

#[tauri::command]
pub async fn initialize_database(state: State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    repository::init_database(&*db).map_err(|e| e.to_string())
}
