use tokio::time::{sleep, Duration};

use crate::logger::{info, warn};
use crate::power_checker::is_plugged;
use crate::telegram_bot::TelegramBot;

pub struct LaptopGuarder {
    tg_bot: TelegramBot,
    chat_id: i64,
    interval: u64,
}

impl LaptopGuarder {
    pub fn new(tg_bot: TelegramBot, chat_id: i64, interval: u64) -> Self {
        Self {
            tg_bot,
            chat_id,
            interval,
        }
    }

    pub async fn run(&self) {
        info("Starting laptop power state monitoring process...");
        info(&format!("Chat ID: {}", self.chat_id));

        let mut was_plugged = is_plugged();

        loop {
            sleep(Duration::from_secs(self.interval)).await;

            let is_plugged = is_plugged();

            if was_plugged && !is_plugged {
                warn("Laptop has been unplugged! Sending alert...");
                self.tg_bot.send_alert(self.chat_id).await;
            }

            was_plugged = is_plugged;
        }
    }
}
