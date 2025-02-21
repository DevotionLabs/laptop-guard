use battery::{Manager, State};

pub fn is_plugged() -> bool {
    let manager = Manager::new().expect("Failed to initialize battery manager");

    if let Ok(batteries) = manager.batteries() {
        if let Some(battery) = batteries.flatten().next() {
            return battery.state() == State::Charging;
        }
    }

    false
}
