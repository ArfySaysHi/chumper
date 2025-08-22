use crate::error::Result;
use crate::import::{YamlImportable, YamlSerializable};
use rusqlite::types::{FromSql, FromSqlError, ToSql, ToSqlOutput, Value, ValueRef};
use rusqlite::{Connection, Result as RusqliteResult};
use serde::{Deserialize, Serialize};

use super::builder::CharacterBuilder;
use super::repository::create_character;
use super::resource::Resource;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub name: String,
    pub metatype: String,
    pub player_name: Option<String>,

    // Attributes
    #[serde(default = "default_attribute")]
    pub body: i32,
    #[serde(default = "default_attribute")]
    pub agility: i32,
    #[serde(default = "default_attribute")]
    pub reaction: i32,
    #[serde(default = "default_attribute")]
    pub strength: i32,
    #[serde(default = "default_attribute")]
    pub willpower: i32,
    #[serde(default = "default_attribute")]
    pub logic: i32,
    #[serde(default = "default_attribute")]
    pub intuition: i32,
    #[serde(default = "default_attribute")]
    pub charisma: i32,

    // Special Attributes
    #[serde(default)]
    pub edge: i32,
    #[serde(default)]
    pub magic: i32,
    #[serde(default)]
    pub resonance: i32,
    #[serde(default = "default_timestamp")]
    pub created_at: Option<String>,
    #[serde(default = "default_timestamp")]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub status: CharacterStatus,
}

impl From<CharacterBuilder> for Character {
    fn from(c: CharacterBuilder) -> Self {
        Self {
            id: c.id,
            name: c.name,
            metatype: c.metatype,
            player_name: c.player_name,
            body: c.body,
            agility: c.agility,
            reaction: c.reaction,
            strength: c.strength,
            willpower: c.willpower,
            logic: c.logic,
            intuition: c.intuition,
            charisma: c.charisma,
            edge: c.edge,
            magic: c.magic,
            resonance: c.resonance,
            created_at: None,
            updated_at: None,
            status: c.status,
        }
    }
}

impl YamlSerializable for Character {}

impl YamlImportable for Character {
    fn insert_into_db(&self, connection: &mut Connection) -> Result<Character> {
        create_character(connection, &self)
    }
}

impl Default for CharacterStatus {
    fn default() -> Self {
        CharacterStatus::Creation
    }
}

fn default_attribute() -> i32 {
    1
}
fn default_timestamp() -> Option<String> {
    Some("datetime('now')".to_string())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CharacterStatus {
    Creation,
    Active,
    Archived,
}

impl ToSql for CharacterStatus {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput<'_>> {
        let val = match self {
            CharacterStatus::Active => Value::Text("Active".into()),
            CharacterStatus::Creation => Value::Text("Creation".into()),
            CharacterStatus::Archived => Value::Text("Archived".into()),
        };
        Ok(ToSqlOutput::Owned(val))
    }
}

impl FromSql for CharacterStatus {
    fn column_result(value: ValueRef<'_>) -> RusqliteResult<Self, FromSqlError> {
        match value {
            ValueRef::Text(text) => match text {
                b"Active" => Ok(CharacterStatus::Active),
                b"Creation" => Ok(CharacterStatus::Creation),
                b"Archived" => Ok(CharacterStatus::Archived),
                _ => Err(FromSqlError::InvalidType),
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSummary {
    pub id: Option<i64>,
    pub name: String,
    pub metatype: String,
    pub player_name: Option<String>,
    pub status: CharacterStatus,
    pub created_at: String,
    pub updated_at: String,
    pub resources: Vec<Resource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCharacterRequest {
    pub name: String,
    pub metatype: String,
    pub player_name: Option<String>,
}
