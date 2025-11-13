use solana_pump_grpc_sdk::{GrpcClient, Config, EventFilter, FilteredLoggingEventHandler};
use rustls::crypto::ring;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置日志级别，如果环境变量 RUST_LOG 未设置，则默认使用 info
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    ring::default_provider()
        .install_default()
        .expect("failed to install Rustls crypto provider");

    // Yellowstone gRPC server URL
    let url = "https://solana-yellowstone-grpc.publicnode.com".to_string();
    
    let client = GrpcClient::new(Config::new(url));

    // 自定义过滤器（精确控制）
    let filter = EventFilter {
        create: true,      // Pump CreateEvent
        create_v2: true,   // Pump CreateV2Event
        complete: true,    // Pump CompleteEvent
        trade: true,        // Pump TradeEvent
        buy: true,          // PumpAmm BuyEvent 
        sell: true,        // PumpAmm SellEvent
        create_pool: true, // PumpAmm CreatePoolEvent
    };

    let handler = FilteredLoggingEventHandler::new(filter);

    // 订阅多个程序
    let program_ids = vec![
        "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",      // Pump
        "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA",     // PumpAmm
    ];

    for program_id in program_ids {
        let client = client.clone();
        let handler = handler.clone();
        tokio::spawn(async move {
            if let Err(e) = client.subscribe(program_id.to_string(), handler).await {
                log::error!("订阅程序 {} 失败: {:?}", program_id, e);
            }
        });
    }

    tokio::signal::ctrl_c().await?;
    Ok(())
}
