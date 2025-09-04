use super::helpers::nest_priority_bundles;
use super::{PriorityBundle, PriorityBundleModifier};
use crate::{error::Result, import::YamlImportable};
use rusqlite::{named_params, Connection};
use std::collections::HashMap;

pub fn list_priority_bundles(connection: &Connection) -> Result<Vec<PriorityBundle>> {
    log::info!("list_priority_bundles");
    let query = "SELECT id, name, domain, parent_bundle_id FROM priority_bundles".to_string();
    let mut stmt = connection.prepare(&query)?;
    let mut priority_bundles = stmt
        .query_map([], |row| {
            Ok(PriorityBundle {
                id: row.get("id")?,
                name: row.get("name")?,
                domain: row.get("domain")?,
                parent_bundle_id: row.get("parent_bundle_id")?,
                priority_bundle_modifiers: Vec::new(),
                priority_bundles: Vec::new(),
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    if priority_bundles.is_empty() {
        return Ok(priority_bundles);
    }

    let pbm_query = "SELECT id, grade, bundle_id, target_key, operation,
                                value_formula, created_at, updated_at
                         FROM priority_bundle_modifiers"
        .to_string();
    let mut pbm_stmt = connection.prepare(&pbm_query)?;
    let pbm_iter = pbm_stmt
        .query_map([], |row| {
            Ok(PriorityBundleModifier {
                id: row.get("id")?,
                grade: row.get("grade")?,
                bundle_id: row.get("bundle_id")?,
                target_key: row.get("target_key")?,
                operation: row.get("operation")?,
                value_formula: row.get("value_formula")?,
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
    for pb in &mut priority_bundles {
        if let Some(pb_id) = &pb.id {
            if let Some(mods) = pbm_map.remove(&pb_id) {
                pb.priority_bundle_modifiers = mods;
            }
        }
    }

    let roots = nest_priority_bundles(priority_bundles);
    Ok(roots)
}

pub fn create_priority_bundle(
    connection: &Connection,
    pb: &PriorityBundle,
) -> Result<PriorityBundle> {
    log::info!("create_priority_bundle with {:#?}", &pb);
    let query = "INSERT INTO priority_bundles (name, domain, parent_bundle_id)
                 VALUES (:name, :domain, :parent_bundle_id)"
        .to_string();
    let mut stmt = connection.prepare(&query)?;
    stmt.execute(named_params! {
        ":name": &pb.name,
        ":domain": &pb.domain,
        ":parent_bundle_id": &pb.parent_bundle_id,
    })?;

    for nested_pb in &pb.priority_bundles {
        let mut child = nested_pb.clone();
        child.parent_bundle_id = Some(connection.last_insert_rowid());
        child.insert_into_db(connection)?;
    }

    let rowid = connection.last_insert_rowid();
    let mut cloned_pb = pb.clone();
    cloned_pb.id = Some(rowid);

    Ok(cloned_pb)
}

pub fn create_priority_bundle_modifiers(
    connection: &Connection,
    pb: &PriorityBundle,
) -> Result<()> {
    log::info!("create_priority_bundle_modifier with {:#?}", &pb);
    let query = "INSERT INTO priority_bundle_modifiers
                   (grade, bundle_id, target_key, operation, value_formula)
                 VALUES (:grade, :bundle_id, :target_key, :operation, :value_formula)"
        .to_string();
    let mut stmt = connection.prepare(&query)?;

    for pbm in pb.priority_bundle_modifiers.iter() {
        stmt.execute(named_params! {
            ":grade": &pbm.grade,
            ":bundle_id": &pb.id,
            ":target_key": &pbm.target_key,
            ":operation": &pbm.operation,
            ":value_formula": &pbm.value_formula,
        })?;
    }

    Ok(())
}
