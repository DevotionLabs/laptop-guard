mod app;
mod app_config;
mod app_error;
mod cli;
mod laptop_guarder;
mod logger;
mod power_checker;
mod service_factory;
mod task_manager;
mod telegram_bot;
mod validator;

use std::process::exit;

use app::App;
use app_config::AppConfig;
use clap::Parser;
use cli::Cli;
use logger::fatal;

#[tokio::main]
async fn main() {
    let config = AppConfig::from(Cli::parse());
    let app = App::new(config);

    if let Err(e) = app.run().await {
        fatal(&e.to_string());
        exit(1);
    }
}
