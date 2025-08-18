use super::types::*;
use std::string::String;

impl From<AppError> for String {
    fn from(err: AppError) -> String {
        match err {
            AppError::Character(CharacterError::NotFound { id }) => {
                format!("Character with ID {} was not found", id)
            }
            _ => err.to_string(),
        }
    }
}
