use crate::app_error::AppError;
use regex::Regex;

fn is_tg_bot_token_valid(token: &str) -> bool {
    let token_pattern = Regex::new(r"^[0-9]{8,10}:[a-zA-Z0-9_-]{35}$").unwrap();
    token_pattern.is_match(token)
}

pub fn validate_tg_bot_token(bot_token: &str) -> Result<(), AppError> {
    if !is_tg_bot_token_valid(bot_token) {
        return Err(AppError::InvalidBotToken(
            "Provided Telegram bot API token format is invalid".to_string(),
        ));
    }

    Ok(())
}
