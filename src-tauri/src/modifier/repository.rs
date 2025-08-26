use super::Modifier;
use crate::error::Result as Res;
use rusqlite::{named_params, Connection};

pub fn list_modifiers(connection: &Connection, character_id: i64) -> Res<Vec<Modifier>> {
    let mut stmt = connection.prepare(
        "SELECT id, name, character_id, origin_id, origin_type, target_key,
            operation, value, activation, condition_id, priority, stack_group,
            created_at, updated_at
         FROM modifiers
         WHERE character_id = :character_id
         ORDER BY priority ASC, id ASC",
    )?;
    let mods = stmt
        .query_map(named_params!(":character_id": character_id), |row| {
            Ok(Modifier {
                id: row.get("id")?,
                name: row.get("name")?,
                character_id: row.get("character_id")?,
                origin_id: row.get("origin_id")?,
                origin_type: row.get("origin_type")?,
                target_key: row.get("target_key")?,
                operation: row.get("operation")?,
                value_formula: row.get("value_formula")?,
                activation: row.get("activation")?,
                condition_id: row.get("condition_id")?,
                priority: row.get("priority")?,
                stack_group: row.get("stack_group")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(mods)
}

pub fn create_modifier(connection: &Connection, m: &Modifier) -> Res<Modifier> {
    let query = "INSERT INTO modifiers
                   (name, character_id, origin_id, origin_type, target_key, operation,
                    value_formula, activation, condition_id, priority, stack_group,
                    created_at, updated_at)
                 VALUES
                   (:name, :character_id, :origin_id, :origin_type, :target_key, :operation,
                    :value_formula, :activation, :condition_id, :priority, :stack_group,
                    (datetime('now')), (datetime('now')))"
        .to_string();

    let mut stmt = connection.prepare(&query)?;
    stmt.execute(named_params! {
        ":name": &m.name,
        ":character_id": &m.character_id,
        ":origin_id": &m.origin_id,
        ":origin_type": &m.origin_type,
        ":target_key": &m.target_key,
        ":operation": &m.operation,
        ":value_formula": &m.value_formula,
        ":activation": &m.activation,
        ":condition_id": &m.condition_id,
        ":priority": &m.priority,
        ":stack_group": &m.stack_group,
    })?;

    let row_id = connection.last_insert_rowid();
    let mut cloned_modifier = m.clone();
    cloned_modifier.id = Some(row_id);

    Ok(cloned_modifier)
}
