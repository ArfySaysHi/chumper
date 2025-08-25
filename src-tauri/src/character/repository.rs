use super::commands::ListCharactersParams;
use super::resource;
use crate::character::types::*;
use crate::error::Result;
use crate::metatype::types::Metatype;
use resource::Resource;
use rusqlite::{named_params, params, Connection};
use std::collections::HashMap;

pub fn list_characters(
    connection: &Connection,
    params: Option<ListCharactersParams>,
) -> Result<Vec<CharacterSummary>> {
    let mut query = "SELECT
                       id, name, metatype, player_name, status, created_at, updated_at
                     FROM characters
                     WHERE 1=1"
        .to_string();
    let mut vals = vec![];

    if let Some(params) = params {
        if let Some(status) = params.status {
            query.push_str(" AND status = ?");
            vals.push(status);
        }
        let ordering = format!(
            " ORDER BY {} {}",
            params.sort_by.as_str(),
            params.sort_direction.as_str()
        );

        query.push_str(&ordering);
    }

    let mut stmt = connection.prepare(&query)?;

    let mut characters = stmt
        .query_map(rusqlite::params_from_iter(vals.iter()), |row| {
            Ok(CharacterSummary {
                id: row.get("id")?,
                name: row.get("name")?,
                metatype: row.get("metatype")?,
                player_name: row.get("player_name")?,
                status: row.get("status")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
                resources: Vec::new(),
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    let character_ids: Vec<i64> = characters.iter().filter_map(|c| c.id).collect();
    if !character_ids.is_empty() {
        let placeholders = character_ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");
        let resources_query = format!(
            "SELECT id, name, base_amount, current_amount, character_id
             FROM resources
             WHERE character_id IN ({})",
            placeholders
        );

        let mut resources_stmt = connection.prepare(&resources_query)?;
        let resources_iter =
            resources_stmt.query_map(rusqlite::params_from_iter(character_ids.iter()), |row| {
                Ok(super::resource::Resource {
                    id: Some(row.get("id")?),
                    name: row.get("name")?,
                    base_amount: row.get("base_amount")?,
                    current_amount: row.get("current_amount")?,
                    character_id: row.get("character_id")?,
                })
            })?;

        let mut resources_map: HashMap<i64, Vec<Resource>> = HashMap::new();
        for resource in resources_iter {
            let result = resource?;
            resources_map
                .entry(result.character_id)
                .or_insert_with(Vec::new)
                .push(result);
        }

        for character in &mut characters {
            if let Some(character_id) = character.id {
                if let Some(resources) = resources_map.remove(&character_id) {
                    character.resources = resources;
                }
            }
        }
    }

    Ok(characters)
}

pub fn get_character_by_id(connection: &Connection, id: i64) -> Result<Character> {
    let mut character = connection.query_row(
        "SELECT id, name, metatype, player_name, created_at, updated_at, status
         FROM characters
         WHERE id = ?1",
        params![id],
        |row| {
            Ok(Character {
                id: Some(row.get("id")?),
                name: row.get("name")?,
                metatype: row.get("metatype")?,
                player_name: row.get("player_name")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
                status: row.get("status")?,
                metatype_info: Metatype::new(row.get("name")?),
            })
        },
    )?;

    let metatype_query = format!(
        "SELECT (
           id, name, body_min, body_max, agility_min, agility_max, reaction_min, reaction_max,
           strength_min, strength_max, willpower_min, willpower_max, logic_min, logic_max,
           intuition_min, intuition_max, charisma_min, charisma_max, edge_min, edge_max,
           magical_type, magic_min, magic_max, resonance_min, resonance_max
         )
         FROM metatypes
         WHERE name = ({})",
        character.metatype
    );

    let metatype = connection.query_row(&metatype_query, [], |row| {
        Ok(Metatype {
            id: row.get("id")?,
            name: row.get("name")?,
            body_min: row.get("body_min")?,
            body_max: row.get("body_max")?,
            agility_min: row.get("agility_min")?,
            agility_max: row.get("agility_max")?,
            reaction_min: row.get("reaction_min")?,
            reaction_max: row.get("reaction_max")?,
            strength_min: row.get("strength_min")?,
            strength_max: row.get("strength_max")?,
            willpower_min: row.get("willpower_min")?,
            willpower_max: row.get("willpower_max")?,
            logic_min: row.get("logic_min")?,
            logic_max: row.get("logic_max")?,
            intuition_min: row.get("intuition_min")?,
            intuition_max: row.get("intuition_max")?,
            charisma_min: row.get("charisma_min")?,
            charisma_max: row.get("charisma_max")?,
            edge_min: row.get("edge_min")?,
            edge_max: row.get("edge_max")?,
            magical_type: row.get("magical_type")?,
            magic_min: row.get("magic_min")?,
            magic_max: row.get("magic_max")?,
            resonance_min: row.get("resonance_min")?,
            resonance_max: row.get("resonance_max")?,
            priority_grades: Vec::new(),
        })
    })?;

    character.metatype_info = metatype;

    Ok(character)
}

// TODO: Change the create character method to take priority params to set base stats / metatype / resources
pub fn create_character(connection: &mut Connection, character: &Character) -> Result<Character> {
    connection.execute(
        "INSERT INTO characters
           (name, metatype, player_name, created_at, updated_at, status)
         VALUES (
           :name, :metatype, :player_name, datetime('now'), datetime('now'), :status
         )",
        named_params!(
            ":name": character.name,
            ":metatype": character.metatype,
            ":player_name": character.player_name,
            ":status": character.status
        ),
    )?;

    let row_id = connection.last_insert_rowid();
    let mut created_character = character.clone();
    created_character.id = Some(row_id);
    created_character.initialize_base_resources(connection)?;

    Ok(created_character)
}

pub fn delete_character(connection: &Connection, id: i64) -> Result<String> {
    let mut stmt =
        connection.prepare(format!("DELETE FROM characters WHERE id = {}", id).as_str())?;
    stmt.execute([])?;

    Ok("Character deleted successfully".to_string())
}

pub fn archive_character(connection: &Connection, id: i64) -> Result<String> {
    let mut stmt = connection.prepare(
        format!(
            "UPDATE characters SET status = 'Archived' WHERE id = {}",
            id
        )
        .as_str(),
    )?;
    stmt.execute([])?;

    Ok("Character archived successfully".to_string())
}
