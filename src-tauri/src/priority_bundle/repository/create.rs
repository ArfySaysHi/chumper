use crate::error::Result;
use crate::priority_bundle::PriorityBundle;
use rusqlite::{named_params, Connection};

pub fn create_priority_bundle(
    connection: &Connection,
    pb: &PriorityBundle,
) -> Result<PriorityBundle> {
    log::info!("create_priority_bundle with {:#?}", &pb);
    let query = "INSERT INTO priority_bundles (name, grade)
                 VALUES (:name, :grade)"
        .to_string();
    let mut stmt = connection.prepare(&query)?;
    stmt.execute(named_params! {
        ":name": &pb.name,
        ":grade": &pb.grade,
    })?;

    let rowid = connection.last_insert_rowid();
    let mut cloned_pb = pb.clone();
    cloned_pb.id = Some(rowid);

    Ok(cloned_pb)
}
