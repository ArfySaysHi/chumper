use super::repository::{create_metatype, create_metatype_qualities, get_metatype};
use crate::import::YamlImportable;
use rusqlite::{
    types::{FromSql, FromSqlError, ToSqlOutput, Value, ValueRef},
    Result, ToSql,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MetatypeQuality {
    pub metatype_name: Option<String>,
    pub name: String,
    #[serde(default = "default_rating")]
    pub default_rating: i32,
    #[serde(default = "default_timestamp")]
    pub created_at: Option<String>,
    #[serde(default = "default_timestamp")]
    pub updated_at: Option<String>,
}

fn default_rating() -> i32 {
    1
}

fn default_timestamp() -> Option<String> {
    Some("datetime('now')".to_string())
}

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
            MagicalType::Magic => Value::Text("Magic".into()),
            MagicalType::Resonance => Value::Text("Resonance".into()),
            MagicalType::Mundane => Value::Text("Mundane".into()),
        };

        Ok(ToSqlOutput::Owned(val))
    }
}

impl FromSql for MagicalType {
    fn column_result(value: ValueRef<'_>) -> Result<Self, FromSqlError> {
        match value {
            ValueRef::Text(text) => match text {
                b"Magic" => Ok(MagicalType::Magic),
                b"Resonance" => Ok(MagicalType::Resonance),
                b"Mundane" => Ok(MagicalType::Mundane),
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
    #[serde(default)]
    pub metatype_qualities: Vec<MetatypeQuality>,
}

impl Metatype {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}

impl YamlImportable for Metatype {
    type Output = Metatype;
    fn insert_into_db(
        &self,
        connection: &rusqlite::Connection,
    ) -> crate::error::Result<Self::Output> {
        create_metatype(connection, &self)?;
        create_metatype_qualities(connection, &self)?;

        let m = get_metatype(connection, &self.name)?;

        Ok(m)
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
