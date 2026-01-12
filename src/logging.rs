use colored::*;
use std::fmt::Display;

/// Log levels supported by the logging system
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
//    Debug = 0,
    Info = 1,
    Success = 2,
    Warning = 3,
    Error = 4,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            //LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Success => write!(f, "SUCCESS"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// Core logging function - all other log functions use this
pub fn log(level: LogLevel, message: &str) {
    let formatted_message = match level {
//        LogLevel::Debug => format!("gorilla: ({}) {}", "dbg".dimmed(), message.dimmed()),
        LogLevel::Info => format!("gorilla: ({}) {}", "inf".cyan(), message),
        LogLevel::Success => format!("gorilla: ({}) {}", "win".green(), message.green()),
        LogLevel::Warning => format!("gorilla: ({}) {}", "wrn".yellow(), message.yellow()),
        LogLevel::Error => format!("gorilla: ({}) {}", "err".red().bold(), message.red()),
    };

    eprintln!("{}", formatted_message);
}

/// Log a debug message (lowest priority, typically for development)
/// These are hidden by default unless debug mode is enabled
//pub fn debug(message: &str) {
//    log(LogLevel::Debug, message);
//}

/// Log an informational message (normal operational messages)
pub fn info(message: &str) {
    log(LogLevel::Info, message);
}


/// Log a success message (operations completed successfully)
pub fn success(message: &str) {
    log(LogLevel::Success, message);
}


/// Log a warning message (something unexpected but not critical)
pub fn warning(message: &str) {
    log(LogLevel::Warning, message);
}
/// Log an error message (critical issues that need attention)
pub fn error(message: &str) {
    log(LogLevel::Error, message);
}


// Macros for easier formatted logging (similar to println! but for logging)

/// Log formatted debug message
/// Usage: `log_debug!("Value: {}", value);`
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::logging::debug_fmt(format_args!($($arg)*))
    };
}

/// Log formatted info message
/// Usage: `log_info!("Processing {} items", count);`
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::logging::info_fmt(format_args!($($arg)*))
    };
}

/// Log formatted success message
/// Usage: `log_success!("Generated {} words", count);`
#[macro_export]
macro_rules! log_success {
    ($($arg:tt)*) => {
        $crate::logging::success_fmt(format_args!($($arg)*))
    };
}

/// Log formatted warning message
/// Usage: `log_warning!("Unexpected value: {}", val);`
#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {
        $crate::logging::warning_fmt(format_args!($($arg)*))
    };
}

/// Log formatted error message
/// Usage: `log_error!("Failed to process: {}", error);`
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::logging::error_fmt(format_args!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_levels() {
//        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warning);
        assert!(LogLevel::Warning < LogLevel::Error);
    }

    #[test]
    fn test_log_level_display() {
     //   assert_eq!(LogLevel::Debug.to_string(), "DEBUG");
        assert_eq!(LogLevel::Info.to_string(), "INFO");
        assert_eq!(LogLevel::Success.to_string(), "SUCCESS");
        assert_eq!(LogLevel::Warning.to_string(), "WARNING");
        assert_eq!(LogLevel::Error.to_string(), "ERROR");
    }

}
