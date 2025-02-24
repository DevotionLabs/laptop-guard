use crate::{
    laptop_guarder::LaptopGuarder, logger::info, telegram_bot::TelegramBot,
    telegram_notifier::TelegramNotifier,
};

pub struct ServiceFactory;

impl ServiceFactory {
    pub fn create_bot(token: &str) -> TelegramBot {
        TelegramBot::new(token.to_string())
    }

    pub fn create_guard(token: &str, chat_id: &str, interval: u64) -> LaptopGuarder {
        let notifier = TelegramNotifier::new(token.to_string(), chat_id.to_string());
        LaptopGuarder::new(notifier, interval)
    }
}
