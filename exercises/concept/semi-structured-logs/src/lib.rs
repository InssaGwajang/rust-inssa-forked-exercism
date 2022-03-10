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
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}

// use std::fmt;
// impl fmt::Display for LogLevel {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 LogLevel::Info => "INFO",
//                 LogLevel::Warning => "WARNING",
//                 LogLevel::Error => "ERROR",
//             }
//         )
//     }
// }
// pub fn log(level: LogLevel, message: &str) -> String {
//     format!("[{}]: {}", level, message)
// }
