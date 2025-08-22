use super::Resource;
use crate::character::resource::repository;
use crate::database::AppState;
use tauri::State;

#[tauri::command]
pub async fn list_character_resources(id: i64, state: State<'_, AppState>) -> Result<Vec<Resource>, String> {
    let pool = state.db_pool.clone();

    tokio::task::spawn_blocking(move || {
        let connection = pool.get().map_err(|e| e.to_string())?;
        repository::list_resources(&connection, id).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}
