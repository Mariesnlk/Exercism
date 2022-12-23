// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    // unimplemented!("return a message for the given log level")
    let level = format!("{:?}", level);
    format!("[{}]: {}", level.to_uppercase(), message)
}
pub fn info(message: &str) -> String {
    // unimplemented!("return a message for info log level")
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    // unimplemented!("return a message for warn log level")
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    // unimplemented!("return a message for error log level")
    log(LogLevel::Error, message)
}
