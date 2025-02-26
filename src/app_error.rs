use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid bot token format: {0}")]
    InvalidBotToken(String),
}
