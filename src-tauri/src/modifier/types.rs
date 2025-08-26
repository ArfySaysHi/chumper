use crate::import::YamlImportable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Modifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    pub character_id: i64,
    #[serde(default)]
    pub origin_id: Option<i64>,
    #[serde(default)]
    pub origin_type: Option<String>,
    pub target_key: String,
    pub operation: String,
    #[serde(default)]
    pub value_formula: String,
    pub activation: String,
    #[serde(default)]
    pub condition_id: Option<i64>,
    #[serde(default = "default_priority")]
    pub priority: i64,
    #[serde(default)]
    pub stack_group: Option<String>,
    #[serde(default = "default_timestamp")]
    pub created_at: Option<String>,
    #[serde(default = "default_timestamp")]
    pub updated_at: Option<String>,
}

impl YamlImportable for Modifier {
    type Output = Modifier;
    fn insert_into_db(
        &self,
        connection: &rusqlite::Connection,
    ) -> crate::error::Result<Self::Output>
    where
        Self: Sized,
    {
        super::repository::create_modifier(connection, &self)
    }
}

fn default_priority() -> i64 {
    100
}

fn default_timestamp() -> Option<String> {
    Some("datetime('now')".to_string())
}
