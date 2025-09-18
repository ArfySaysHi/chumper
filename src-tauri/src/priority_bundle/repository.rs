use super::helpers::nest_priority_bundles;
use super::{PriorityBundle, PriorityBundleModifier};
use crate::priority_bundle::helpers::group_by_grade;
use crate::priority_bundle::{PriorityBundleMetatype, PriorityBundleQuality, PriorityBundleSkill};
use crate::{error::Result, import::YamlImportable};
use rusqlite::{named_params, Connection};
use std::collections::HashMap;

pub fn list_priority_bundles(connection: &Connection) -> Result<Vec<PriorityBundle>> {
    log::info!("list_priority_bundles");
    let query =
        "SELECT id, name, grade, menu_order, parent_bundle_id, system FROM priority_bundles"
            .to_string();
    let mut stmt = connection.prepare(&query)?;
    let mut priority_bundles = stmt
        .query_map([], |row| {
            Ok(PriorityBundle {
                id: row.get("id")?,
                name: row.get("name")?,
                grade: row.get("grade")?,
                menu_order: row.get("menu_order")?,
                parent_bundle_id: row.get("parent_bundle_id")?,
                system: row.get("system")?,
                modifiers: HashMap::new(),
                skills: HashMap::new(),
                metatypes: HashMap::new(),
                qualities: HashMap::new(),
                children: HashMap::new(),
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
                pb.modifiers = group_by_grade(mods, |m| &m.grade);
            }
            if let Some(skills) = pbs_map.remove(&pb_id) {
                pb.skills = group_by_grade(skills, |s| &s.grade);
            }
            if let Some(metatypes) = pb_meta_map.remove(&pb_id) {
                pb.metatypes = group_by_grade(metatypes, |m| &m.grade);
            }
            if let Some(qualities) = pbq_map.remove(&pb_id) {
                pb.qualities = group_by_grade(qualities, |q| &q.grade);
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
    let keys = pb.children.keys();

    for key in keys {
        for item in &pb.children[key] {
            let mut child = item.clone();
            child.parent_bundle_id = Some(rowid);
            child.grade = key.to_string();
            child.insert_into_db(connection)?;
        }
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
    let keys = pb.modifiers.keys();

    for key in keys {
        for item in &pb.modifiers[key] {
            stmt.execute(named_params! {
                ":grade": key,
                ":bundle_id": &pb.id,
                ":target_key": &item.target_key,
                ":operation": &item.operation,
                ":value": &item.value,
            })?;
        }
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
    let keys = pb.skills.keys();

    for key in keys {
        for item in &pb.skills[key] {
            stmt.execute(named_params! {
                ":grade": key,
                ":bundle_id": &pb.id,
                ":attribute": &item.attribute,
                ":amount": &item.amount,
                ":rating": &item.rating,
            })?;
        }
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
    let keys = pb.metatypes.keys();

    for key in keys {
        for item in &pb.metatypes[key] {
            stmt.execute(named_params! {
                ":grade": key,
                ":bundle_id": &pb.id,
                ":special_points": &item.special_points,
                ":name": &item.name,
            })?;
        }
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
    let keys = pb.qualities.keys();

    for key in keys {
        for item in &pb.qualities[key] {
            stmt.execute(named_params! {
                ":bundle_id": &pb.id,
                ":grade": key,
                ":name": &item.name,
            })?;
        }
    }

    Ok(())
}
