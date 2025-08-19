use crate::error::Result;
use crate::metatype::{Metatype, MetatypeSummary};
use rusqlite::Connection;
use serde_yml::from_str;

pub fn list_metatypes(connection: &Connection) -> Result<Vec<MetatypeSummary>> {
    let mut stmt = connection.prepare("SELECT name, magical_type FROM metatypes ORDER BY name")?;

    let metatype_iter = stmt.query_map([], |row| {
        Ok(MetatypeSummary {
            name: row.get(0)?,
            magical_type: row.get(1)?,
        })
    })?;

    let mut metatypes = Vec::new();
    for metatype in metatype_iter {
        metatypes.push(metatype?);
    }

    Ok(metatypes)
}

pub fn get_metatype(connection: &Connection, name: &str) -> Result<Metatype> {
    let metatype = connection.query_row(
        "SELECT id, name, body_min, body_max, agility_min, agility_max,
                    reaction_min, reaction_max, strength_min, strength_max,
                    willpower_min, willpower_max, logic_min, logic_max,
                    intuition_min, intuition_max, charisma_min, charisma_max,
                    edge_min, edge_max, magical_type, magic_min, magic_max,
                    resonance_min, resonance_max
             FROM metatypes WHERE name = ?1",
        rusqlite::params![name],
        |row| {
            Ok(Metatype {
                id: row.get(0)?,
                name: row.get(1)?,
                body_min: row.get(2)?,
                body_max: row.get(3)?,
                agility_min: row.get(4)?,
                agility_max: row.get(5)?,
                reaction_min: row.get(6)?,
                reaction_max: row.get(7)?,
                strength_min: row.get(8)?,
                strength_max: row.get(9)?,
                willpower_min: row.get(10)?,
                willpower_max: row.get(11)?,
                logic_min: row.get(12)?,
                logic_max: row.get(13)?,
                intuition_min: row.get(14)?,
                intuition_max: row.get(15)?,
                charisma_min: row.get(16)?,
                charisma_max: row.get(17)?,
                edge_min: row.get(18)?,
                edge_max: row.get(19)?,
                magical_type: row.get(20)?,
                magic_min: row.get(21)?,
                magic_max: row.get(22)?,
                resonance_min: row.get(23)?,
                resonance_max: row.get(24)?,
            })
        },
    )?;

    Ok(metatype)
}

pub fn create_metatype(connection: &Connection, m: &Metatype) -> Result<Metatype> {
    let mut stmt = connection.prepare(
        "INSERT INTO metatypes (
               name, body_min, body_max, agility_min, agility_max,
               reaction_min, reaction_max, strength_min, strength_max,
               willpower_min, willpower_max, logic_min, logic_max,
               intuition_min, intuition_max, charisma_min, charisma_max,
               edge_min, edge_max, magical_type, magic_min, magic_max,
               resonance_min, resonance_max
             )
             VALUES (
               ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14,
               ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24
             )",
    )?;

    stmt.execute(rusqlite::params![
        &m.name,
        &m.body_min,
        &m.body_max,
        &m.agility_min,
        &m.agility_max,
        &m.reaction_min,
        &m.reaction_max,
        &m.strength_min,
        &m.strength_max,
        &m.willpower_min,
        &m.willpower_max,
        &m.logic_min,
        &m.logic_max,
        &m.intuition_min,
        &m.intuition_max,
        &m.charisma_min,
        &m.charisma_max,
        &m.edge_min,
        &m.edge_max,
        &m.magical_type,
        &m.magic_min,
        &m.magic_max,
        &m.resonance_min,
        &m.resonance_max
    ])?;

    let row_id = connection.last_insert_rowid();
    let mut created_metatype = m.clone();
    created_metatype.id = Some(row_id);

    Ok(created_metatype)
}

pub fn import_metatype(connection: &Connection, yaml: &str) -> Result<Metatype> {
    let metatype: Metatype = from_str(yaml)?;
    let created_metatype = create_metatype(connection, &metatype)?;

    Ok(created_metatype)
}
