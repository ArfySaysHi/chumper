use super::Skill;
use crate::error::Result;
use rusqlite::{named_params, Connection};

pub fn create_skill(connection: &Connection, s: &Skill) -> Result<Skill> {
    log::info!("create_skill with {:#?}", &s);
    let query = "INSERT INTO skills
                   (name, attribute, can_default, skill_group)
                 VALUES (:name, :attribute, :can_default, :skill_group)"
        .to_string();
    let mut stmt = connection.prepare(&query)?;
    stmt.execute(named_params! {
        ":name": &s.name,
        ":attribute": &s.attribute,
        ":can_default": &s.can_default,
        ":skill_group": &s.skill_group
    })?;

    let row_id = connection.last_insert_rowid();
    let mut cloned_skill = s.clone();
    cloned_skill.id = Some(row_id);

    Ok(cloned_skill)
}

pub fn create_skill_group(connection: &Connection, s: &Skill) -> Result<Skill> {
    log::info!("create_skill_group with {:#?}", &s);
    let query = "INSERT INTO skill_groups (name) VALUES (:name) ON CONFLICT DO NOTHING".to_string();
    let mut stmt = connection.prepare(&query)?;
    stmt.execute(named_params! {
        ":name": &s.name
    })?;

    Ok(s.clone())
}

pub fn create_skill_specialization(connection: &Connection, s: &Skill) -> Result<Skill> {
    log::info!("create_skill_group with {:#?}", &s);
    let query = "INSERT INTO specializations (name, skill_id)
                 VALUES (:name, :skill_id)
                 ON CONFLICT DO NOTHING"
        .to_string();
    let mut stmt = connection.prepare(&query)?;

    if let Some(specs) = &s.specializations {
        for spec in specs.iter() {
            stmt.execute(named_params! {
                ":name": &spec.name,
                ":skill_id": &s.id
            })?;
        }
    }

    Ok(s.clone())
}
