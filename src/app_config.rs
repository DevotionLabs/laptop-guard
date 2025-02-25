use crate::cli::Cli;

pub struct AppConfig {
    pub bot_token: String,
    pub chat_id: Option<i64>,
    pub interval: u64,
}

impl From<Cli> for AppConfig {
    fn from(cli: Cli) -> Self {
        Self {
            bot_token: cli.bot_token,
            chat_id: cli.chat_id,
            interval: cli.interval,
        }
    }
}
