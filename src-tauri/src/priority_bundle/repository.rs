use super::PriorityBundle;
use crate::error::Result;
use rusqlite::{named_params, Connection};

pub fn create_priority_bundle(
    connection: &Connection,
    pb: &PriorityBundle,
) -> Result<PriorityBundle> {
    log::info!("create_priority_bundle with {:#?}", &pb);
    let query = "INSERT INTO priority_bundles (domain)
                 VALUES (:domain)"
        .to_string();
    let mut stmt = connection.prepare(&query)?;
    stmt.execute(named_params! {
        ":domain": &pb.domain,
    })?;

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
