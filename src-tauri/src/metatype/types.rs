use super::repository::{create_metatype, create_metatype_qualities, get_metatype};
use crate::import::YamlImportable;
use crate::shared::defaults::{attribute_max, attribute_min, rating, timestamp};
use rusqlite::{
    types::{FromSql, FromSqlError, ToSqlOutput, Value, ValueRef},
    Result, ToSql,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MetatypeQuality {
    pub metatype_name: Option<String>,
    pub name: String,
    #[serde(default = "rating")]
    pub default_rating: i32,
    #[serde(default = "timestamp")]
    pub created_at: Option<String>,
    #[serde(default = "timestamp")]
    pub updated_at: Option<String>,
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
    #[serde(default = "attribute_min")]
    pub body_min: i32,
    #[serde(default = "attribute_max")]
    pub body_max: i32,
    #[serde(default = "attribute_min")]
    pub agility_min: i32,
    #[serde(default = "attribute_max")]
    pub agility_max: i32,
    #[serde(default = "attribute_min")]
    pub reaction_min: i32,
    #[serde(default = "attribute_max")]
    pub reaction_max: i32,
    #[serde(default = "attribute_min")]
    pub strength_min: i32,
    #[serde(default = "attribute_max")]
    pub strength_max: i32,
    #[serde(default = "attribute_min")]
    pub willpower_min: i32,
    #[serde(default = "attribute_max")]
    pub willpower_max: i32,
    #[serde(default = "attribute_min")]
    pub logic_min: i32,
    #[serde(default = "attribute_max")]
    pub logic_max: i32,
    #[serde(default = "attribute_min")]
    pub intuition_min: i32,
    #[serde(default = "attribute_max")]
    pub intuition_max: i32,
    #[serde(default = "attribute_min")]
    pub charisma_min: i32,
    #[serde(default = "attribute_max")]
    pub charisma_max: i32,
    #[serde(default = "attribute_min")]
    pub edge_min: i32,
    #[serde(default = "attribute_max")]
    pub edge_max: i32,
    #[serde(default)]
    pub magical_type: MagicalType,
    #[serde(default)]
    pub magic_min: i32,
    #[serde(default = "attribute_max")]
    pub magic_max: i32,
    #[serde(default)]
    pub resonance_min: i32,
    #[serde(default = "attribute_max")]
    pub resonance_max: i32,
    #[serde(default)]
    pub metatype_qualities: Vec<MetatypeQuality>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MetatypeSummary {
    pub name: String,
    pub magical_type: MagicalType,
}
