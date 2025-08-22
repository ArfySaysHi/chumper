use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub name: String,
    #[serde(default)]
    pub base_amount: f32,
    #[serde(default)]
    pub current_amount: f32,
    pub character_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResourceParams {
    pub name: String,
    #[serde(default)]
    pub base_amount: f32,
    #[serde(default)]
    pub current_amount: f32,
    pub character_id: i64,
}
