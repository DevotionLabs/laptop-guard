use crate::logger::info;
use tokio::{select, signal, task::JoinHandle};

pub struct TaskManager {
    bot_task: JoinHandle<()>,
    guard_task: Option<JoinHandle<()>>,
}

impl TaskManager {
    pub fn new(bot_task: JoinHandle<()>, guard_task: Option<JoinHandle<()>>) -> Self {
        Self {
            bot_task,
            guard_task,
        }
    }

    pub async fn run(&mut self) {
        if let Some(guard_task) = self.guard_task.take() {
            self.run_full_mode(guard_task).await;
        } else {
            self.run_bot_only_mode().await;
        }

        info("All tasks stopped.");
    }

    async fn run_full_mode(&mut self, guard_task: JoinHandle<()>) {
        select! {
            _ = &mut self.bot_task => info("Telegram bot task finished."),
            _ = guard_task =>  info("Laptop Guard task finished."),
            _ = signal::ctrl_c() => info("🛑 CTRL+C detected. Stopping all tasks..."),
        }
    }

    async fn run_bot_only_mode(&mut self) {
        select! {
            _ = signal::ctrl_c() => info("🛑 CTRL+C detected. Stopping all tasks..."),
            _ = &mut self.bot_task => info("Telegram bot task finished.")
        }
    }
}
