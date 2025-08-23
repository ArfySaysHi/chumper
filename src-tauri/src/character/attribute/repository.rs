use super::Attribute;
use crate::error::Result;
use rusqlite::{named_params, Connection};

pub fn create_attribute(connection: &mut Connection, params: Attribute) -> Result<()> {
    let query = "INSERT INTO attributes
                 VALUES (
                   :character_id,
                   :body,
                   :agility,
                   :reaction,
                   :strength,
                   :willpower,
                   :logic,
                   :intuition,
                   :charisma,
                   :edge,
                   :magic,
                   :resonance
                 )";
    let mut stmt = connection.prepare(&query)?;
    stmt.execute(named_params! {
        ":character_id": params.character_id,
        ":body": params.body,
        ":agility": params.agility,
        ":reaction": params.reaction,
        ":strength": params.strength,
        ":willpower": params.willpower,
        ":logic": params.logic,
        ":intuition": params.intuition,
        ":charisma": params.charisma,
        ":edge": params.edge,
        ":magic": params.magic,
        ":resonance": params.resonance,
    })?;
    Ok(())
}
