use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateResourceParams {
    pub name: String,
    #[serde(default)]
    pub base_amount: f32,
    #[serde(default)]
    pub current_amount: f32,
    pub character_id: i64,
}

#[allow(dead_code)]
impl CreateResourceParams {
    pub fn new(
        name: impl Into<String>,
        base_amount: f32,
        current_amount: f32,
        character_id: i64,
    ) -> Self {
        Self {
            name: name.into(),
            base_amount,
            current_amount,
            character_id,
        }
    }
}
