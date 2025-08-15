use crate::models::character::{Character, CharacterSummary};
use crate::utils::error_handling::CommandResult;
use rusqlite::{Connection, Result};
use serde_yml::from_str;

pub struct CharacterController {
    db_path: String,
}

impl CharacterController {
    #[allow(dead_code)]
    pub fn new() -> Result<Self> {
        Ok(Self {
            db_path: "chumper.db3".to_string(),
        })
    }

    fn get_connection(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
    }

    pub fn list_characters(&self) -> CommandResult<Vec<CharacterSummary>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, metatype, player_name, karma_total, created_at, updated_at
             FROM characters
             WHERE is_active = TRUE
             ORDER BY updated_at DESC",
        )?;

        let character_iter = stmt.query_map([], |row| {
            Ok(CharacterSummary {
                id: row.get(0)?,
                name: row.get(1)?,
                metatype: row.get(2)?,
                player_name: row.get(3)?,
                karma_total: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;

        let mut characters = Vec::new();
        for character in character_iter {
            characters.push(character?)
        }

        Ok(characters)
    }

    pub fn create_character(&self, c: Character) -> CommandResult<i64> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "INSERT INTO characters (name, metatype, player_name, karma_total, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, datetime('now'), datetime('now'))",
        )?;
        stmt.execute(rusqlite::params![
            &c.name,
            &c.metatype,
            &c.player_name,
            &c.karma_total,
        ])?;

        Ok(conn.last_insert_rowid())
    }

    pub fn import_character(&self, yaml: &str) -> CommandResult<i64> {
        let mut character: Character = from_str(yaml)?;
        character.id = None; // Clear any existing ID
        let id = self.create_character(character)?; // Gets new auto-increment ID

        Ok(id)
    }
}
