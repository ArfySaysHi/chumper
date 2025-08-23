use super::repository::create_metatype;
use crate::import::{YamlImportable, YamlSerializable};
use rusqlite::{
    types::{FromSql, FromSqlError, ToSqlOutput, Value, ValueRef},
    Result, ToSql,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MagicalType {
    Magic,
    Resonance,
    Mundane,
}

impl Default for MagicalType {
    fn default() -> Self {
        Self::Mundane
    }
}

impl ToSql for MagicalType {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let val = match self {
            MagicalType::Magic => Value::Text("magic".into()),
            MagicalType::Resonance => Value::Text("resonance".into()),
            MagicalType::Mundane => Value::Text("mundane".into()),
        };

        Ok(ToSqlOutput::Owned(val))
    }
}

impl FromSql for MagicalType {
    fn column_result(value: ValueRef<'_>) -> Result<Self, FromSqlError> {
        match value {
            ValueRef::Text(text) => match text {
                b"magic" => Ok(MagicalType::Magic),
                b"resonance" => Ok(MagicalType::Resonance),
                b"mundane" => Ok(MagicalType::Mundane),
                _ => Err(FromSqlError::InvalidType),
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Metatype {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub name: String,
    #[serde(default = "default_minimum")]
    pub body_min: i32,
    #[serde(default = "default_maximum")]
    pub body_max: i32,
    #[serde(default = "default_minimum")]
    pub agility_min: i32,
    #[serde(default = "default_maximum")]
    pub agility_max: i32,
    #[serde(default = "default_minimum")]
    pub reaction_min: i32,
    #[serde(default = "default_maximum")]
    pub reaction_max: i32,
    #[serde(default = "default_minimum")]
    pub strength_min: i32,
    #[serde(default = "default_maximum")]
    pub strength_max: i32,
    #[serde(default = "default_minimum")]
    pub willpower_min: i32,
    #[serde(default = "default_maximum")]
    pub willpower_max: i32,
    #[serde(default = "default_minimum")]
    pub logic_min: i32,
    #[serde(default = "default_maximum")]
    pub logic_max: i32,
    #[serde(default = "default_minimum")]
    pub intuition_min: i32,
    #[serde(default = "default_maximum")]
    pub intuition_max: i32,
    #[serde(default = "default_minimum")]
    pub charisma_min: i32,
    #[serde(default = "default_maximum")]
    pub charisma_max: i32,
    #[serde(default = "default_minimum")]
    pub edge_min: i32,
    #[serde(default = "default_maximum")]
    pub edge_max: i32,
    #[serde(default)]
    pub magical_type: MagicalType,
    #[serde(default)]
    pub magic_min: i32,
    #[serde(default = "default_maximum")]
    pub magic_max: i32,
    #[serde(default)]
    pub resonance_min: i32,
    #[serde(default = "default_maximum")]
    pub resonance_max: i32,
}

impl Metatype {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}

impl YamlSerializable for Metatype {}

impl YamlImportable for Metatype {
    fn insert_into_db(
        &self,
        connection: &mut rusqlite::Connection,
    ) -> crate::error::Result<Metatype> {
        create_metatype(connection, &self)
    }
}

fn default_minimum() -> i32 {
    1
}

fn default_maximum() -> i32 {
    6
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetatypeSummary {
    pub name: String,
    pub magical_type: MagicalType,
}
