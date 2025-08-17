use crate::models::character::{Character, CharacterStatus, CharacterSummary};
use crate::traits::controller::Connectable;
use crate::utils::error_handling::CommandResult;
use rusqlite::Result;
use serde_yml::from_str;

pub struct CharacterController {
    pub db_path: String,
}

impl CharacterController {
    pub fn new() -> Result<Self> {
        Ok(Self {
            db_path: "chumper.db3".to_string(),
        })
    }

    pub fn list_characters(&self) -> CommandResult<Vec<CharacterSummary>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, metatype, player_name, karma_total, status, created_at, updated_at
             FROM characters
             WHERE status != 'archived'
             ORDER BY updated_at DESC",
        )?;

        let character_iter = stmt.query_map([], |row| {
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

        let mut characters = Vec::new();
        for character in character_iter {
            characters.push(character?)
        }

        Ok(characters)
    }

    pub fn list_characters_by_status(
        &self,
        status: CharacterStatus,
    ) -> CommandResult<Vec<CharacterSummary>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, metatype, player_name, karma_total, status, created_at, updated_at
             FROM characters
             WHERE status = ?1
             ORDER BY updated_at DESC",
        )?;

        let character_iter = stmt.query_map([&status], |row| {
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

        let mut characters = Vec::new();
        for character in character_iter {
            characters.push(character?)
        }

        Ok(characters)
    }

    pub fn get_character(&self, id: i64) -> CommandResult<Character> {
        let conn = self.get_connection()?;
        let character = conn.query_row(
            "SELECT id, name, metatype, player_name, body, agility, reaction,
                    strength, willpower, logic, intuition, charisma, edge, magic,
                    resonance, karma_total, karma_spent, nuyen, created_at, updated_at,
                    status
             FROM characters
             WHERE id = ?1 AND active_id = TRUE",
            rusqlite::params![id],
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
                    karma_spent: row.get(16)?,
                    nuyen: row.get(17)?,
                    created_at: row.get(18)?,
                    updated_at: row.get(19)?,
                    status: row.get(20)?,
                })
            },
        )?;

        Ok(character)
    }

    pub fn create_character(&self, c: Character) -> CommandResult<i64> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "INSERT INTO characters
             name, metatype, player_name, body, agility, reaction, strength,
             willpower, logic, intuition, charisma, edge, magic, resonance,
             karma_total, karma_spent, nuyen, created_at, updated_at
             VALUES
             ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13,
             ?14, ?15, ?16, ?17, datetime('now'), datetime('now')",
        )?;
        stmt.execute(rusqlite::params![
            &c.name,
            &c.metatype,
            &c.player_name,
            &c.body,
            &c.agility,
            &c.reaction,
            &c.strength,
            &c.willpower,
            &c.logic,
            &c.intuition,
            &c.charisma,
            &c.edge,
            &c.magic,
            &c.resonance,
            &c.karma_total,
            &c.karma_spent,
            &c.nuyen,
        ])?;

        Ok(conn.last_insert_rowid())
    }

    pub fn import_character(&self, yaml: &str) -> CommandResult<i64> {
        let mut character: Character = from_str(yaml)?;
        character.id = None;
        let id = self.create_character(character)?;

        Ok(id)
    }

    pub fn export_character(&self, id: i64) -> CommandResult<String> {
        let character = self.get_character(id)?;
        Ok(serde_yml::to_string(&character)?)
    }
}
