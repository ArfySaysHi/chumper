use crate::priority_bundle::PriorityBundle;
use rusqlite::Connection;

pub struct CharacterCreateParams {
    pub priority_bundles: HashMap<String, PriorityBundle>,
    pub metatype_id: i64,
    pub skill_selections: Vec<String>,
}

pub fn create(connection: &Connection, params: CharacterCreateParams) -> Result<()> {
    Ok(())
}
