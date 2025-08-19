use crate::character::*;

pub struct CharacterBuilder {
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

#[allow(dead_code)]
impl CharacterBuilder {
    pub fn new() -> Self {
        Self {
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

    pub fn build(self) -> Character {
        Character {
            id: None,
            name: self.name,
            metatype: self.metatype,
            player_name: self.player_name,
            body: self.body,
            agility: self.agility,
            reaction: self.reaction,
            strength: self.strength,
            willpower: self.willpower,
            logic: self.logic,
            intuition: self.intuition,
            charisma: self.charisma,
            edge: self.edge,
            magic: self.magic,
            resonance: self.resonance,
            karma_total: self.karma_total,
            nuyen: self.nuyen,
            created_at: None,
            updated_at: None,
            status: self.status,
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
