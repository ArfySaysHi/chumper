use crate::models::metatype::{MagicalType, Metatype, MetatypeSummary};
use crate::traits::Connectable;
use crate::utils::error_handling::CommandResult;
use rusqlite::Result;

pub struct MetatypeController {
    pub db_path: String,
}

impl MetatypeController {
    pub fn new() -> Result<Self> {
        Ok(Self {
            db_path: "chumper.db3".to_string(),
        })
    }

    pub fn get_metatype(&self, name: &str) -> CommandResult<Metatype> {
        let conn = self.get_connection()?;

        let metatype = conn.query_row(
            "SELECT name, body_min, body_max, agility_min, agility_max,
                    reaction_min, reaction_max, strength_min, strength_max,
                    willpower_min, willpower_max, logic_min, logic_max,
                    intuition_min, intuition_max, charisma_min, charisma_max,
                    edge_min, edge_max, magical_type, magic_min, magic_max,
                    resonance_min, resonance_max
             FROM metatypes WHERE name = ?1",
            rusqlite::params![name],
            |row| {
                Ok(Metatype {
                    name: row.get(0)?,
                    body_min: row.get(1)?,
                    body_max: row.get(2)?,
                    agility_min: row.get(3)?,
                    agility_max: row.get(4)?,
                    reaction_min: row.get(5)?,
                    reaction_max: row.get(6)?,
                    strength_min: row.get(7)?,
                    strength_max: row.get(8)?,
                    willpower_min: row.get(9)?,
                    willpower_max: row.get(10)?,
                    logic_min: row.get(11)?,
                    logic_max: row.get(12)?,
                    intuition_min: row.get(13)?,
                    intuition_max: row.get(14)?,
                    charisma_min: row.get(15)?,
                    charisma_max: row.get(16)?,
                    edge_min: row.get(17)?,
                    edge_max: row.get(18)?,
                    magical_type: MagicalType::from_db_string(&row.get::<_, String>(19)?),
                    magic_min: row.get(20)?,
                    magic_max: row.get(21)?,
                    resonance_min: row.get(22)?,
                    resonance_max: row.get(23)?,
                })
            },
        )?;

        Ok(metatype)
    }

    pub fn list_metatypes(&self) -> CommandResult<Vec<MetatypeSummary>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT name, magical_type FROM metatypes ORDER BY name")?;

        let metatype_iter = stmt.query_map([], |row| {
            Ok(MetatypeSummary {
                name: row.get(0)?,
                magical_type: MagicalType::from_db_string(&row.get::<_, String>(1)?),
            })
        })?;

        let mut metatypes = Vec::new();
        for metatype in metatype_iter {
            metatypes.push(metatype?);
        }

        Ok(metatypes)
    }
}
