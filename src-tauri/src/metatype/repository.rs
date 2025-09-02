use std::collections::HashMap;

use super::types::{MetatypePriorityGrade, MetatypeQuality};
use crate::error::Result;
use crate::metatype::Metatype;
use rusqlite::{named_params, Connection};

pub fn list_metatypes(connection: &Connection) -> Result<Vec<Metatype>> {
    let query = "SELECT id, name, body_min, body_max, agility_min, agility_max,
                    reaction_min, reaction_max, strength_min, strength_max,
                    willpower_min, willpower_max, logic_min, logic_max,
                    intuition_min, intuition_max, charisma_min, charisma_max,
                    edge_min, edge_max, magical_type, magic_min, magic_max,
                    resonance_min, resonance_max
             FROM metatypes ORDER BY name"
        .to_string();

    let mut stmt = connection.prepare(&query)?;
    let mut metatypes = stmt
        .query_map([], |row| {
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
                metatype_qualities: Vec::new(),
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    if metatypes.is_empty() {
        return Ok(metatypes);
    }

    let mut stmt = connection.prepare(
        "SELECT special_attribute_bonus, metatype_name, grade
         FROM metatypes_priority_grades",
    )?;

    let mut grades_map: HashMap<String, Vec<MetatypePriorityGrade>> = HashMap::new();
    let grades_iter = stmt
        .query_map([], |row| {
            Ok(MetatypePriorityGrade {
                special_attribute_bonus: row.get("special_attribute_bonus")?,
                metatype_name: row.get("metatype_name")?,
                grade: row.get("grade")?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    for grade in grades_iter {
        if let Some(metatype_name) = &grade.metatype_name {
            grades_map
                .entry(metatype_name.clone())
                .or_insert_with(Vec::new)
                .push(grade.clone());
        }
    }

    let query = "SELECT metatype_name, quality_name, default_rating, created_at, updated_at
                 FROM metatype_qualities"
        .to_string();
    let mut stmt = connection.prepare(&query)?;
    let metatype_qualities = stmt
        .query_map([], |row| {
            Ok(MetatypeQuality {
                metatype_name: row.get("metatype_name")?,
                name: row.get("quality_name")?,
                default_rating: row.get("default_rating")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    let mut metatype_qualities_map: HashMap<String, Vec<MetatypeQuality>> = HashMap::new();
    for mq in metatype_qualities {
        match mq.metatype_name {
            Some(ref name) => metatype_qualities_map
                .entry(name.clone())
                .or_default()
                .push(mq),
            None => {
                continue;
            }
        }
    }

    for metatype in &mut metatypes {
        if let Some(grades) = grades_map.remove(&metatype.name) {
            metatype.priority_grades = grades;
        }

        if let Some(metatype_qualities) = metatype_qualities_map.remove(&metatype.name) {
            metatype.metatype_qualities = metatype_qualities;
        }
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
                metatype_qualities: Vec::new(),
            })
        },
    )?;
    let mut stmt = connection.prepare(
        "SELECT grade, special_attribute_bonus, metatype_name
         FROM metatypes_priority_grades
         WHERE metatype_name = ?1",
    )?;
    let grades_iter = stmt.query_map(rusqlite::params![name], |row| {
        Ok(MetatypePriorityGrade {
            grade: row.get("grade")?,
            metatype_name: row.get("metatype_name")?,
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

pub fn create_metatype_qualities(connection: &Connection, m: &Metatype) -> Result<()> {
    log::info!("create_metatype_qualities with {:?}", &m);
    for mq in &m.metatype_qualities {
        log::debug!(
            "Inserting metatype quality {} into metatype_qualities for {:?} if not exists",
            &mq.name,
            &mq.metatype_name
        );
        connection.execute(
            "INSERT INTO metatype_qualities (metatype_name, quality_name, default_rating)
             VALUES (:metatype_name, :quality_name, :default_rating)
             ON CONFLICT(metatype_name, quality_name)
             DO NOTHING",
            named_params! {
                ":metatype_name": &m.name,
                ":quality_name": &mq.name,
                ":default_rating": &mq.default_rating
            },
        )?;
    }

    Ok(())
}
