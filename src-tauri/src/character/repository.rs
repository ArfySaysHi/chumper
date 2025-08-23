use super::resource;
use super::{attribute::Attribute, commands::ListCharactersParams};
use crate::character::types::*;
use crate::error::Result;
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
                attributes: Attribute::new_defaults(row.get("id")?),
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

        let attributes_query = format!(
            "SELECT character_id, body, agility, reaction, strength, willpower,
                    logic, intuition, charisma, edge, magic, resonance
             FROM attributes
             WHERE character_id IN ({})",
            placeholders
        );
        let mut attributes_stmt = connection.prepare(&attributes_query)?;
        let attributes_iter =
            attributes_stmt.query_map(rusqlite::params_from_iter(character_ids.iter()), |row| {
                Ok(Attribute {
                    character_id: row.get("character_id")?,
                    body: row.get("body")?,
                    agility: row.get("agility")?,
                    reaction: row.get("reaction")?,
                    strength: row.get("strength")?,
                    willpower: row.get("willpower")?,
                    logic: row.get("logic")?,
                    intuition: row.get("intuition")?,
                    charisma: row.get("charisma")?,
                    edge: row.get("edge")?,
                    magic: row.get("magic")?,
                    resonance: row.get("resonance")?,
                })
            })?;
        let mut attributes_map: HashMap<i64, Attribute> = HashMap::new();
        for attribute in attributes_iter {
            let result = attribute?;
            attributes_map.insert(result.character_id, result);
        }

        for character in &mut characters {
            if let Some(character_id) = character.id {
                if let Some(resources) = resources_map.remove(&character_id) {
                    character.resources = resources;
                }
                if let Some(attribute) = attributes_map.remove(&character_id) {
                    character.attributes = attribute;
                }
            }
        }
    }

    Ok(characters)
}

pub fn get_character_by_id(connection: &Connection, id: i64) -> Result<Character> {
    let res = connection.query_row(
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
            })
        },
    )?;

    Ok(res)
}

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
    created_character.initialize_attributes(connection)?;

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
