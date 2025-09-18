use crate::priority_bundle::PriorityBundle;
use crate::{character::CharacterStatus, error::Result};
use rusqlite::{named_params, Connection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCreatePriorityOption {
    pub grade: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_bundle_id: Option<i64>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct CharacterCreateParams {
    pub priority_system: String,
    pub priorities: HashMap<String, CharacterCreatePriorityOption>,
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

    let character_id = connection.last_insert_rowid();
    create_priorities(connection, &params, &character_id)?;

    let bundles = get_relevant_bundles(connection, &params)?;
    log::debug!("Listed priority bundles: {:#?}", &bundles);
    // Add Enum for sub-choices within a field i.e. magic/resonance choice within bundle

    Ok(())
}

static PRIORITY_CREATE_QUERY: &str = "INSERT INTO character_priorities (bundle_name, grade, priority_system, character_id)
                                      VALUES (:bundle_name, :grade, :priority_system, :character_id)";

fn create_priorities(
    connection: &Connection,
    params: &CharacterCreateParams,
    character_id: &i64,
) -> Result<()> {
    log::debug!("create_priorities with {:#?}", &params.priorities);
    let mut stmt = connection.prepare(PRIORITY_CREATE_QUERY)?;

    for (bundle_name, bundle_value) in &params.priorities {
        stmt.execute(named_params! {
            ":bundle_name": &bundle_name,
            ":grade": &bundle_value.grade,
            ":priority_system": &params.priority_system,
            ":character_id": &character_id,
        })?;
    }

    Ok(())
}

static BUNDLE_GET_QUERY: &str =
    "SELECT id, name, grade, menu_order, parent_bundle_id, system, created_at, updated_at
     FROM priority_bundles
     WHERE system = :system";

fn get_relevant_bundles(
    connection: &Connection,
    params: &CharacterCreateParams,
) -> Result<Vec<PriorityBundle>> {
    let mut stmt = connection.prepare(BUNDLE_GET_QUERY)?;
    let priority_bundles = stmt
        .query_map(named_params! { ":system": params.priority_system }, |row| {
            Ok(PriorityBundle {
                id: row.get("id")?,
                name: row.get("name")?,
                grade: row.get("grade")?,
                menu_order: row.get("menu_order")?,
                parent_bundle_id: row.get("parent_bundle_id")?,
                system: row.get("system")?,
                modifiers: HashMap::new(),
                children: HashMap::new(),
                qualities: HashMap::new(),
                skills: HashMap::new(),
                metatypes: HashMap::new(),
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(priority_bundles)
}
