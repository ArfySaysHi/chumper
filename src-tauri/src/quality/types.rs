use crate::import::YamlImportable;
use crate::{error::Result, shared::Operation};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Quality {
    pub id: Option<i64>,
    pub name: String,
    pub resource_name: String,
    pub cost: i32,
    pub category: String,
    pub quality_modifiers: Option<Vec<QualityModifier>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct QualityModifier {
    pub id: Option<i64>,
    pub target_key: String,
    pub operation: Operation,
    pub value_formula: String,
    pub activation: String,
    pub priority: i32,
}

impl YamlImportable for Quality {
    type Output = Quality;
    fn insert_into_db(&self, connection: &Connection) -> Result<Self::Output> {
        let q = super::repository::create_quality(connection, &self)?;
        super::repository::create_quality_modifiers(connection, &q)?;

        Ok(q)
    }
}
