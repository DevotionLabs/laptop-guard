use crate::{
    app_config::AppConfig, laptop_guarder::LaptopGuarder, service_factory::ServiceFactory,
    task_manager::TaskManager, telegram_bot::TelegramBot, validator::validate_tg_bot_token,
};
use tokio::task::spawn;

pub struct App {
    config: AppConfig,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let (bot, guard) = self.init_services().await?;

        let bot_task = spawn(async move { bot.run().await });

        let guard_task = guard.map(|g| spawn(async move { g.run().await }));

        let mut task_manager = TaskManager::new(bot_task, guard_task);
        task_manager.run().await;

        Ok(())
    }

    async fn init_services(
        &self,
    ) -> Result<(TelegramBot, Option<LaptopGuarder>), Box<dyn std::error::Error>> {
        validate_tg_bot_token(&self.config.bot_token)?;

        let bot = ServiceFactory::create_bot(&self.config.bot_token);

        let guard = self.config.chat_id.map(|chat_id| {
            ServiceFactory::create_guard(bot.clone(), chat_id, self.config.interval)
        });

        Ok((bot, guard))
    }
}
