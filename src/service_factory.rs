use crate::{laptop_guarder::LaptopGuarder, telegram_bot::TelegramBot};

pub struct ServiceFactory;

impl ServiceFactory {
    pub fn create_bot(token: &str) -> TelegramBot {
        TelegramBot::new(token.to_string())
    }

    pub fn create_guard(bot: TelegramBot, chat_id: i64, interval: u64) -> LaptopGuarder {
        LaptopGuarder::new(bot, chat_id, interval)
    }
}
