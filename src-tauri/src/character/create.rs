use crate::shared::Grade;
use crate::{character::CharacterStatus, error::Result};
use rusqlite::{named_params, Connection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCreatePriorityOption {
    pub grade: Grade,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_bundle_id: Option<i64>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCreateParams {
    pub priority_system: String,
    pub priorities: HashMap<i64, CharacterCreatePriorityOption>,
    pub metatype_id: i64,
    pub skill_selections: Vec<HashMap<i64, i32>>,
}

static CHARACTER_CREATE_QUERY: &str =
    "INSERT INTO characters (name, player_name, status, metatype_id)
                                       VALUES (:name, :player_name, :status, :metatype_id)";

pub fn create(connection: &Connection, params: CharacterCreateParams) -> Result<()> {
    log::debug!("character/create with {:#?}", &params);
    let mut stmt = connection.prepare(CHARACTER_CREATE_QUERY)?;
    stmt.execute(named_params! {
        ":name": "",
        ":player_name": "",
        ":status": CharacterStatus::Creation,
        ":metatype_id": &params.metatype_id,
    })?;

    let _character_id = connection.last_insert_rowid();
    Ok(())
}
