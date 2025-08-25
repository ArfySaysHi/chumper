use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Query conversion type error: {0}")]
    QueryConversion(#[from] rusqlite::types::FromSqlError),

    #[error("Tokio join error: {0}")]
    TokioJoin(#[from] tokio::task::JoinError),

    #[error("Yaml parsing error: {0}")]
    YamlParse(#[from] serde_yml::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Db pool error: {0}")]
    DbPool(#[from] r2d2::Error),

    #[error("Character error: {0}")]
    Character(#[from] CharacterError),
}

// TODO: Add more once we have filled out the validations etc
#[derive(Error, Debug)]
pub enum CharacterError {
    #[error("Character not found with id: {id}")]
    NotFound { id: i64 },
}
