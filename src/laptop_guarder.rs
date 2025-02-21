use std::thread::sleep;
use std::time::Duration;

use crate::logger::{info, warn};
use crate::power_checker::is_plugged;
use crate::telegram_notifier::TelegramNotifier;

pub struct LaptopGuarder {
    telegram_notifier: TelegramNotifier,
    interval: u64,
}

impl LaptopGuarder {
    pub fn new(telegram_notifier: TelegramNotifier, interval: u64) -> Self {
        Self {
            telegram_notifier,
            interval,
        }
    }

    pub fn start(&self) {
        info("Monitoring laptop power state...");
        let mut was_plugged = is_plugged();

        loop {
            sleep(Duration::from_secs(self.interval));

            let is_plugged = is_plugged();

            if was_plugged && !is_plugged {
                warn("Laptop has been unplugged! Sending alert...");
                self.telegram_notifier
                    .send_message("Laptop Unplugged! Possible theft alert!");
            }

            was_plugged = is_plugged;
        }
    }
}
