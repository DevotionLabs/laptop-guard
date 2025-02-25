use crate::{
    app_config::AppConfig, logger::info, service_factory::ServiceFactory,
    task_manager::TaskManager, validator::validate_tg_bot_token,
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

        let (bot_task, guard_task) = self.initialize_services();

        let mut task_manager = TaskManager::new(bot_task, guard_task);
        task_manager.run().await;

        Ok(())
    }

    fn initialize_services(&self) -> (task::JoinHandle<()>, Option<task::JoinHandle<()>>) {
        let bot = ServiceFactory::create_bot(&self.config.bot_token);
        let bot_task = {
            let bot_clone = bot.clone();
            task::spawn(async move { bot_clone.run().await })
        };

        if let Some(chat_id) = self.config.chat_id {
            info("Starting laptop guard process in full mode...");
            info(&format!("Chat ID: {}", chat_id));

            let guard = ServiceFactory::create_guard(bot, chat_id, self.config.interval);
            let guard_task = task::spawn_blocking(move || guard.start());
            (bot_task, Some(guard_task))
        } else {
            info("Starting laptop guard in bot-only mode...");
            info("Use /chatid command to get your chat ID");
            (bot_task, None)
        }
    }
}
