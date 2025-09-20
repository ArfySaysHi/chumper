use super::repository::{
    create_priority_bundle, create_priority_bundle_metatypes, create_priority_bundle_modifiers,
    create_priority_bundle_qualities, create_priority_bundle_skills,
};
use crate::import::YamlImportable;
use crate::shared::Grade;
use crate::shared::{defaults::system, defaults::timestamp, Operation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriorityBundleModifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<i64>,
    pub target_key: String,
    #[serde(default)]
    pub operation: Operation,
    pub value: String,
    #[serde(default = "timestamp")]
    pub created_at: Option<String>,
    #[serde(default = "timestamp")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriorityBundle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub menu_order: i32,
    #[serde(default)]
    pub grade: Grade,
    #[serde(default = "system")]
    pub system: String,
    #[serde(default)]
    pub modifiers: Vec<PriorityBundleModifier>,
    #[serde(default)]
    pub skills: Vec<PriorityBundleSkill>,
    #[serde(default)]
    pub metatypes: Vec<PriorityBundleMetatype>,
    #[serde(default)]
    pub qualities: Vec<PriorityBundleQuality>,
    #[serde(default)]
    pub options: Vec<PriorityBundle>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriorityBundleOption {
    pub id: i64,
    pub bundle_id: i64,
    #[serde(default)]
    pub modifiers: Vec<PriorityBundleModifier>,
    #[serde(default)]
    pub skills: Vec<PriorityBundleSkill>,
    #[serde(default)]
    pub metatypes: Vec<PriorityBundleMetatype>,
    #[serde(default)]
    pub qualities: Vec<PriorityBundleQuality>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriorityBundleSkill {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<i64>,
    #[serde(default)]
    pub grade: Grade,
    pub attribute: String,
    #[serde(default)]
    pub amount: i32,
    #[serde(default)]
    pub rating: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriorityBundleMetatype {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<i64>,
    #[serde(default)]
    pub grade: Grade,
    #[serde(default)]
    pub special_points: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriorityBundleQuality {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<i64>,
    #[serde(default)]
    pub grade: Grade,
    pub name: String,
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
