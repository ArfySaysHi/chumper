use crate::database::AppState;
use tauri::State;

#[tauri::command]
pub async fn list_priority_grades(state: State<'_, AppState>) -> Result<(), String> {
    log::info!("list_priority_grades");
    let pool = state.db_pool.clone();

    tokio::task::spawn_blocking(move || {
        let mut connection = pool.get().map_err(|e| e.to_string())?;
        super::repository::list_priority_grades(&mut connection).map_err(|e| e.to_string())
    }).await.map_err(|e| e.to_string())?
}
