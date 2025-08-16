use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MagicalType {
    Magic,
    Resonance,
    Mundane,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metatype {
    pub name: String,
    pub body_min: i32,
    pub body_max: i32,
    pub agility_min: i32,
    pub agility_max: i32,
    pub reaction_min: i32,
    pub reaction_max: i32,
    pub strength_min: i32,
    pub strength_max: i32,
    pub willpower_min: i32,
    pub willpower_max: i32,
    pub logic_min: i32,
    pub logic_max: i32,
    pub intuition_min: i32,
    pub intuition_max: i32,
    pub charisma_min: i32,
    pub charisma_max: i32,
    pub edge_min: i32,
    pub edge_max: i32,
    pub magical_type: MagicalType,
    pub magic_min: i32,
    pub magic_max: i32,
    pub resonance_min: i32,
    pub resonance_max: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetatypeSummary {
    pub name: String,
    pub magical_type: MagicalType,
}

// Database storage - just store as TEXT
impl MagicalType {
    pub fn to_db_string(&self) -> String {
        match self {
            MagicalType::Magic => "magic".to_string(),
            MagicalType::Resonance => "resonance".to_string(),
            MagicalType::Mundane => "mundane".to_string(),
        }
    }

    pub fn from_db_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "magic" => MagicalType::Magic,
            "resonance" => MagicalType::Resonance,
            _ => MagicalType::Mundane,
        }
    }
}
