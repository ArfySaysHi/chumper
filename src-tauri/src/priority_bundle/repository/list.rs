use crate::error::Result;
use crate::priority_bundle::{
    PriorityBundle, PriorityBundleMetatype, PriorityBundleModifier, PriorityBundleQuality,
    PriorityBundleSkill,
};
use rusqlite::Connection;
use std::collections::HashMap;

pub fn list_priority_bundles(connection: &Connection) -> Result<Vec<PriorityBundle>> {
    log::info!("list_priority_bundles");
    let query = "SELECT id, name, grade, system FROM priority_bundles".to_string();
    let mut stmt = connection.prepare(&query)?;
    let mut priority_bundles = stmt
        .query_map([], |row| {
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
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    if priority_bundles.is_empty() {
        return Ok(priority_bundles);
    }

    let pbm_query = "SELECT id, bundle_id, target_key, operation, value, created_at, updated_at
                     FROM priority_bundle_modifiers"
        .to_string();
    let mut pbm_stmt = connection.prepare(&pbm_query)?;
    let pbm_iter = pbm_stmt
        .query_map([], |row| {
            Ok(PriorityBundleModifier {
                id: row.get("id")?,
                bundle_id: row.get("bundle_id")?,
                target_key: row.get("target_key")?,
                operation: row.get("operation")?,
                value: row.get("value")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let mut pbm_map: HashMap<i64, Vec<PriorityBundleModifier>> = HashMap::new();
    for res in pbm_iter {
        if let Some(bundle_id) = res.bundle_id {
            pbm_map.entry(bundle_id).or_default().push(res);
        }
    }

    let pbs_query =
        "SELECT id, bundle_id, attribute, amount, rating FROM priority_bundle_skills".to_string();
    let mut pbs_stmt = connection.prepare(&pbs_query)?;
    let pbs_iter = pbs_stmt
        .query_map([], |row| {
            Ok(PriorityBundleSkill {
                id: row.get("id")?,
                bundle_id: row.get("bundle_id")?,
                attribute: row.get("attribute")?,
                amount: row.get("amount")?,
                rating: row.get("rating")?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let mut pbs_map: HashMap<i64, Vec<PriorityBundleSkill>> = HashMap::new();
    for res in pbs_iter {
        if let Some(bundle_id) = res.bundle_id {
            pbs_map.entry(bundle_id).or_default().push(res);
        }
    }

    let pb_meta_query =
        "SELECT id, bundle_id, special_points, name FROM priority_bundle_metatypes".to_string();
    let mut pb_meta_stmt = connection.prepare(&pb_meta_query)?;
    let pb_meta_iter = pb_meta_stmt
        .query_map([], |row| {
            Ok(PriorityBundleMetatype {
                id: row.get("id")?,
                bundle_id: row.get("bundle_id")?,
                special_points: row.get("special_points")?,
                name: row.get("name")?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let mut pb_meta_map: HashMap<i64, Vec<PriorityBundleMetatype>> = HashMap::new();
    for res in pb_meta_iter {
        if let Some(bundle_id) = res.bundle_id {
            pb_meta_map.entry(bundle_id).or_default().push(res);
        }
    }

    let pbq_query = "SELECT id, bundle_id, name FROM priority_bundle_qualities".to_string();
    let mut pbq_stmt = connection.prepare(&pbq_query)?;
    let pbq_iter = pbq_stmt
        .query_map([], |row| {
            Ok(PriorityBundleQuality {
                id: row.get("id")?,
                bundle_id: row.get("bundle_id")?,
                name: row.get("name")?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let mut pbq_map: HashMap<i64, Vec<PriorityBundleQuality>> = HashMap::new();
    for res in pbq_iter {
        if let Some(bundle_id) = res.bundle_id {
            pbq_map.entry(bundle_id).or_default().push(res);
        }
    }
    for pb in &mut priority_bundles {
        if let Some(pb_id) = &pb.id {
            if let Some(mods) = pbm_map.remove(&pb_id) {
                pb.modifiers = mods;
            }
            if let Some(skills) = pbs_map.remove(&pb_id) {
                pb.skills = skills;
            }
            if let Some(metatypes) = pb_meta_map.remove(&pb_id) {
                pb.metatypes = metatypes;
            }
            if let Some(qualities) = pbq_map.remove(&pb_id) {
                pb.qualities = qualities;
            }
        }
    }

    Ok(priority_bundles)
}
