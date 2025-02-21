mod cli;
mod laptop_guarder;
mod logger;
mod power_checker;
mod telegram_bot;
mod telegram_notifier;
mod validator;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use clap::Parser;
use cli::Cli;
use laptop_guarder::LaptopGuarder;
use logger::{fatal, info};
use telegram_bot::TelegramBot;
use telegram_notifier::TelegramNotifier;
use tokio::{signal, task};
use validator::validate_telegram_inputs;

#[tokio::main]
async fn main() {
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

    let shutdown_flag = Arc::new(AtomicBool::new(false));

    let tg_bot = TelegramBot::new(bot_token.clone());
    let tg_notifier = TelegramNotifier::new(bot_token.clone(), chat_id);

    let guarder = LaptopGuarder::new(tg_notifier, args.interval, shutdown_flag.clone());

    let bot_task = task::spawn(async move {
        tg_bot.run().await;
    });

    let guarder_task = task::spawn_blocking(move || {
        guarder.start();
    });

    let shutdown_task = task::spawn(async move {
        signal::ctrl_c().await.unwrap();
        info("Shutting down...");
        shutdown_flag.store(true, Ordering::SeqCst);
    });

    let _ = tokio::join!(bot_task, guarder_task, shutdown_task);
}
