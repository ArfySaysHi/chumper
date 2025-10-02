use crate::priority_bundle::repository::{list_priority_bundles, PriorityBundleListParams};
use crate::priority_bundle::PriorityBundle;
use crate::shared::{group_by_key, Grade};
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

    let priority_bundles = list_priority_bundles(
        connection,
        &PriorityBundleListParams {
            system: "Core".to_string(),
        },
    )?;
    let bundle_map = group_by_key(priority_bundles, |pb| pb.id);

    let character_id = connection.last_insert_rowid();
    create_priorities(connection, params, &bundle_map, &character_id)?;

    Ok(())
}

// Bookkeeping for later
fn create_priorities(
    connection: &Connection,
    params: CharacterCreateParams,
    bundle_map: &HashMap<i64, Vec<PriorityBundle>>,
    character_id: &i64,
) -> Result<()> {
    let query = "INSERT INTO priorities (bundle_name, grade, priority_system, character_id)
                 VALUES (:bundle_name, :grade, :priority_system, :character_id)";

    let mut stmt = connection.prepare(&query)?;
    for (bundle_id, option) in &params.priorities {
        let pb_vec = bundle_map.get(&bundle_id);

        if let Some(bundles) = pb_vec {
            if let Some(current_bundle) = bundles.first() {
                stmt.execute(named_params! {
                    ":bundle_name": current_bundle.name,
                    ":grade": option.grade,
                    ":priority_system": &params.priority_system,
                    ":character_id": character_id,
                })?;
            }
        }
    }

    Ok(())
}
