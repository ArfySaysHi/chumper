use tauri::State;
use crate::utils::error_handling::CommandResult;
use crate::controllers::character_controller::CharacterController;
use crate::models::character::{Character, CharacterSummary};

#[tauri::command]
pub async fn list_characters(controller: State<'_, CharacterController>) -> CommandResult<Vec<CharacterSummary>> {
    controller.list_characters()
}

#[tauri::command]
pub async fn import_character(yaml: &str, controller: State<'_, CharacterController>) -> CommandResult<i64> {
    controller.import_character(yaml)
}

#[tauri::command]
pub async fn create_character(character: Character, controller: State<'_, CharacterController>) -> CommandResult<i64> {
    controller.create_character(character)
}
