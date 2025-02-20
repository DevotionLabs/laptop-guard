use battery::{Manager, State};

pub struct PowerChecker {
    manager: Manager,
}

impl PowerChecker {
    /// Creates a new PowerChecker instance
    pub fn new() -> Self {
        let manager = Manager::new().expect("Failed to initialize battery manager");
        Self { manager }
    }

    /// Checks if the laptop is plugged into power
    pub fn is_plugged(&self) -> bool {
        if let Ok(batteries) = self.manager.batteries() {
            for battery in batteries.flatten() {
                return battery.state() == State::Charging;
            }
        }
        false
    }
}
