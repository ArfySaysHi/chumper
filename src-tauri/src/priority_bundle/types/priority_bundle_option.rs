use super::{
    PriorityBundleMetatype, PriorityBundleModifier, PriorityBundleQuality, PriorityBundleSkill,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriorityBundleOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<i64>,
    #[serde(default)]
    pub modifiers: Vec<PriorityBundleModifier>,
    #[serde(default)]
    pub skills: Vec<PriorityBundleSkill>,
    #[serde(default)]
    pub metatypes: Vec<PriorityBundleMetatype>,
    #[serde(default)]
    pub qualities: Vec<PriorityBundleQuality>,
}
