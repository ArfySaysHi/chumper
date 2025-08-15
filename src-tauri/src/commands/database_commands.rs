use tauri::State;
use crate::utils::error_handling::CommandResult;
use crate::controllers::database_controller::DatabaseController;

#[tauri::command]
pub async fn init_database(controller: State<'_, DatabaseController>) -> CommandResult<()> {
    controller.init_database()
}
