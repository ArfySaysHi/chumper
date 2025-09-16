use crate::priority_bundle::PriorityBundle;
use crate::{character::CharacterStatus, error::Result};
use rusqlite::{named_params, Connection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCreateParams {
    pub priority_system: String,
    pub grades: HashMap<String, String>,
    pub metatype_id: i64,
    pub skill_selections: Vec<i64>,
}

pub fn create(connection: &Connection, params: CharacterCreateParams) -> Result<()> {
    log::debug!("character/create with {:#?}", &params);
    let query = "INSERT INTO characters (name, player_name, status)
                 VALUES (:name, :player_name, :status)"
        .to_string();
    let mut stmt = connection.prepare(&query)?;
    stmt.execute(named_params! {
        ":name": "",
        ":player_name": "",
        ":status": CharacterStatus::Creation
    })?;

    Ok(())
}
