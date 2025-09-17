use crate::{character::CharacterStatus, error::Result};
use rusqlite::{named_params, Connection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCreateParams {
    pub priority_system: String,
    pub priorities: HashMap<String, String>,
    pub metatype_id: i64,
    pub skill_selections: Vec<HashMap<i64, i32>>,
}

static CHARACTER_CREATE_QUERY: &str = "INSERT INTO characters (name, player_name, status)
                                       VALUES (:name, :player_name, :status)";

static PRIORITY_CREATE_QUERY: &str = "INSERT INTO character_priorities (bundle_name, grade, priority_system, character_id)
                                      VALUES (:bundle_name, :grade, :priority_system, :character_id)";

static BUNDLE_GET_QUERY: &str =
    "SELECT id, name, grade, menu_order, parent_bundle_id, created_at, updated_at
                                 FROM priority_bundles";

pub fn create(connection: &Connection, params: CharacterCreateParams) -> Result<()> {
    log::debug!("character/create with {:#?}", &params);
    let mut stmt = connection.prepare(CHARACTER_CREATE_QUERY)?;
    stmt.execute(named_params! {
        ":name": "",
        ":player_name": "",
        ":status": CharacterStatus::Creation,
        ":metatype_id": &params.metatype_id,
    })?;

    let character_id = connection.last_insert_rowid();
    create_priorities(connection, &params, &character_id)?;

    // Get all bundles for selected priorities
    // Add Enum for sub-choices within a field i.e. magic/resonance choice within bundle

    Ok(())
}

fn create_priorities(
    connection: &Connection,
    params: &CharacterCreateParams,
    character_id: &i64,
) -> Result<()> {
    log::debug!("create_priorities with {:#?}", &params.priorities);
    let mut stmt = connection.prepare(PRIORITY_CREATE_QUERY)?;

    for (bundle_name, grade) in &params.priorities {
        stmt.execute(named_params! {
            ":bundle_name": &bundle_name,
            ":grade": &grade,
            ":priority_system": &params.priority_system,
            ":character_id": &character_id,
        })?;
    }

    Ok(())
}
