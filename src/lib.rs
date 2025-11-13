pub mod client;
pub mod error;
pub mod models;
pub mod parser;
pub mod trading;

// 重新导出公共API
pub use client::{
    Config, EventContext, EventFilter, EventHandler, FilteredLoggingEventHandler, GrpcClient,
    LoggingEventHandler,
};
pub use error::{Error, Result};
pub use models::*;
pub use trading::{
    TradeClient, OptionBool, pump_amm_program_id, pump_program_id,
    derive_pump_amm_pool_pda, derive_pump_amm_global_config_pda,
    wsol_mint, WSOL_MINT,
};

/// SDK版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
