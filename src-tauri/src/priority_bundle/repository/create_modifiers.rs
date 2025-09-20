use crate::error::Result;
use crate::priority_bundle::PriorityBundle;
use rusqlite::{named_params, Connection};

pub fn create_priority_bundle_modifiers(
    connection: &Connection,
    pb: &PriorityBundle,
) -> Result<()> {
    log::info!("create_priority_bundle_modifier with {:#?}", &pb);
    let query = "INSERT INTO priority_bundle_modifiers
                   (bundle_id, target_key, operation, value)
                 VALUES (:bundle_id, :target_key, :operation, :value)"
        .to_string();
    let mut stmt = connection.prepare(&query)?;

    for item in &pb.modifiers {
        stmt.execute(named_params! {
            ":bundle_id": &pb.id,
            ":target_key": &item.target_key,
            ":operation": &item.operation,
            ":value": &item.value,
        })?;
    }

    Ok(())
}
