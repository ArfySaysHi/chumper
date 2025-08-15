use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    pub id: Option<i64>,
    pub name: String,
    pub metatype: String,
    pub player_name: Option<String>,
    pub karma_total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSummary {
    pub id: Option<i64>,
    pub name: String,
    pub metatype: String,
    pub player_name: Option<String>,
    pub karma_total: i32,
    pub created_at: String,
    pub updated_at: String,
}
