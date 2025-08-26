use super::Quality;
use crate::error::Result;
use rusqlite::{named_params, Connection};

pub fn create_quality(connection: &Connection, q: &Quality) -> Result<Quality> {
    log::info!("create_quality with {:#?}", &q);
    let query = "INSERT INTO qualities
                   (name, resource_name, cost, category)
                 VALUES (:name, :resource_name, :cost, :category)"
        .to_string();
    let mut stmt = connection.prepare(&query)?;
    stmt.execute(named_params! {
        ":name": &q.name,
        ":resource_name": &q.resource_name,
        ":cost": &q.cost,
        ":category": &q.category
    })?;

    let row_id = connection.last_insert_rowid();
    let mut cloned_quality = q.clone();
    cloned_quality.id = Some(row_id);

    Ok(cloned_quality)
}

pub fn create_quality_effects(connection: &Connection, q: &Quality) -> Result<()> {
    log::info!("create_quality_effects with {:#?}", &q);
    let query = "INSERT INTO quality_effects
                   (quality_id, target_key, operation, value_formula, activation, priority)
                 VALUES
                   (:quality_id, :target_key, :operation, :value_formula, :activation, :priority)"
        .to_string();
    let mut stmt = connection.prepare(&query)?;

    if let Some(quality_effects) = &q.quality_effects {
        for qe in quality_effects.iter() {
            stmt.execute(named_params! {
                ":quality_id": &q.id,
                ":target_key": &qe.target_key,
                ":operation": &qe.operation,
                ":value_formula": &qe.value_formula,
                ":activation": &qe.activation,
                ":priority": &qe.priority
            })?;
        }
    }

    Ok(())
}
