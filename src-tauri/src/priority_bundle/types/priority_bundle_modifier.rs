use crate::shared::defaults::timestamp;
use crate::shared::Operation;
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
