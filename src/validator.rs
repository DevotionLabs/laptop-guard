use crate::error::AppError;
use regex::Regex;

fn is_tg_bot_token_valid(token: &str) -> bool {
    let token_pattern = Regex::new(r"^[0-9]{8,10}:[a-zA-Z0-9_-]{35}$").unwrap();
    token_pattern.is_match(token)
}

fn is_tg_chat_id_valid(chat_id: &str) -> bool {
    let chat_id_pattern = Regex::new(r"^-?[0-9]{9,}$").unwrap();
    chat_id_pattern.is_match(chat_id)
}

pub fn validate_telegram_inputs(bot_token: &str, chat_id: Option<&str>) -> Result<(), AppError> {
    if !is_tg_bot_token_valid(bot_token) {
        return Err(AppError::InvalidBotToken(
            "It must follow the format '123456789:ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz0123456789'".to_string(),
        ));
    }

    if let Some(chat_id) = chat_id {
        if !is_tg_chat_id_valid(chat_id) {
            return Err(AppError::InvalidChatId(
                "It must be a numeric string with at least 9 digits (may be negative for groups)"
                    .to_string(),
            ));
        }
    }

    Ok(())
}
