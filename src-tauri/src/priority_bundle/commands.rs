use tauri::State;
use crate::{database::AppState, shared::Grade};
use super::{repository::PriorityBundleListParams, PriorityBundle};
use std::collections::HashMap;

#[tauri::command]
pub async fn list_priority_bundles(params: PriorityBundleListParams, state: State<'_, AppState>) -> Result<Vec<PriorityBundle>, String> {
    log::info!("list_priority_bundles");
    let pool = state.db_pool.clone();

    tokio::task::spawn_blocking(move || {
        let connection = pool.get().map_err(|e| e.to_string())?;
        super::repository::list_priority_bundles(&connection, &params).map_err(|e| e.to_string())
    }).await.map_err(|e| e.to_string())?
}


#[tauri::command]
pub async fn list_priority_bundles_by_grade(params: PriorityBundleListParams, state: State<'_, AppState>) -> Result<HashMap<Grade, Vec<PriorityBundle>>, String> {
    log::info!("list_priority_bundles");
    let pool = state.db_pool.clone();

    tokio::task::spawn_blocking(move || {
        let connection = pool.get().map_err(|e| e.to_string())?;
        super::repository::list_by_grade(&connection, &params).map_err(|e| e.to_string())
    }).await.map_err(|e| e.to_string())?
}
