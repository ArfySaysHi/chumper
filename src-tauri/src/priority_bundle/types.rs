use crate::import::YamlImportable;
use serde::{Deserialize, Serialize};

use super::repository::{create_priority_bundle, create_priority_bundle_modifiers};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriorityBundleModifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub grade: String,
    pub target_key: String,
    pub operation: String,
    pub value_formula: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriorityBundle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub domain: String,
    pub priority_bundle_modifiers: Vec<PriorityBundleModifier>,
}

impl YamlImportable for PriorityBundle {
    type Output = Self;
    fn insert_into_db(
        &self,
        connection: &rusqlite::Connection,
    ) -> crate::error::Result<Self::Output>
    where
        Self: Sized,
    {
        let res = create_priority_bundle(connection, &self)?;
        create_priority_bundle_modifiers(connection, &res)?;

        Ok(res)
    }
}
