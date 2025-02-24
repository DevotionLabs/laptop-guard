use crate::{
    app_config::AppConfig, logger::info, service_factory::ServiceFactory,
    task_manager::TaskManager, validator::validate_telegram_inputs,
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
        validate_telegram_inputs(&self.config.bot_token, self.config.chat_id.as_deref())?;

        let (bot_task, guard_task) = self.initialize_services();

        let mut task_manager = TaskManager::new(bot_task, guard_task);
        task_manager.run().await;

        Ok(())
    }

    fn initialize_services(&self) -> (task::JoinHandle<()>, Option<task::JoinHandle<()>>) {
        let bot = ServiceFactory::create_bot(&self.config.bot_token);
        let bot_task = task::spawn(async move { bot.run().await });

        let guard_task = self.config.chat_id.as_ref().map(|chat_id| {
            info("Starting laptop guard process in full mode...");
            let guard =
                ServiceFactory::create_guard(&self.config.bot_token, chat_id, self.config.interval);
            task::spawn_blocking(move || guard.start())
        });

        if guard_task.is_none() {
            info("Starting laptop guard in bot-only mode...");
            info("Use /chatid command to get your chat ID");
        }

        (bot_task, guard_task)
    }
}
