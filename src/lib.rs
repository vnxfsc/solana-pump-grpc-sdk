pub mod client;
pub mod error;
pub mod models;
pub mod parser;

// 重新导出公共API
pub use client::{
    Config, EventContext, EventFilter, EventHandler, FilteredLoggingEventHandler, GrpcClient,
    LoggingEventHandler,
};
pub use error::{Error, Result};
pub use models::*;

/// SDK版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
