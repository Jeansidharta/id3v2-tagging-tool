use colored::Colorize;
use std::env;

#[derive(Clone)]
pub enum LogLevels {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

const DEFAULT_LOG_LEVEL: LogLevels = LogLevels::Info;

static mut LOG_LEVEL: LogLevels = DEFAULT_LOG_LEVEL;

fn get_level_from_env() -> LogLevels {
    let env_string_level = env::var("RUST_LOG")
        .unwrap_or_else(|_| "INFO".to_string())
        .to_lowercase();

    match &env_string_level[..] {
        "info" => LogLevels::Info,
        "trace" => LogLevels::Trace,
        "debug" => LogLevels::Debug,
        "warn" | "warning" => LogLevels::Warn,
        "err" | "error" => LogLevels::Error,
        _ => DEFAULT_LOG_LEVEL,
    }
}

pub fn init(initial_level: Option<LogLevels>) {
    unsafe {
        LOG_LEVEL = initial_level.unwrap_or_else(get_level_from_env);
    }
}

pub fn get_log_level() -> LogLevels {
    unsafe { LOG_LEVEL.clone() }
}

fn log(log_level: LogLevels, message: String) {
    let current_log_level = get_log_level() as i32;
    if log_level as i32 >= current_log_level {
        println!("{}", message);
    }
}

pub fn warn(message: String) {
    log(LogLevels::Warn, format!("{}  {}", "⚠".yellow(), message))
}
pub fn error(message: String) {
    log(LogLevels::Error, format!("{}  {}", "❌".red(), message))
}
