use serde::{ser::Serializer, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error(transparent)]
    RusqliteError(#[from] rusqlite::Error),
    #[error(transparent)]
    Error(#[from] std::io::Error),
    #[error(transparent)]
    YamlError(#[from] serde_yml::Error),
}

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type CommandResult<T, E = CommandError> = anyhow::Result<T, E>;
