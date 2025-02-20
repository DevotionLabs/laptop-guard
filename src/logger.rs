use chrono::Local;
use colored::*;

pub fn info(message: &str) {
    log("INFO", message);
}

pub fn warn(message: &str) {
    log("WARN", message);
}

pub fn error(message: &str) {
    log("ERROR", message);
}

pub fn fatal(message: &str) {
    log("FATAL", message);
}

fn log(level: &str, message: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let colored_level = match level {
        "FATAL" => level.bright_red(),
        "ERROR" => level.red(),
        "WARN" => level.yellow(),
        "INFO" => level.green(),
        _ => level.normal(),
    };
    println!("{} [{}] {}", timestamp, colored_level, message);
}
