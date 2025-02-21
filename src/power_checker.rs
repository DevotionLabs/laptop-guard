use battery::{Manager, State};

use crate::logger::info;

/// Checks if the laptop is plugged into power
pub fn is_plugged() -> bool {
    let manager = Manager::new().expect("Failed to initialize battery manager");

    if let Ok(batteries) = manager.batteries() {
        for battery in batteries.flatten() {
            info(&battery.state().to_string());

            return battery.state() == State::Charging;
        }
    }
    false
}
