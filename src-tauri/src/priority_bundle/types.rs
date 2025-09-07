use super::repository::{
    create_priority_bundle, create_priority_bundle_metatypes, create_priority_bundle_modifiers,
    create_priority_bundle_qualities, create_priority_bundle_skills,
};
use crate::import::YamlImportable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriorityBundleModifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<i64>,
    #[serde(default = "default_wildcard")]
    pub grade: String,
    pub target_key: String,
    #[serde(default = "default_operation")]
    pub operation: String,
    pub value_formula: String,
    #[serde(default = "default_timestamp")]
    pub created_at: Option<String>,
    #[serde(default = "default_timestamp")]
    pub updated_at: Option<String>,
}

fn default_operation() -> String {
    "add".to_string()
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
    #[serde(default = "default_wildcard")]
    pub grade: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_bundle_id: Option<i64>,
    #[serde(default)]
    pub priority_bundle_modifiers: Vec<PriorityBundleModifier>,
    #[serde(default)]
    pub priority_bundle_skills: Vec<PriorityBundleSkill>,
    #[serde(default)]
    pub priority_bundle_metatypes: Vec<PriorityBundleMetatype>,
    #[serde(default)]
    pub priority_bundle_qualities: Vec<PriorityBundleQuality>,
    #[serde(default)]
    pub priority_bundles: Vec<PriorityBundle>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriorityBundleSkill {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<i64>,
    #[serde(default = "default_wildcard")]
    pub grade: String,
    pub attribute: String,
    #[serde(default)]
    pub amount: i32,
    #[serde(default)]
    pub default_rating: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriorityBundleMetatype {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<i64>,
    #[serde(default = "default_wildcard")]
    pub grade: String,
    #[serde(default)]
    pub special_attribute_bonus: i32,
    pub metatype_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriorityBundleQuality {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<i64>,
    #[serde(default = "default_wildcard")]
    pub grade: String,
    pub quality_name: String,
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
        create_priority_bundle_skills(connection, &res)?;
        create_priority_bundle_metatypes(connection, &res)?;
        create_priority_bundle_qualities(connection, &res)?;

        Ok(res)
    }
}
