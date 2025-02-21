use crate::logger::info;
use teloxide::prelude::*;
use tokio::signal;

pub struct TelegramBot {
    bot: Bot,
}

impl TelegramBot {
    pub fn new(token: String) -> Self {
        Self {
            bot: Bot::new(token),
        }
    }

    pub async fn run(&self) {
        info("🤖 Telegram bot is online!");

        signal::ctrl_c()
            .await
            .expect("Failed to listen for shutdown signal");

        info("🛑 Telegram bot shutting down...");
    }
}
