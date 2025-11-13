# Solana Pump gRPC SDK

[![Crates.io](https://img.shields.io/crates/v/solana-pump-grpc-sdk.svg)](https://crates.io/crates/solana-pump-grpc-sdk)
[![Documentation](https://docs.rs/solana-pump-grpc-sdk/badge.svg)](https://docs.rs/solana-pump-grpc-sdk)
[![License](https://img.shields.io/crates/l/solana-pump-grpc-sdk.svg)](https://github.com/vnxfsc/solana-pump-grpc-sdk/blob/main/LICENSE)

这是一个用于监听 Solana 链上 Pump 和 PumpAmm 程序事件的 高性能 Rust SDK。基于 `yellowstone-grpc`，提供简单易用的 API 来处理各种事件。

## 特性

- ⚡ **高性能**：基于 Tokio 异步运行时，支持高并发事件处理，单连接可处理数千 TPS
- 🚀 **简单易用**：基于 trait 的事件处理器，只需实现感兴趣的事件处理方法
- 🔌 **灵活配置**：支持自定义连接超时、请求超时、承诺级别等配置
- 📦 **类型安全**：完整的事件类型定义，编译时类型检查
- 🎯 **多程序支持**：可同时订阅多个程序的事件，每个订阅独立异步处理
- 🛡️ **错误处理**：完善的错误处理机制，提供详细的错误信息
- 📝 **事件支持**：完整支持 Pump 和 PumpAmm 程序的所有事件类型
- 🎨 **事件过滤**：内置事件过滤器，可选择性地处理特定类型的事件，减少不必要开销
- ⏱️ **性能监控**：自动记录事件处理耗时，便于性能优化
- 🔄 **连接复用**：支持 keep-alive 和连接池，减少连接建立开销

## 支持的事件

### Pump 程序 `6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P`
- `CreateEvent / CreateV2Event`：创建代币（含 `is_mayhem_mode` 标记）
- `CompleteEvent`：曲线完成
- `TradeEvent`：买卖撮合

### PumpAmm 程序 `pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA`
- `BuyEvent`：买入
- `SellEvent`：卖出
- `CreatePoolEvent`：创建池

## 安装

### 从 crates.io 安装（推荐）

在你的 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
solana-pump-grpc-sdk = "0.1.0"
tokio = { version = "1.48", features = ["full"] }
log = "0.4"
pretty_env_logger = "0.5"
```

### 从 GitHub 安装

```toml
[dependencies]
solana-pump-grpc-sdk = { git = "https://github.com/vnxfsc/solana-pump-grpc-sdk" }
```

### 本地开发

```toml
[dependencies]
solana-pump-grpc-sdk = { path = "../solana-pump-grpc-sdk" }
```

然后运行：
```bash
cargo build
```

## 性能特点

- **异步非阻塞**：基于 Tokio 异步运行时，所有 I/O 操作都是非阻塞的，单线程可处理大量并发事件
- **零拷贝解析**：使用高效的 Borsh 反序列化，最小化内存分配和拷贝
- **批量处理**：单次订阅可接收多个事件，自动批量解析和处理
- **低延迟**：直接监听 gRPC 流式数据，实时处理链上事件，延迟通常在毫秒级
- **高吞吐量**：支持单连接处理数千 TPS（每秒交易数），多连接可线性扩展
- **资源高效**：使用连接复用和 keep-alive，最小化网络开销和连接建立时间

## 快速开始

### 基本使用（使用内置日志处理器）

```rust
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

    // 配置要打印的事件类型
    let filter = EventFilter {
        create: false,      // Pump CreateEvent
        create_v2: false,   // Pump CreateV2Event
        complete: false,    // Pump CompleteEvent
        trade: true,        // Pump TradeEvent 
        buy: true,          // PumpAmm BuyEvent 
        sell: false,        // PumpAmm SellEvent
        create_pool: false, // PumpAmm CreatePoolEvent
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
```

### 自定义事件处理器

如果需要自定义事件处理逻辑，可以实现 `EventHandler` trait：

```rust
use solana_pump_grpc_sdk::{EventHandler, EventContext, GrpcClient, Config};
use solana_pump_grpc_sdk::models::*;
use rustls::crypto::ring;

struct MyEventHandler;

impl EventHandler for MyEventHandler {
    fn on_create_event(&self, event: &CreateEvent, ctx: &EventContext) {
        println!("创建事件: {:?}", event);
        println!("槽位: {}, 签名: {}", ctx.slot, ctx.signature);
    }

    fn on_trade_event(&self, event: &TradeEvent, ctx: &EventContext) {
        println!("交易事件: {:?}", event);
        println!("耗时: {:?}", ctx.elapsed);
    }

    // 实现其他感兴趣的事件处理方法...
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    
    ring::default_provider()
        .install_default()
        .expect("failed to install Rustls crypto provider");
    
    let url = "https://solana-yellowstone-grpc.publicnode.com".to_string();
    let client = GrpcClient::new(Config::new(url));
    let handler = MyEventHandler;
    
    client.subscribe(
        "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P".to_string(),
        handler
    ).await?;
    
    Ok(())
}
```

### 高级配置

```rust
use solana_pump_grpc_sdk::{Config, GrpcClient};
use std::time::Duration;
use yellowstone_grpc_proto::geyser::CommitmentLevel;

// 自定义配置
let config = Config::new("https://solana-yellowstone-grpc.publicnode.com".to_string())
    .with_connect_timeout(Duration::from_secs(30))
    .with_timeout(Duration::from_secs(120))
    .with_keep_alive(true)
    .with_commitment(CommitmentLevel::Confirmed);

let client = GrpcClient::new(config);
```


## API 文档

### `Config`

客户端配置结构体。

```rust
pub struct Config {
    pub url: String,
    pub connect_timeout: Duration,
    pub timeout: Duration,
    pub keep_alive_while_idle: bool,
    pub commitment: CommitmentLevel,
}
```

**方法：**
- `new(url: String) -> Self`：创建新配置
- `with_connect_timeout(timeout: Duration) -> Self`：设置连接超时
- `with_timeout(timeout: Duration) -> Self`：设置请求超时
- `with_keep_alive(keep_alive: bool) -> Self`：设置是否保持连接
- `with_commitment(commitment: CommitmentLevel) -> Self`：设置承诺级别

### `GrpcClient`

gRPC 客户端。

```rust
impl GrpcClient {
    pub fn new(config: Config) -> Self;
    pub async fn subscribe<H: EventHandler>(
        &self,
        program_id: String,
        handler: H,
    ) -> Result<()>;
}
```

### `EventHandler`

事件处理器 trait。所有方法都有默认的空实现，只需实现感兴趣的事件处理方法。

```rust
pub trait EventHandler: Send + Sync {
    fn on_create_event(&self, event: &CreateEvent, ctx: &EventContext);
    fn on_create_v2_event(&self, event: &CreateV2Event, ctx: &EventContext);
    fn on_complete_event(&self, event: &CompleteEvent, ctx: &EventContext);
    fn on_trade_event(&self, event: &TradeEvent, ctx: &EventContext);
    fn on_buy_event(&self, event: &BuyEvent, ctx: &EventContext);
    fn on_sell_event(&self, event: &SellEvent, ctx: &EventContext);
    fn on_create_pool_event(&self, event: &CreatePoolEvent, ctx: &EventContext);
}
```

### `LoggingEventHandler`

内置的日志事件处理器，自动将所有事件记录到日志中。

```rust
pub struct LoggingEventHandler;
```

**使用示例：**
```rust
let handler = LoggingEventHandler;
client.subscribe(program_id, handler).await?;
```

### `EventFilter` 和 `FilteredLoggingEventHandler`

事件过滤器，用于选择性地打印特定类型的事件。

```rust
pub struct EventFilter {
    pub create: bool,       // CreateEvent
    pub create_v2: bool,    // CreateV2Event
    pub complete: bool,     // CompleteEvent
    pub trade: bool,        // TradeEvent
    pub buy: bool,          // BuyEvent
    pub sell: bool,         // SellEvent
    pub create_pool: bool,  // CreatePoolEvent
}
```

**方法：**
- `all() -> Self`：启用所有事件（默认）
- `none() -> Self`：禁用所有事件
- `pump_only() -> Self`：只启用 Pump 相关事件（Create, CreateV2, Complete, Trade）
- `pumpamm_only() -> Self`：只启用 PumpAmm 相关事件（Buy, Sell, CreatePool）

**使用示例：**
```rust
// 使用预定义过滤器
let filter = EventFilter::pump_only();
let handler = FilteredLoggingEventHandler::new(filter);

// 或自定义过滤器
let filter = EventFilter {
    trade: true,
    buy: true,
    ..EventFilter::none()
};
let handler = FilteredLoggingEventHandler::new(filter);
```

### `EventContext`

事件上下文，包含事件发生的上下文信息。

```rust
pub struct EventContext {
    pub slot: u64,              // 区块槽位
    pub tx_index: u64,          // 交易索引
    pub signature: Signature,   // 交易签名
    pub timestamp: Instant,     // 事件处理开始时间戳
    pub elapsed: Duration,      // 从开始处理到当前事件的耗时
}
```


## 运行示例

项目包含一个基本使用示例：

```bash
# 运行示例（无需设置环境变量）
cargo run --example basic

# 或者设置日志级别
RUST_LOG=debug cargo run --example basic
```

## 项目结构

```
.
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs              # 库入口
│   ├── client/             # gRPC 客户端
│   │   ├── mod.rs
│   │   ├── config.rs       # 配置结构
│   │   ├── handler.rs      # 事件处理器 trait
│   │   └── grpc.rs         # gRPC 客户端实现
│   ├── models/             # 事件模型
│   │   └── mod.rs
│   ├── parser/             # 事件解析器
│   │   ├── mod.rs
│   │   └── events.rs       # EventTrait 和 discriminator 常量定义
│   └── error.rs            # 错误类型
└── examples/
    └── basic.rs            # 基本使用示例
```

## 错误处理

SDK 使用 `Result<T, Error>` 类型进行错误处理。错误类型包括：

- `GrpcClient`：gRPC 客户端错误
- `GrpcBuilder`：gRPC 客户端构建错误
- `GrpcConnection`：gRPC 连接错误
- `TlsConfig`：TLS 配置错误
- `SubscribeError`：订阅错误
- `ParseError`：事件解析错误
- `BorshDeserialize`：Borsh 反序列化错误
- `SignatureParse`：签名解析错误

## 依赖

- `tokio`：异步运行时
- `yellowstone-grpc-client`：Yellowstone gRPC 客户端
- `borsh`：Borsh 序列化/反序列化
- `solana-sdk`：Solana SDK
- `thiserror`：错误处理

## 许可证

MIT

## 参考

- [Crates.io](https://crates.io/crates/solana-pump-grpc-sdk) - 在 crates.io 上查看
- [文档](https://docs.rs/solana-pump-grpc-sdk) - 完整 API 文档
- [GitHub](https://github.com/vnxfsc/solana-pump-grpc-sdk) - 源代码和问题追踪
- [Pump 官方文档](https://github.com/pump-fun/pump-public-docs)
- [Yellowstone gRPC](https://github.com/rpcpool/yellowstone-grpc)
