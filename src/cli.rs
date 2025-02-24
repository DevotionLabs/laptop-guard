use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(
        short = 't',
        long = "token",
        help = "Sets the Telegram Bot API token",
        required = true
    )]
    pub bot_token: String,

    #[clap(
        short = 'c',
        long = "chat",
        help = "Sets the Telegram chat ID",
        required = false
    )]
    pub chat_id: Option<String>,

    #[clap(
        short = 'i',
        long = "interval",
        help = "Sets the interval between checks in seconds",
        default_value = "20",
        required = false
    )]
    pub interval: u64,
}
