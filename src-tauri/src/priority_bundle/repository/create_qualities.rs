use crate::error::Result;
use crate::priority_bundle::PriorityBundle;
use rusqlite::{named_params, Connection};

pub fn create_priority_bundle_qualities(
    connection: &Connection,
    pb: &PriorityBundle,
) -> Result<()> {
    log::info!("create_priority_bundle_qualities with {:#?}", &pb);
    let query = "INSERT INTO priority_bundle_qualities
                   (bundle_id, name)
                 VALUES (:bundle_id, :name)"
        .to_string();
    let mut stmt = connection.prepare(&query)?;

    for item in &pb.qualities {
        stmt.execute(named_params! {
            ":bundle_id": &pb.id,
            ":name": &item.name,
        })?;
    }

    Ok(())
}
