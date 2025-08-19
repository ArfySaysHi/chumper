use tauri::State;
use crate::database::{repository, AppState};
use tokio::task::spawn_blocking;

#[tauri::command]
pub async fn initialize_database(state: State<'_, AppState>) -> Result<(), String> {
    let pool = state.db_pool.clone();

    spawn_blocking(move || {
        let connection = pool.get().map_err(|e| e.to_string())?;
        repository::init_database(&connection).map_err(|e| e.to_string())
    }).await.map_err(|e| e.to_string())?
}
