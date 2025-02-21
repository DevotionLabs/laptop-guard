use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use crate::logger::{info, warn};
use crate::power_checker::is_plugged;
use crate::telegram_notifier::TelegramNotifier;

pub struct LaptopGuarder {
    telegram_notifier: TelegramNotifier,
    interval: u64,
    shutdown_flag: Arc<AtomicBool>,
}

impl LaptopGuarder {
    pub fn new(
        telegram_notifier: TelegramNotifier,
        interval: u64,
        shutdown_flag: Arc<AtomicBool>,
    ) -> Self {
        Self {
            telegram_notifier,
            interval,
            shutdown_flag,
        }
    }

    pub fn start(&self) {
        info("Monitoring laptop power state...");
        let mut was_plugged = is_plugged();

        while !self.shutdown_flag.load(Ordering::SeqCst) {
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
