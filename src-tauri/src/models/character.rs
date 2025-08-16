use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    pub id: Option<i64>,
    pub name: String,
    pub metatype: String,
    pub player_name: Option<String>,
    pub body: i32,
    pub agility: i32,
    pub reaction: i32,
    pub strength: i32,
    pub willpower: i32,
    pub logic: i32,
    pub intuition: i32,
    pub charisma: i32,
    pub edge: i32,
    pub magic: Option<i32>,
    pub resonance: Option<i32>,
    pub karma_total: Option<i32>,
    pub karma_spent: Option<i32>,
    pub nuyen: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCharacterRequest {
    pub name: String,
    pub metatype: String,
    pub player_name: Option<String>,
}
