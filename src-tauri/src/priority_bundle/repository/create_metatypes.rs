use crate::error::Result;
use crate::priority_bundle::PriorityBundle;
use rusqlite::{named_params, Connection};

pub fn create_priority_bundle_metatypes(
    connection: &Connection,
    pb: &PriorityBundle,
) -> Result<()> {
    log::info!("create_priority_bundle_metatypes with {:#?}", &pb);
    let query = "INSERT INTO priority_bundle_metatypes
                   (bundle_id, special_points, name)
                 VALUES (:bundle_id, :special_points, :name)"
        .to_string();
    let mut stmt = connection.prepare(&query)?;

    for item in &pb.metatypes {
        stmt.execute(named_params! {
            ":bundle_id": &pb.id,
            ":special_points": &item.special_points,
            ":name": &item.name,
        })?;
    }

    Ok(())
}
