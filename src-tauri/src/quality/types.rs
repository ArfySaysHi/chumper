use crate::error::Result;
use crate::import::YamlImportable;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Quality {
    pub id: Option<i64>,
    pub name: String,
    pub resource_name: String,
    pub cost: i32,
    pub category: String,
    pub quality_effects: Option<Vec<QualityEffect>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct QualityEffect {
    pub id: Option<i64>,
    pub target_key: String,
    pub operation: String,
    pub value_formula: String,
    pub activation: String,
    pub priority: i32,
}

impl YamlImportable for Quality {
    type Output = Quality;
    // TODO: Transactions for all of the inserts that require it (which means removing mut on all)
    fn insert_into_db(&self, connection: &Connection) -> Result<Self::Output> {
        let q = super::repository::create_quality(connection, &self)?;
        super::repository::create_quality_effects(connection, &q)?;

        Ok(q)
    }
}
