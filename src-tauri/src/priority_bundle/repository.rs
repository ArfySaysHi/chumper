use super::helpers::nest_priority_bundles;
use super::{PriorityBundle, PriorityBundleModifier};
use crate::priority_bundle::{PriorityBundleMetatype, PriorityBundleQuality, PriorityBundleSkill};
use crate::{error::Result, import::YamlImportable};
use rusqlite::{named_params, Connection};
use std::collections::HashMap;

pub fn list_priority_bundles(connection: &Connection) -> Result<Vec<PriorityBundle>> {
    log::info!("list_priority_bundles");
    let query =
        "SELECT id, name, grade, menu_order, parent_bundle_id FROM priority_bundles".to_string();
    let mut stmt = connection.prepare(&query)?;
    let mut priority_bundles = stmt
        .query_map([], |row| {
            Ok(PriorityBundle {
                id: row.get("id")?,
                name: row.get("name")?,
                grade: row.get("grade")?,
                menu_order: row.get("menu_order")?,
                parent_bundle_id: row.get("parent_bundle_id")?,
                modifiers: Vec::new(),
                skills: Vec::new(),
                metatypes: Vec::new(),
                qualities: Vec::new(),
                children: Vec::new(),
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    if priority_bundles.is_empty() {
        return Ok(priority_bundles);
    }

    let pbm_query = "SELECT id, grade, bundle_id, target_key, operation,
                                value, created_at, updated_at
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
        "SELECT id, grade, bundle_id, attribute, amount, rating FROM priority_bundle_skills"
            .to_string();
    let mut pbs_stmt = connection.prepare(&pbs_query)?;
    let pbs_iter = pbs_stmt
        .query_map([], |row| {
            Ok(PriorityBundleSkill {
                id: row.get("id")?,
                grade: row.get("grade")?,
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
        "SELECT id, bundle_id, grade, special_points, name FROM priority_bundle_metatypes"
            .to_string();
    let mut pb_meta_stmt = connection.prepare(&pb_meta_query)?;
    let pb_meta_iter = pb_meta_stmt
        .query_map([], |row| {
            Ok(PriorityBundleMetatype {
                id: row.get("id")?,
                bundle_id: row.get("bundle_id")?,
                grade: row.get("grade")?,
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

    let pbq_query = "SELECT id, bundle_id, grade, name FROM priority_bundle_qualities".to_string();
    let mut pbq_stmt = connection.prepare(&pbq_query)?;
    let pbq_iter = pbq_stmt
        .query_map([], |row| {
            Ok(PriorityBundleQuality {
                id: row.get("id")?,
                bundle_id: row.get("bundle_id")?,
                grade: row.get("grade")?,
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

    let roots = nest_priority_bundles(priority_bundles);
    Ok(roots)
}

pub fn create_priority_bundle(
    connection: &Connection,
    pb: &PriorityBundle,
) -> Result<PriorityBundle> {
    log::info!("create_priority_bundle with {:#?}", &pb);
    let query = "INSERT INTO priority_bundles (name, grade, menu_order, parent_bundle_id)
                 VALUES (:name, :grade, :menu_order, :parent_bundle_id)"
        .to_string();
    let mut stmt = connection.prepare(&query)?;
    stmt.execute(named_params! {
        ":name": &pb.name,
        ":grade": &pb.grade,
        ":menu_order": &pb.menu_order,
        ":parent_bundle_id": &pb.parent_bundle_id,
    })?;

    let rowid = connection.last_insert_rowid();
    for nested_pb in &pb.children {
        let mut child = nested_pb.clone();
        child.parent_bundle_id = Some(rowid);
        child.insert_into_db(connection)?;
    }

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
                   (grade, bundle_id, target_key, operation, value)
                 VALUES (:grade, :bundle_id, :target_key, :operation, :value)"
        .to_string();
    let mut stmt = connection.prepare(&query)?;

    for pbm in pb.modifiers.iter() {
        stmt.execute(named_params! {
            ":grade": &pbm.grade,
            ":bundle_id": &pb.id,
            ":target_key": &pbm.target_key,
            ":operation": &pbm.operation,
            ":value": &pbm.value,
        })?;
    }

    Ok(())
}

pub fn create_priority_bundle_skills(connection: &Connection, pb: &PriorityBundle) -> Result<()> {
    log::info!("create_priority_bundle_skills with {:#?}", &pb);
    let query = "INSERT INTO priority_bundle_skills
                   (grade, bundle_id, attribute, amount, rating)
                 VALUES (:grade, :bundle_id, :attribute, :amount, :rating)"
        .to_string();
    let mut stmt = connection.prepare(&query)?;

    for pbs in pb.skills.iter() {
        stmt.execute(named_params! {
            ":grade": &pb.grade,
            ":bundle_id": &pb.id,
            ":attribute": &pbs.attribute,
            ":amount": &pbs.amount,
            ":rating": &pbs.rating,
        })?;
    }

    Ok(())
}

pub fn create_priority_bundle_metatypes(
    connection: &Connection,
    pb: &PriorityBundle,
) -> Result<()> {
    log::info!("create_priority_bundle_metatypes with {:#?}", &pb);
    let query = "INSERT INTO priority_bundle_metatypes
                   (bundle_id, grade, special_points, name)
                 VALUES (:bundle_id, :grade, :special_points, :name)"
        .to_string();
    let mut stmt = connection.prepare(&query)?;

    for pbm in pb.metatypes.iter() {
        stmt.execute(named_params! {
            ":bundle_id": &pb.id,
            ":grade": &pbm.grade,
            ":special_points": &pbm.special_points,
            ":name": &pbm.name,
        })?;
    }

    Ok(())
}

pub fn create_priority_bundle_qualities(
    connection: &Connection,
    pb: &PriorityBundle,
) -> Result<()> {
    log::info!("create_priority_bundle_qualities with {:#?}", &pb);
    let query = "INSERT INTO priority_bundle_qualities
                   (bundle_id, grade, name)
                 VALUES (:bundle_id, :grade, :name)"
        .to_string();
    let mut stmt = connection.prepare(&query)?;

    for pbq in pb.qualities.iter() {
        stmt.execute(named_params! {
            ":bundle_id": &pb.id,
            ":grade": &pbq.grade,
            ":name": &pbq.name,
        })?;
    }

    Ok(())
}
