mod cli;
mod laptop_guarder;
mod logger;
mod power_checker;
mod telegram_notifier;
mod validator;

use clap::Parser;
use cli::Cli;
use laptop_guarder::LaptopGuarder;
use logger::{fatal, info};
use telegram_notifier::TelegramNotifier;
use validator::validate_telegram_inputs;

fn main() {
    let args = Cli::parse();
    let bot_token = args.bot_token;
    let chat_id = args.chat_id;

    match validate_telegram_inputs(&bot_token, &chat_id) {
        Ok(()) => info("Starting laptop guard process..."),
        Err(e) => {
            fatal(&e);
            std::process::exit(1);
        }
    }

    let tg_notifier = TelegramNotifier::new(bot_token, chat_id);
    let guarder = LaptopGuarder::new(tg_notifier, args.interval);

    guarder.start();
}
