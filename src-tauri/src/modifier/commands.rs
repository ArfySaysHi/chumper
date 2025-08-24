use tauri::State;
use crate::modifier::Modifier;
use crate::database::AppState;
use super::repository;

#[tauri::command]
pub async fn list_modifiers(character_id: i64, state: State<'_, AppState>) -> Result<Vec<Modifier>, String> {
    log::info!("list_modifiers with {:?}", &character_id);
    let pool = state.db_pool.clone();

    tokio::task::spawn_blocking(move || {
        let mut connection = pool.get().map_err(|e| e.to_string())?;
        repository::list_modifiers(&mut connection, character_id).map_err(|e| e.to_string())
    }).await.map_err(|e| e.to_string())?
}
