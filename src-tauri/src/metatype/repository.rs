use super::types::MetatypePriorityGrade;
use crate::error::Result;
use crate::metatype::{Metatype, MetatypeSummary};
use rusqlite::{named_params, Connection};

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
                priority_grades: Vec::new(),
            })
        },
    )?;
    let mut stmt = connection.prepare(
        "SELECT grade, special_attribute_bonus
         FROM metatypes_priority_grades
         WHERE metatype_name = ?1",
    )?;
    let grades_iter = stmt.query_map(rusqlite::params![name], |row| {
        Ok(MetatypePriorityGrade {
            grade: row.get("grade")?,
            special_attribute_bonus: row.get("special_attribute_bonus")?,
        })
    })?;

    let mut priority_grades = Vec::new();
    for g in grades_iter {
        priority_grades.push(g?);
    }

    let mut metatype = metatype;
    metatype.priority_grades = priority_grades;

    Ok(metatype)
}

pub fn create_metatype(connection: &Connection, m: &Metatype) -> Result<Metatype> {
    log::info!("create_metatype with {:#?}", &m);
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

    let id = connection.last_insert_rowid();
    let mut cloned_metatype = m.clone();
    cloned_metatype.id = Some(id);

    Ok(cloned_metatype)
}

pub fn create_metatype_priority_grades(connection: &Connection, m: &Metatype) -> Result<()> {
    log::info!("create_metatype_priority_grades with {:#?}", &m);
    for pg in &m.priority_grades {
        log::debug!(
            "Inserting grade {:?} into priority_grades for {} if not exists",
            &pg.grade,
            &m.name
        );
        connection.execute(
            "INSERT INTO priority_grades (grade) VALUES (:grade) ON CONFLICT(grade) DO NOTHING",
            named_params! { ":grade": &pg.grade},
        )?;
        connection.execute(
            "INSERT INTO
               metatypes_priority_grades (special_attribute_bonus, metatype_name, grade)
             VALUES
               (:special_attribute_bonus, :metatype_name, :grade)
             ON CONFLICT(metatype_name, grade)
             DO UPDATE SET special_attribute_bonus = excluded.special_attribute_bonus",
            named_params! {
                ":special_attribute_bonus": &pg.special_attribute_bonus,
                ":metatype_name": &m.name,
                ":grade": &pg.grade
            },
        )?;
    }

    Ok(())
}
