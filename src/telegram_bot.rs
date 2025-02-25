use crate::logger::{error, info};
use teloxide::{prelude::*, repl, types::ParseMode};
use tokio::runtime::Handle;

#[derive(Clone)]
pub struct TelegramBot {
    bot: Bot,
}

impl TelegramBot {
    pub fn new(token: String) -> Self {
        Self {
            bot: Bot::new(&token),
        }
    }

    pub async fn run(&self) {
        repl(self.bot.clone(), |bot: Bot, msg: Message| async move {
            Self::handle_message(bot, msg).await;
            Ok(())
        })
        .await;

        info("🛑 Telegram bot shutting down...");
    }

    pub fn send_alert(&self, chat_id: i64) {
        let alert_msg = "🚨 Laptop unplugged\n\n\
        Possible theft alert";

        Handle::current().block_on(Self::send_markdown_message(
            &self.bot,
            ChatId(chat_id),
            alert_msg,
        ));
    }

    async fn handle_message(bot: Bot, msg: Message) {
        match msg.text() {
            Some("/chatid") => Self::handle_chatid(&bot, msg).await,
            _ => Self::handle_default(&bot, msg).await,
        }
    }

    async fn handle_chatid(bot: &Bot, msg: Message) {
        let chat_id = msg.chat.id;

        let reply_msg = format!("Your chat ID is: `{}`", chat_id);

        Self::send_markdown_message(bot, chat_id, &reply_msg).await;
    }

    async fn handle_default(bot: &Bot, msg: Message) {
        let helper_text = "🔔 Laptop Guard Alert System\n\n\
        This bot alerts you in this chat if your laptop is unplugged while Laptop Guard is running\n\n\
        📌 Available commands:\n\
        `/chatid`: Retrieve your chat ID\n\n\
        ℹ️ No other commands are available\n\
        Just wait for alerts if your laptop is unplugged";

        Self::send_markdown_message(bot, msg.chat.id, helper_text).await;
    }

    async fn send_markdown_message(bot: &Bot, chat_id: ChatId, msg: &str) {
        if let Err(e) = bot
            .send_message(chat_id, msg)
            .parse_mode(ParseMode::MarkdownV2)
            .await
        {
            error(&format!("Failed to send Telegram message: {}", e));
        }
    }
}
