use tauri::{Emitter, State};
use crate::database::AppState;
use crate::character::{repository, types::*};
use crate::import::{YamlImportable, YamlSerializable};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListCharactersParams {
    pub status: Option<String>,
    #[serde(default = "default_sort_by")]
    pub sort_by: String,
    #[serde(default = "default_sort_direction")]
    pub sort_direction: String,
}

fn default_sort_by() -> String { "updated_at".to_string() }
fn default_sort_direction() -> String { "DESC".to_string() }

#[tauri::command]
pub async fn list_characters(params: ListCharactersParams, state: State<'_, AppState>) -> Result<Vec<CharacterSummary>, String> {
    let pool = state.db_pool.clone();

    tokio::task::spawn_blocking(move || {
        let connection = pool.get().map_err(|e| e.to_string())?;
        repository::list_characters(&connection, Some(params)).map_err(|e| e.to_string())
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn get_character(id: i64, state: State<'_, AppState>) -> Result<Character, String> {
    let pool = state.db_pool.clone();

    tokio::task::spawn_blocking(move || {
        let connection = pool.get().map_err(|e| e.to_string())?;
        repository::get_character_by_id(&connection, id).map_err(|e| e.to_string())
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn create_character(character: Character, state: State<'_, AppState>) -> Result<Character, String> {
    let pool = state.db_pool.clone();
    let app_handle = state.app_handle.clone();

    let res = tokio::task::spawn_blocking(move || -> Result<Character, String> {
        let mut connection = pool.get().map_err(|e| e.to_string())?;
        let result = repository::create_character(&mut connection, &character).map_err(|e| e.to_string())?;
        Ok(result)
    })
    .await
    .map_err(|e| e.to_string())??; // Double ? for join error + your Result

    app_handle.emit("character_created", &res).map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
pub async fn import_character(yaml: String, state: State<'_, AppState>) -> Result<Character, String> {
    let pool = state.db_pool.clone();

    tokio::task::spawn_blocking(move || {
        let mut connection = pool.get().map_err(|e| e.to_string())?;
        let mut character = Character::from_yaml(&yaml).map_err(|e| format!("YAML parse error: {}", e))?;
        character.id = None;
        character.insert_into_db(&mut connection).map_err(|e| format!("Database error: {}", e))
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn export_character(id: i64, state: State<'_, AppState>) -> Result<String, String> {
    let pool = state.db_pool.clone();

    tokio::task::spawn_blocking(move || {
        let connection = pool.get().map_err(|e| e.to_string())?;
        let character = repository::get_character_by_id(&connection, id).map_err(|e| e.to_string())?;
        character.to_yaml().map_err(|e| e.to_string())
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn delete_character(id: i64, state: State<'_, AppState>) -> Result<String, String> {
    let pool = state.db_pool.clone();

    let res = tokio::task::spawn_blocking(move || {
        let connection = pool.get().map_err(|e| e.to_string())?;
        repository::delete_character(&connection, id).map_err(|e| e.to_string())
    }).await.map_err(|e| e.to_string())?;

    state.app_handle.emit("character_deleted", id).map_err(|e| e.to_string())?;
    res
}

#[tauri::command]
pub async fn archive_character(id: i64, state: State<'_, AppState>) -> Result<String, String> {
    let pool = state.db_pool.clone();

    let res = tokio::task::spawn_blocking(move || {
        let connection = pool.get().map_err(|e| e.to_string())?;
        repository::archive_character(&connection, id).map_err(|e| e.to_string())
    }).await.map_err(|e| e.to_string())?;

    state.app_handle.emit("character_deleted", id).map_err(|e| e.to_string())?;
    res
}
