use crate::models::character::Character;
use rusqlite::Result;
use tauri::{AppHandle, Emitter};

pub struct CharacterService {
    app_handle: AppHandle,
}

impl CharacterService {
    pub fn new(app_handle: AppHandle) -> Result<Self> {
        Ok(Self { app_handle })
    }

    pub fn character_created(&self, character: &Character) {
        self.app_handle.emit("character_created", character);
    }
}
