use crate::error::Result;
use crate::priority_bundle::{
    PriorityBundle, PriorityBundleMetatype, PriorityBundleModifier, PriorityBundleQuality,
    PriorityBundleSkill,
};
use crate::shared::defaults::system;
use crate::shared::{group_by_key, query_vec};
use rusqlite::{named_params, Connection};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListPriorityBundlesParams {
    #[serde(default = "system")]
    pub system: String,
}

pub fn list_priority_bundles(
    connection: &Connection,
    params: &ListPriorityBundlesParams,
) -> Result<Vec<PriorityBundle>> {
    log::info!("list_priority_bundles");
    let mut priority_bundles = get_priority_bundles(connection, params)?;

    if priority_bundles.is_empty() {
        return Ok(priority_bundles);
    }

    let modifiers = get_modifiers(connection)?;
    let mut mod_map = group_by_key(modifiers, |modi| modi.bundle_id);

    let skills = get_skills(connection)?;
    let mut skill_map = group_by_key(skills, |skill| skill.bundle_id);

    let metatypes = get_metatypes(connection)?;
    let mut metatype_map = group_by_key(metatypes, |meta| meta.bundle_id);

    let qualities = get_qualities(connection)?;
    let mut quality_map = group_by_key(qualities, |quality| quality.bundle_id);

    for pb in &mut priority_bundles {
        if let Some(pb_id) = &pb.id {
            if let Some(mods) = mod_map.remove(&pb_id) {
                pb.modifiers = mods;
            }
            if let Some(skills) = skill_map.remove(&pb_id) {
                pb.skills = skills;
            }
            if let Some(metatypes) = metatype_map.remove(&pb_id) {
                pb.metatypes = metatypes;
            }
            if let Some(qualities) = quality_map.remove(&pb_id) {
                pb.qualities = qualities;
            }
        }
    }

    Ok(priority_bundles)
}

fn get_priority_bundles(
    connection: &Connection,
    params: &ListPriorityBundlesParams,
) -> Result<Vec<PriorityBundle>> {
    let mut query = "SELECT id, name, grade, system FROM priority_bundles WHERE 1=1".to_string();
    query.push_str(" AND system = :system");

    let res = query_vec(
        connection,
        &query,
        named_params! { ":system": &params.system},
        |row| {
            Ok(PriorityBundle {
                id: row.get("id")?,
                name: row.get("name")?,
                grade: row.get("grade")?,
                system: row.get("system")?,
                modifiers: Vec::new(),
                skills: Vec::new(),
                metatypes: Vec::new(),
                qualities: Vec::new(),
                options: Vec::new(),
            })
        },
    )?;

    Ok(res)
}

fn get_modifiers(connection: &Connection) -> Result<Vec<PriorityBundleModifier>> {
    let query = "SELECT id, bundle_id, target_key, operation, value, created_at, updated_at
                 FROM priority_bundle_modifiers";
    let res = query_vec(connection, query, [], |row| {
        Ok(PriorityBundleModifier {
            id: row.get("id")?,
            bundle_id: row.get("bundle_id")?,
            target_key: row.get("target_key")?,
            operation: row.get("operation")?,
            value: row.get("value")?,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
        })
    })?;
    Ok(res)
}

fn get_skills(connection: &Connection) -> Result<Vec<PriorityBundleSkill>> {
    let query = "SELECT id, bundle_id, attribute, amount, rating FROM priority_bundle_skills";
    let res = query_vec(connection, query, [], |row| {
        Ok(PriorityBundleSkill {
            id: row.get("id")?,
            bundle_id: row.get("bundle_id")?,
            attribute: row.get("attribute")?,
            amount: row.get("amount")?,
            rating: row.get("rating")?,
        })
    })?;

    Ok(res)
}

fn get_metatypes(connection: &Connection) -> Result<Vec<PriorityBundleMetatype>> {
    let query = "SELECT id, bundle_id, special_points, name FROM priority_bundle_metatypes";
    let res = query_vec(connection, query, [], |row| {
        Ok(PriorityBundleMetatype {
            id: row.get("id")?,
            bundle_id: row.get("bundle_id")?,
            special_points: row.get("special_points")?,
            name: row.get("name")?,
        })
    })?;

    Ok(res)
}

fn get_qualities(connection: &Connection) -> Result<Vec<PriorityBundleQuality>> {
    let query = "SELECT id, bundle_id, name FROM priority_bundle_qualities";
    let res = query_vec(connection, query, [], |row| {
        Ok(PriorityBundleQuality {
            id: row.get("id")?,
            bundle_id: row.get("bundle_id")?,
            name: row.get("name")?,
        })
    })?;

    Ok(res)
}
