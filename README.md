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
- 💰 **交易功能**：提供完整的交易指令构建功能，支持 Pump 和 PumpAMM 的买卖操作
- 🎭 **Mayhem Mode 支持**：自动处理 mayhem mode 代币，选择合适的 fee recipient 和 token program
- 🔀 **智能指令选择**：PumpAMM 根据 quote_mint 类型自动选择正确的指令类型

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

### 交易指令构建

SDK 提供了完整的交易指令构建功能，支持 Pump 和 PumpAMM 的买卖操作：

#### Pump (Bonding Curve) 交易

```rust
use solana_pump_grpc_sdk::{TradeClient, OptionBool};
use solana_sdk::pubkey::Pubkey;

// 创建 Pump 交易客户端
let client = TradeClient::new();

// 买入指令（普通模式）
let buy_ix = client.build_buy_instruction(
    &user,                      // 用户地址（signer）
    &mint,                      // 代币 mint 地址
    1000000,                    // 买入的代币数量
    100000000,                  // 最大 SOL 成本（lamports）
    OptionBool::Some(true),     // 跟踪交易量
    false,                      // is_mayhem_mode: 普通模式
)?;

// 买入指令（Mayhem 模式）
let buy_ix_mayhem = client.build_buy_instruction(
    &user,
    &mint,
    1000000,
    100000000,
    OptionBool::Some(true),
    true,                       // is_mayhem_mode: Mayhem 模式
)?;

// 卖出指令
let sell_ix = client.build_sell_instruction(
    &user,                      // 用户地址（signer）
    &mint,                      // 代币 mint 地址
    1000000,                    // 卖出的代币数量
    95000000,                   // 最小 SOL 输出（lamports）
    false,                      // is_mayhem_mode: 普通模式
)?;
```

#### PumpAMM 交易

```rust
use solana_pump_grpc_sdk::{TradeClient, OptionBool, wsol_mint};
use solana_sdk::pubkey::Pubkey;

// 创建 PumpAMM 交易客户端
let client = TradeClient::pump_amm();

let pool = "pool_address".parse::<Pubkey>()?;
let base_mint = "token_mint".parse::<Pubkey>()?;
let quote_mint = wsol_mint();   // 通常是 WSOL
let coin_creator = "creator_address".parse::<Pubkey>()?;
let protocol_fee_recipient = "fee_recipient".parse::<Pubkey>()?;

// 买入指令（quote_mint 是 WSOL/USDC）
// SDK 会自动使用买入指令类型，并添加 volume accumulator
let buy_ix = client.build_pump_amm_buy_instruction(
    &user,
    &pool,
    &base_mint,
    &quote_mint,               // WSOL
    &coin_creator,
    &protocol_fee_recipient,
    1000000,                   // base_amount_out: 期望买入的 base token 数量
    100000000,                 // max_quote_amount_in: 最大 quote token 输入
    OptionBool::Some(true),    // track_volume: 跟踪交易量
    false,                     // is_mayhem_mode: 普通模式
)?;

// 卖出指令（quote_mint 是 WSOL/USDC）
// SDK 会自动使用卖出指令类型，不添加 volume accumulator
let sell_ix = client.build_pump_amm_sell_instruction(
    &user,
    &pool,
    &base_mint,
    &quote_mint,               // WSOL
    &coin_creator,
    &protocol_fee_recipient,
    1000000,                   // base_amount_in: 卖出的 base token 数量
    95000000,                  // min_quote_amount_out: 最小 quote token 输出
    false,                     // is_mayhem_mode: 普通模式
)?;
```

**注意**：
- PumpAMM 会根据 `quote_mint` 是否为 WSOL/USDC 自动选择指令类型和账户列表
- 如果 `quote_mint` 是 WSOL/USDC，使用标准的买入/卖出指令
- 如果 `quote_mint` 不是 WSOL/USDC，使用反向交易指令（买入指令会调用卖出方法，反之亦然）
- Mayhem mode 代币会自动使用正确的 fee recipient 和 token program

#### OptionBool 说明

`OptionBool` 用于表示可选的布尔值，主要用于 `track_volume` 参数：

```rust
use solana_pump_grpc_sdk::OptionBool;

// 三种状态：
OptionBool::None          // 不跟踪交易量，序列化为 [0]
OptionBool::Some(true)    // 跟踪交易量，序列化为 [1, 1]
OptionBool::Some(false)   // 不跟踪交易量（显式），序列化为 [1, 0]
```

**使用建议**：
- 买入指令：通常使用 `OptionBool::Some(true)` 来跟踪交易量
- 卖出指令：不需要 `track_volume` 参数


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

### `TradeClient`

交易客户端，用于构建 Pump 和 PumpAMM 的交易指令。

