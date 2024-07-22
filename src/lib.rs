mod log_state;
mod log_viewer;

pub use log_state::LogLevel;
pub use log_viewer::{Logger, register_logger, register_logger_with_custom_name, error, info, warn, send, register_custom_logger, register_custom_logger_simple};