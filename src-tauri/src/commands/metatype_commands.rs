use tauri::State;
use crate::utils::error_handling::CommandResult;
use crate::controllers::metatype_controller::MetatypeController;
use crate::models::metatype::{Metatype, MetatypeSummary};

#[tauri::command]
pub async fn list_metatypes(controller: State<'_, MetatypeController>) -> CommandResult<Vec<MetatypeSummary>> {
    controller.list_metatypes()
}

#[tauri::command]
pub async fn get_metatype(name: &str, controller: State<'_, MetatypeController>) -> CommandResult<Metatype> {
    controller.get_metatype(name)
}
