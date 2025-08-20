use super::commands::ListCharactersParams;
use crate::character::types::*;
use crate::error::Result;
use rusqlite::{params, Connection};

pub fn list_characters(
    connection: &Connection,
    params: Option<ListCharactersParams>,
) -> Result<Vec<CharacterSummary>> {
    let mut query = "SELECT
                       id, name, metatype, player_name, karma_total, status, created_at, updated_at
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

    let character_iter = stmt.query_map(rusqlite::params_from_iter(vals.iter()), |row| {
        Ok(CharacterSummary {
            id: row.get(0)?,
            name: row.get(1)?,
            metatype: row.get(2)?,
            player_name: row.get(3)?,
            karma_total: row.get(4)?,
            status: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;

    Ok(character_iter.collect::<rusqlite::Result<Vec<_>>>()?)
}

pub fn get_character_by_id(connection: &Connection, id: i64) -> Result<Character> {
    let res = connection.query_row(
        "SELECT id, name, metatype, player_name, body, agility, reaction,
                strength, willpower, logic, intuition, charisma, edge, magic,
                resonance, karma_total, nuyen, created_at, updated_at, status
         FROM characters
         WHERE id = ?1",
        params![id],
        |row| {
            Ok(Character {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                metatype: row.get(2)?,
                player_name: row.get(3)?,
                body: row.get(4)?,
                agility: row.get(5)?,
                reaction: row.get(6)?,
                strength: row.get(7)?,
                willpower: row.get(8)?,
                logic: row.get(9)?,
                intuition: row.get(10)?,
                charisma: row.get(11)?,
                edge: row.get(12)?,
                magic: row.get(13)?,
                resonance: row.get(14)?,
                karma_total: row.get(15)?,
                nuyen: row.get(16)?,
                created_at: row.get(17)?,
                updated_at: row.get(18)?,
                status: row.get(19)?,
            })
        },
    )?;

    Ok(res)
}

pub fn create_character(connection: &Connection, character: &Character) -> Result<Character> {
    connection.execute(
        "INSERT INTO characters
           (name, metatype, player_name, body, agility, reaction, strength,
            willpower, logic, intuition, charisma, edge, magic, resonance,
            karma_total, nuyen, created_at, updated_at, status)
         VALUES (
           ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14,
           ?15, ?16, datetime('now'), datetime('now'), ?17
         )",
        params![
            character.name,
            character.metatype,
            character.player_name,
            character.body,
            character.agility,
            character.reaction,
            character.strength,
            character.willpower,
            character.logic,
            character.intuition,
            character.charisma,
            character.edge,
            character.magic,
            character.resonance,
            character.karma_total,
            character.nuyen,
            character.status,
        ],
    )?;

    let row_id = connection.last_insert_rowid();
    let mut created_character = character.clone();
    created_character.id = Some(row_id);

    Ok(created_character)
}
