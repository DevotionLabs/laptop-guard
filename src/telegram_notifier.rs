use ureq::get;

use crate::logger::{error, info};

pub struct TelegramNotifier {
    bot_token: String,
    chat_id: String,
}

impl TelegramNotifier {
    pub fn new(bot_token: String, chat_id: String) -> Self {
        Self { bot_token, chat_id }
    }

    pub fn send_message(&self, message: &str) {
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
            self.bot_token, self.chat_id, message
        );

        if let Err(e) = get(&url).call() {
            error(&format!("Failed to send TG alert: {}", e));
        } else {
            info("Sent TG alert successfully");
        }
    }
}
