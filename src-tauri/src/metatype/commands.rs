use tauri::{Emitter, State};
use crate::AppState;
use crate::metatype::{types::*, repository};

#[tauri::command]
pub async fn list_metatypes(state: State<'_, AppState>) -> Result<Vec<MetatypeSummary>, String> {
    log::info!("list_metatypes");
    let pool = state.db_pool.clone();

    tokio::task::spawn_blocking(move || {
        let connection = pool.get().map_err(|e| e.to_string())?;
        repository::list_metatypes(&connection).map_err(|e| e.to_string())
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn get_metatype(name: String, state: State<'_, AppState>) -> Result<Metatype, String> {
    log::info!("get_metatype with {:?}", &name);
    let pool = state.db_pool.clone();

    tokio::task::spawn_blocking(move || {
        let connection = pool.get().map_err(|e| e.to_string())?;
        repository::get_metatype(&connection, &name).map_err(|e| e.to_string())
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn create_metatype(metatype: Metatype, state: State<'_, AppState>) -> Result<Metatype, String> {
    log::info!("create_metatype with {:#?}", &metatype);
    let pool = state.db_pool.clone();
    let app_handle = state.app_handle.clone();

    let res = tokio::task::spawn_blocking(move || {
        let connection = pool.get().map_err(|e| e.to_string())?;
        repository::create_metatype(&connection, &metatype).map_err(|e| e.to_string())
    }).await.map_err(|e| e.to_string())??;

    app_handle.emit("created_metatype", &res).map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
pub async fn import_metatype(yaml: String, state: State<'_, AppState>) -> Result<Metatype, String> {
    log::info!("import_metatype with {:#?}", &yaml);
    let pool = state.db_pool.clone();
    let app_handle = state.app_handle.clone();

    let res = tokio::task::spawn_blocking(move || {
        let connection = pool.get().map_err(|e| e.to_string())?;
        repository::import_metatype(&connection, &yaml).map_err(|e| e.to_string())
    }).await.map_err(|e| e.to_string())??;

    app_handle.emit("created_metatype", &res).map_err(|e| e.to_string())?;
    Ok(res)
}
