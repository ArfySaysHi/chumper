use crate::character::*;

pub struct CharacterBuilder {
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
    pub magic: i32,
    pub resonance: i32,
    pub karma_total: i32,
    pub nuyen: i32,
    pub status: CharacterStatus,
}

#[allow(dead_code)]
impl CharacterBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            name: String::new(),
            metatype: "Human".to_string(),
            player_name: None,
            body: 1,
            agility: 1,
            reaction: 1,
            strength: 1,
            willpower: 1,
            logic: 1,
            intuition: 1,
            charisma: 1,
            edge: 1,
            magic: 0,
            resonance: 0,
            karma_total: 0,
            nuyen: 0,
            status: CharacterStatus::Creation,
        }
    }

    crate::builder_setters! {
    name: String,
    metatype: String,
    player_name: Option<String>,
    body: i32,
    agility: i32,
    reaction: i32,
    strength: i32,
    willpower: i32,
    logic: i32,
    intuition: i32,
    charisma: i32,
    edge: i32,
    magic: i32,
    resonance: i32,
    karma_total: i32,
    nuyen: i32,
    status: CharacterStatus,
    }
}
