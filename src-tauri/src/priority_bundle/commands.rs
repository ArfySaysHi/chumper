use tauri::State;
use crate::database::AppState;
use super::{repository::ListPriorityBundlesParams, PriorityBundle};

#[tauri::command]
pub async fn list_priority_bundles(params: ListPriorityBundlesParams, state: State<'_, AppState>) -> Result<Vec<PriorityBundle>, String> {
    log::info!("list_priority_bundles");
    let pool = state.db_pool.clone();

    tokio::task::spawn_blocking(move || {
        let connection = pool.get().map_err(|e| e.to_string())?;
        super::repository::list_priority_bundles(&connection, &params).map_err(|e| e.to_string())
    }).await.map_err(|e| e.to_string())?
}
