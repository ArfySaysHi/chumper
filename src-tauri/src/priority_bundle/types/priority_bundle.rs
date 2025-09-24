use super::{
    PriorityBundleMetatype, PriorityBundleModifier, PriorityBundleQuality, PriorityBundleSkill,
};
use crate::import::YamlImportable;
use crate::priority_bundle::repository::{
    create_priority_bundle, create_priority_bundle_metatypes, create_priority_bundle_modifiers,
    create_priority_bundle_qualities, create_priority_bundle_skills,
};
use crate::shared::defaults::system;
use crate::shared::Grade;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriorityBundle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i64>,
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
