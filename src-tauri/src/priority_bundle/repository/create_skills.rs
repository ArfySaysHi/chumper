use crate::error::Result;
use crate::priority_bundle::PriorityBundle;
use rusqlite::{named_params, Connection};

pub fn create_priority_bundle_skills(connection: &Connection, pb: &PriorityBundle) -> Result<()> {
    log::info!("create_priority_bundle_skills with {:#?}", &pb);
    let query = "INSERT INTO priority_bundle_skills
                   (bundle_id, attribute, amount, rating)
                 VALUES (:bundle_id, :attribute, :amount, :rating)"
        .to_string();
    let mut stmt = connection.prepare(&query)?;

    for item in &pb.skills {
        stmt.execute(named_params! {
            ":bundle_id": &pb.id,
            ":attribute": &item.attribute,
            ":amount": &item.amount,
            ":rating": &item.rating,
        })?;
    }

    Ok(())
}
