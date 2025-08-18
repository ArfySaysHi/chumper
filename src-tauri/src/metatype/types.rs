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

#[derive(Debug, Serialize, Deserialize)]
pub struct Metatype {
    pub name: String,
    pub body_min: Option<i32>,
    pub body_max: Option<i32>,
    pub agility_min: Option<i32>,
    pub agility_max: Option<i32>,
    pub reaction_min: Option<i32>,
    pub reaction_max: Option<i32>,
    pub strength_min: Option<i32>,
    pub strength_max: Option<i32>,
    pub willpower_min: Option<i32>,
    pub willpower_max: Option<i32>,
    pub logic_min: Option<i32>,
    pub logic_max: Option<i32>,
    pub intuition_min: Option<i32>,
    pub intuition_max: Option<i32>,
    pub charisma_min: Option<i32>,
    pub charisma_max: Option<i32>,
    pub edge_min: Option<i32>,
    pub edge_max: Option<i32>,
    pub magical_type: MagicalType,
    pub magic_min: Option<i32>,
    pub magic_max: Option<i32>,
    pub resonance_min: Option<i32>,
    pub resonance_max: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetatypeSummary {
    pub name: String,
    pub magical_type: MagicalType,
}
