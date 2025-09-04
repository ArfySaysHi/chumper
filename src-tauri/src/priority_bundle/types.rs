use crate::import::YamlImportable;
use serde::{Deserialize, Serialize};

use super::repository::{create_priority_bundle, create_priority_bundle_modifiers};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriorityBundleModifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<i64>,
    #[serde(default = "default_wildcard")]
    pub grade: String,
    pub target_key: String,
    pub operation: String,
    pub value_formula: String,
    #[serde(default = "default_timestamp")]
    pub created_at: Option<String>,
    #[serde(default = "default_timestamp")]
    pub updated_at: Option<String>,
}

fn default_wildcard() -> String {
    "*".to_string()
}

fn default_timestamp() -> Option<String> {
    Some("datetime('now')".to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriorityBundle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub domain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_bundle_id: Option<i64>,
    #[serde(default)]
    pub priority_bundle_modifiers: Vec<PriorityBundleModifier>,
    #[serde(default)]
    pub priority_bundles: Vec<PriorityBundle>,
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