```rust
impl TradeClient {
    // 创建 Pump 交易客户端
    pub fn new() -> Self;
    
    // 创建 PumpAMM 交易客户端
    pub fn pump_amm() -> Self;
    
    // 使用自定义程序 ID 创建客户端
    pub fn with_program_id(program_id: Pubkey) -> Self;
    
    // 构建 Pump 买入指令
    pub fn build_buy_instruction(
        &self,
        user: &Pubkey,
        mint: &Pubkey,
        amount: u64,
        max_sol_cost: u64,
        track_volume: OptionBool,
        is_mayhem_mode: bool,
    ) -> Result<Instruction>;
    
    // 构建 Pump 卖出指令
    pub fn build_sell_instruction(
        &self,
        user: &Pubkey,
        mint: &Pubkey,
        amount: u64,
        min_sol_output: u64,
        is_mayhem_mode: bool,
    ) -> Result<Instruction>;
    
    // 构建 PumpAMM 买入指令
    pub fn build_pump_amm_buy_instruction(
        &self,
        user: &Pubkey,
        pool: &Pubkey,
        base_mint: &Pubkey,
        quote_mint: &Pubkey,
        coin_creator: &Pubkey,
        protocol_fee_recipient: &Pubkey,
        base_amount_out: u64,
        max_quote_amount_in: u64,
        track_volume: OptionBool,
        is_mayhem_mode: bool,
    ) -> Result<Instruction>;
    
    // 构建 PumpAMM 卖出指令
    pub fn build_pump_amm_sell_instruction(
        &self,
        user: &Pubkey,
        pool: &Pubkey,
        base_mint: &Pubkey,
        quote_mint: &Pubkey,
        coin_creator: &Pubkey,
        protocol_fee_recipient: &Pubkey,
        base_amount_in: u64,
        min_quote_amount_out: u64,
        is_mayhem_mode: bool,
    ) -> Result<Instruction>;
}
```

**特性**：
- 自动根据 mayhem mode 选择正确的 fee recipient 和 token program
- PumpAMM 自动根据 `quote_mint` 类型选择指令类型和账户列表
- 自动派生所需的 PDA（Program Derived Address）
- 完整的账户列表构建，符合程序要求

### 辅助函数

SDK 提供了多个辅助函数用于派生 PDA 和检查 mint 类型：

```rust
// Pump 相关
pub fn pump_program_id() -> Pubkey;
pub fn derive_global_pda(program_id: &Pubkey) -> (Pubkey, u8);
pub fn derive_bonding_curve_pda(mint: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8);
pub fn derive_creator_vault_pda(creator: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8);
pub fn derive_user_associated_token_account(user: &Pubkey, mint: &Pubkey) -> Pubkey;

// PumpAMM 相关
pub fn pump_amm_program_id() -> Pubkey;
pub fn derive_pump_amm_global_config_pda(program_id: &Pubkey) -> (Pubkey, u8);
pub fn derive_pump_amm_pool_pda(
    index: u16,
    creator: &Pubkey,
    base_mint: &Pubkey,
    quote_mint: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8);

// 工具函数
pub fn wsol_mint() -> Pubkey;
pub fn usdc_mint() -> Pubkey;
pub fn is_wsol(mint: &Pubkey) -> bool;
pub fn is_usdc(mint: &Pubkey) -> bool;
pub fn is_wsol_or_usdc(quote_mint: &Pubkey) -> bool;
pub fn get_fee_recipient(is_mayhem_mode: bool) -> Pubkey;
pub fn get_token_program_id(is_mayhem_mode: bool) -> Pubkey;
```


## 运行示例

项目包含多个示例：

### 基本事件监听示例

```bash
# 运行基本示例（无需设置环境变量）
cargo run --example basic

# 或者设置日志级别
RUST_LOG=debug cargo run --example basic
```

### 交易指令构建示例

```bash
# 运行交易指令构建示例
cargo run --example trading

# 这个示例展示了如何使用 TradeClient 构建各种交易指令：
# - Pump 普通模式和 Mayhem 模式的买入/卖出指令
# - PumpAMM 买入/卖出指令（自动选择指令类型）
# - OptionBool 的使用方法
```

所有示例都是只读的，不会发送实际的交易到链上。它们仅用于演示如何构建指令。

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
│   ├── trading/            # 交易功能模块
│   │   ├── mod.rs
│   │   ├── client.rs       # TradeClient 实现
│   │   └── helpers.rs      # PDA 派生和辅助函数
│   └── error.rs            # 错误类型
└── examples/
    ├── basic.rs            # 基本事件监听示例
    └── trading.rs          # 交易指令构建示例
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
- `solana-sdk`：Solana SDK（用于交易指令构建）
- `spl-token`：SPL Token 程序支持
- `thiserror`：错误处理

## 功能对比

### 事件监听 vs 交易指令构建

- **事件监听**：被动监听链上事件，用于数据分析、监控等场景
- **交易指令构建**：主动构建交易指令，用于实现交易机器人、跟单策略等

SDK 同时支持这两种功能，可以结合使用：
1. 监听链上事件
2. 根据事件数据构建交易指令
3. 发送交易指令到链上（需要用户自己实现交易发送部分）

## 注意事项

### 交易指令构建

- SDK **只负责构建指令**，不负责发送交易
- 用户需要自己实现：
  - 钱包签名
  - 交易发送
  - 交易确认
  - 错误重试等逻辑

### Mayhem Mode

- Mayhem mode 代币使用 Token Program 2022 和不同的 fee recipient
- SDK 会根据 `is_mayhem_mode` 参数自动选择合适的配置
- 建议从链上读取 bonding curve 账户数据来判断是否为 mayhem mode

### PumpAMM 指令类型

- PumpAMM 的买入/卖出指令类型会根据 `quote_mint` 自动选择
- 如果 `quote_mint` 是 WSOL/USDC，使用标准买入/卖出指令
- 如果 `quote_mint` 不是 WSOL/USDC，使用反向交易指令
- SDK 会自动处理账户列表的差异（如 volume accumulator）

## 许可证

MIT

## 参考

- [Crates.io](https://crates.io/crates/solana-pump-grpc-sdk) - 在 crates.io 上查看
- [文档](https://docs.rs/solana-pump-grpc-sdk) - 完整 API 文档
- [GitHub](https://github.com/vnxfsc/solana-pump-grpc-sdk) - 源代码和问题追踪
- [Pump 官方文档](https://github.com/pump-fun/pump-public-docs)
- [Yellowstone gRPC](https://github.com/rpcpool/yellowstone-grpc)
