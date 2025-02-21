use battery::{Manager, State};

pub fn is_plugged() -> bool {
    let manager = Manager::new().expect("Failed to initialize battery manager");

    if let Ok(batteries) = manager.batteries() {
        for battery in batteries.flatten() {
            return battery.state() == State::Charging;
        }
    }
    false
}
