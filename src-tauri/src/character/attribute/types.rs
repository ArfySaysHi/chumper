use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Attribute {
    pub character_id: i64,
    #[serde(default = "default_attribute")]
    pub body: i32,
    #[serde(default = "default_attribute")]
    pub agility: i32,
    #[serde(default = "default_attribute")]
    pub reaction: i32,
    #[serde(default = "default_attribute")]
    pub strength: i32,
    #[serde(default = "default_attribute")]
    pub willpower: i32,
    #[serde(default = "default_attribute")]
    pub logic: i32,
    #[serde(default = "default_attribute")]
    pub intuition: i32,
    #[serde(default = "default_attribute")]
    pub charisma: i32,
    #[serde(default = "default_attribute")]
    pub edge: i32,
    #[serde(default)]
    pub magic: i32,
    #[serde(default)]
    pub resonance: i32,
}

impl Attribute {
    pub fn new_defaults(character_id: i64) -> Self {
        Self {
            character_id,
            ..Default::default()
        }
    }
}

fn default_attribute() -> i32 {
    1
}
