use crate::priority_bundle::PriorityBundle;
use crate::{error::Result, import::YamlImportable};
use rusqlite::{named_params, Connection};

pub fn create_priority_bundle(
    connection: &Connection,
    pb: &PriorityBundle,
) -> Result<PriorityBundle> {
    log::info!("create_priority_bundle with {:#?}", &pb);
    let query = "INSERT INTO priority_bundles (name, grade, parent_id)
                 VALUES (:name, :grade, :parent_id)"
        .to_string();
    let mut stmt = connection.prepare(&query)?;
    stmt.execute(named_params! {
        ":name": &pb.name,
        ":grade": &pb.grade,
        ":parent_id": &pb.parent_id,
    })?;

    let rowid = connection.last_insert_rowid();

    for option in &pb.options {
        let mut child = option.clone();
        child.parent_id = Some(rowid);
        child.insert_into_db(connection)?;
    }

    let mut cloned_pb = pb.clone();
    cloned_pb.id = Some(rowid);

    Ok(cloned_pb)
}
