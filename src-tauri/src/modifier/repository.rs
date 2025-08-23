use super::Modifier;
use crate::error::Result as Res;
use rusqlite::{named_params, Connection};

pub fn list_modifiers(connection: &mut Connection, character_id: i64) -> Res<Vec<Modifier>> {
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
                value: row.get("value")?,
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
