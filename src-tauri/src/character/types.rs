use super::attribute::repository::create_attribute;
use super::attribute::Attribute;
use super::repository::create_character;
use super::resource::repository::create_resource;
use super::resource::{CreateResourceParams, Resource};
use crate::error::Result;
use crate::import::{YamlImportable, YamlSerializable};
use crate::metatype::types::Metatype;
use rusqlite::types::{FromSql, FromSqlError, ToSql, ToSqlOutput, Value, ValueRef};
use rusqlite::{Connection, Result as RusqliteResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Character {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub name: String,
    pub metatype: String,
    pub player_name: Option<String>,
    #[serde(default = "default_timestamp")]
    pub created_at: Option<String>,
    #[serde(default = "default_timestamp")]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub status: CharacterStatus,
    pub metatype_info: Metatype,
}

impl Character {
    pub fn initialize_base_resources(&self, connection: &mut Connection) -> Result<()> {
        let resources = vec![
            ("Essence", 6.0, 6.0),
            ("Edge", 1.0, 1.0),
            ("Nuyen", 0.0, 0.0),
            ("Karma", 0.0, 0.0),
        ];

        for (name, base, current) in resources {
            let params =
                CreateResourceParams::new(name.to_string(), base, current, self.id.unwrap());
            create_resource(connection, params)?;
        }

        Ok(())
    }

    pub fn initialize_attributes(&self, connection: &mut Connection) -> Result<()> {
        let attribute = Attribute::new_defaults(self.id.unwrap());
        create_attribute(connection, attribute)?;

        Ok(())
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
    pub attributes: Attribute,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCharacterRequest {
    pub name: String,
    pub metatype: String,
    pub player_name: Option<String>,
}
