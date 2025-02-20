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
        required = true
    )]
    pub chat_id: String,

    #[clap(
        short = 'i',
        long = "interval",
        help = "Sets the interval between checks in seconds",
        default_value = "20"
    )]
    pub interval: u64,
}
