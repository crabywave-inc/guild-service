use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum GuildError {
    #[error("Guild not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Failed to create guild: {0}")]
    CreateError(String),
}
