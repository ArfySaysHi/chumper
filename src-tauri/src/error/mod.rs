pub mod conversion;
pub mod types;

pub use types::*;

pub type Result<T> = std::result::Result<T, AppError>;
