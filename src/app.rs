use crate::{
    app_config::AppConfig, logger::info, service_factory::ServiceFactory,
    task_manager::TaskManager, telegram_bot::TelegramBot, validator::validate_tg_bot_token,
};
use tokio::task;

pub struct App {
    config: AppConfig,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        validate_tg_bot_token(&self.config.bot_token)?;

        let bot = ServiceFactory::create_bot(&self.config.bot_token);

        let bot_task = self.initialize_bot(bot.clone());
        let guard_task = self.initialize_guard(bot);

        let mut task_manager = TaskManager::new(bot_task, guard_task);
        task_manager.run().await;

        Ok(())
    }

    fn initialize_bot(&self, bot: TelegramBot) -> task::JoinHandle<()> {
        info("Starting telegram bot...");
        info("Use /chatid command to get your chat ID");

        let bot_clone = bot.clone();
        task::spawn(async move { bot_clone.run().await })
    }

    fn initialize_guard(&self, bot: TelegramBot) -> Option<task::JoinHandle<()>> {
        let chat_id = match self.config.chat_id {
            Some(id) => id,
            None => return None,
        };

        info("Starting laptop guard process in full mode...");
        info(&format!("Chat ID: {}", chat_id));

        let guard = ServiceFactory::create_guard(bot, chat_id, self.config.interval);
        let guard_task = task::spawn_blocking(move || guard.start());

        Some(guard_task)
    }
}
