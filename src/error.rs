use std::fmt;

#[derive(Debug)]
pub enum AppError {
    InvalidBotToken(String),
    InvalidChatId(String),
    TelegramError(String),
    BatteryError(String),
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidBotToken(msg) => write!(f, "Invalid bot token: {}", msg),
            AppError::InvalidChatId(msg) => write!(f, "Invalid chat ID: {}", msg),
            AppError::TelegramError(msg) => write!(f, "Telegram error: {}", msg),
            AppError::BatteryError(msg) => write!(f, "Battery error: {}", msg),
        }
    }
}
