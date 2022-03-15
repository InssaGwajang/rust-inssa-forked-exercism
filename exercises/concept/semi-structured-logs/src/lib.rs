use self::LogLevel::*;

#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub fn log(level: LogLevel, message: &str) -> String {
    format!("[{}]: {}", format!("{:?}", level).to_uppercase(), message)
}

pub fn info(message: &str) -> String {
    log(Info, message)
}

pub fn warn(message: &str) -> String {
    log(Warning, message)
}

pub fn error(message: &str) -> String {
    log(Error, message)
}
