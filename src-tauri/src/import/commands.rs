use super::repository::import_yaml_file;
use crate::character::Character;
use crate::database::AppState;
use crate::metatype::types::Metatype;
use crate::priority_grade::PriorityGrade;
use std::result::Result;
use tauri::State;

#[tauri::command]
pub async fn import_characters(path: String, state: State<'_, AppState>) -> Result<String, String> {
    log::info!("import characters with {:?}", &path);
    let pool = state.db_pool.clone();

    tokio::task::spawn_blocking(move || {
        let mut connection = pool.get().map_err(|e| e.to_string())?;
        import_yaml_file::<Character>(&mut connection, &path).map_err(|e| e.to_string())?;
        Ok("Characters imported successfully".to_string())
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn import_metatypes(path: String, state: State<'_, AppState>) -> Result<String, String> {
    log::info!("import_metatypes with {:?}", &path);
    let pool = state.db_pool.clone();

    tokio::task::spawn_blocking(move || {
        let mut connection = pool.get().map_err(|e| e.to_string())?;
        import_yaml_file::<Metatype>(&mut connection, &path).map_err(|e| e.to_string())?;
        Ok("Metatypes imported successfully".to_string())
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn import_priority_grades(path: String, state: State<'_, AppState>) -> Result<String, String> {
    log::info!("import_priority_grades with {:?}", path);
    let pool = state.db_pool.clone();

    tokio::task::spawn_blocking(move || {
        let mut connection = pool.get().map_err(|e| e.to_string())?;
        import_yaml_file::<PriorityGrade>(&mut connection, &path).map_err(|e| e.to_string())?;
        Ok("Priority grades imported successfully".to_string())
    }).await.map_err(|e| e.to_string())?
}
