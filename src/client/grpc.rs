use futures_util::{SinkExt, StreamExt};
use log::error;
use solana_sdk::signature::Signature;
use std::{collections::HashMap, ops::ControlFlow, sync::Arc};
use tokio::sync::Mutex;
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient};
use yellowstone_grpc_proto::geyser::{
    subscribe_update::UpdateOneof, SubscribeRequest,
    SubscribeRequestFilterTransactions, SubscribeRequestPing,
};

use crate::{
    error::{Error, Result},
    models::{
        BuyEvent, CompleteEvent, CreateEvent, CreatePoolEvent, CreateV2Event, SellEvent, TradeEvent,
    },
    parser::events::{
        visit_program_logs, EventTrait,
        BUY_DISCRIMINATOR, COMPLETE_DISCRIMINATOR, CREATE_DISCRIMINATOR, CREATE_POOL_DISCRIMINATOR,
        CREATE_V2_DISCRIMINATOR, SELL_DISCRIMINATOR, TRADE_DISCRIMINATOR,
    },
};

use super::{config::Config, handler::EventHandler, handler::EventContext};

/// gRPC客户端
#[derive(Clone)]
pub struct GrpcClient {
    config: Config,
}

impl GrpcClient {
    /// 创建新的gRPC客户端
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 订阅指定程序ID的事件
    /// 
    /// # 参数
    /// 
    /// * `program_id` - 要订阅的程序ID
    /// * `handler` - 事件处理器，实现 `EventHandler` trait
    /// 
    /// # 示例
    /// 
    /// ```no_run
    /// use solana_pump_grpc_sdk::{GrpcClient, Config, EventHandler};
    /// use solana_pump_grpc_sdk::models::*;
    /// 
    /// struct MyHandler;
    /// impl EventHandler for MyHandler {
    ///     fn on_create_event(&self, event: &CreateEvent, ctx: &EventContext) {
    ///         println!("Received CreateEvent: {:?}", event);
    ///     }
    /// }
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::new("https://solana-yellowstone-grpc.publicnode.com".to_string());
    /// let client = GrpcClient::new(config);
    /// let handler = MyHandler;
    /// client.subscribe("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P".to_string(), handler).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn subscribe<H: EventHandler>(
        &self,
        program_id: String,
        handler: H,
    ) -> Result<()> {
        let tls_config = ClientTlsConfig::new().with_native_roots();

        let mut builder = GeyserGrpcClient::build_from_shared(self.config.url.clone())
            .map_err(|e| Error::GrpcBuilder(e.to_string()))?;
        
        builder = builder
            .tls_config(tls_config)
            .map_err(|e| Error::TlsConfig(e.to_string()))?
            .connect_timeout(self.config.connect_timeout)
            .keep_alive_while_idle(self.config.keep_alive_while_idle)
            .timeout(self.config.timeout);

        let client = builder
            .connect()
            .await
            .map_err(|e| Error::GrpcConnection(e.to_string()))?;

        let client = Arc::new(Mutex::new(client));

        let addrs = vec![program_id.clone()];
        let subscribe_request = SubscribeRequest {
            transactions: HashMap::from([(
                "client".to_string(),
                SubscribeRequestFilterTransactions {
                    vote: Some(false),
                    failed: Some(false),
                    signature: None,
                    account_include: addrs,
                    account_exclude: vec![],
                    account_required: vec![],
                },
            )]),
            commitment: Some(self.config.commitment.into()),
            ..Default::default()
        };

        let (mut subscribe_tx, mut stream) = client
            .lock()
            .await
            .subscribe_with_request(Some(subscribe_request))
            .await
            .map_err(|e| Error::SubscribeError(e.to_string()))?;

        while let Some(message) = stream.next().await {
            match message {
                Ok(msg) => match msg.update_oneof {
                    Some(UpdateOneof::Transaction(sut)) => {
                        let slot = sut.slot;
                        if let Some(tx_info) = sut.transaction {
                            let tx_index = tx_info.index;
                            let signature = Signature::try_from(tx_info.signature.as_slice())
                                .map_err(|_| Error::SignatureParse)?;
                                   if let Some(meta) = tx_info.meta {
                                       let start = std::time::Instant::now();
                                       let logs = meta.log_messages;
                                       if !logs.is_empty() {
                                           self.handle_logs(
                                               slot,
                                               tx_index,
                                               &signature,
                                               &logs,
                                               start,
                                               &handler,
                                           )
                                           .await?;
                                       }
                                   }
                        }
                    }
                    Some(UpdateOneof::Ping(_)) => {
                        let _ = subscribe_tx
                            .send(SubscribeRequest {
                                ping: Some(SubscribeRequestPing { id: 1 }),
                                ..Default::default()
                            })
                            .await;
                    }
                    _ => {}
                },
                Err(e) => {
                    error!("Stream error: {:?}", e);
                    return Err(Error::SubscribeError(e.to_string()));
                }
            }
        }
        Ok(())
    }

    async fn handle_logs<H: EventHandler>(
        &self,
        slot: u64,
        tx_index: u64,
        signature: &Signature,
        logs: &[String],
        start_time: std::time::Instant,
        handler: &H,
    ) -> Result<()> {
        // 优化：使用 events.rs 中导出的 discriminator 常量，避免重复定义

        let mut logged_create = false;
        let mut logged_create_v2 = false;
        let mut logged_complete = false;
        let mut logged_trade = false;
        let mut logged_buy = false;
        let mut logged_create_pool = false;
        let mut logged_sell = false;

        // 优化：预先创建基础 EventContext，只更新 elapsed
        let base_ctx = EventContext {
            slot,
            tx_index,
            signature: *signature,
            timestamp: start_time,
            elapsed: std::time::Duration::ZERO,
        };

        // 优化：内联函数检查是否所有事件都已找到（避免重复代码）
        #[inline(always)]
        fn all_logged(
            create: bool,
            create_v2: bool,
            complete: bool,
            trade: bool,
            buy: bool,
            create_pool: bool,
            sell: bool,
        ) -> bool {
            create && create_v2 && complete && trade && buy && create_pool && sell
        }

        visit_program_logs(logs, |discriminator, data| {
            // 优化：使用直接字节比较，避免函数调用开销
            // 优化：优先检查最常见的事件类型（Buy/Sell > Trade > 其他）
            if discriminator == BUY_DISCRIMINATOR {
                if !logged_buy {
                    if let Ok(buy_event) = BuyEvent::from_bytes(data) {
                        let elapsed = std::time::Instant::now().duration_since(start_time);
                        handler.on_buy_event(
                            &buy_event,
                            &EventContext { elapsed, ..base_ctx },
                        );
                        logged_buy = true;
                    }
                }
                if all_logged(logged_create, logged_create_v2, logged_complete, logged_trade,
                        logged_buy, logged_create_pool, logged_sell) {
                    return ControlFlow::Break(());
                }
                return ControlFlow::Continue(());
            }

            if discriminator == SELL_DISCRIMINATOR {
                if !logged_sell {
                    if let Ok(sell_event) = SellEvent::from_bytes(data) {
                        let elapsed = std::time::Instant::now().duration_since(start_time);
                        handler.on_sell_event(
                            &sell_event,
                            &EventContext { elapsed, ..base_ctx },
                        );
                        logged_sell = true;
                    }
                }
                if all_logged(logged_create, logged_create_v2, logged_complete, logged_trade,
                        logged_buy, logged_create_pool, logged_sell) {
                    return ControlFlow::Break(());
                }
                return ControlFlow::Continue(());
            }

            if discriminator == TRADE_DISCRIMINATOR {
                if !logged_trade {
                    if let Ok(trade_event) = TradeEvent::from_bytes(data) {
                        let elapsed = std::time::Instant::now().duration_since(start_time);
                        handler.on_trade_event(
                            &trade_event,
                            &EventContext { elapsed, ..base_ctx },
                        );
                        logged_trade = true;
                    }
                }
                if all_logged(logged_create, logged_create_v2, logged_complete, logged_trade,
                        logged_buy, logged_create_pool, logged_sell) {
                    return ControlFlow::Break(());
                }
                return ControlFlow::Continue(());
            }

            if discriminator == CREATE_DISCRIMINATOR {
                if !logged_create {
                    if let Ok(create_event) = CreateEvent::from_bytes(data) {
                        let elapsed = std::time::Instant::now().duration_since(start_time);
                        handler.on_create_event(
                            &create_event,
                            &EventContext { elapsed, ..base_ctx },
                        );
                        logged_create = true;
                    }
                }
                if all_logged(logged_create, logged_create_v2, logged_complete, logged_trade,
                        logged_buy, logged_create_pool, logged_sell) {
                    return ControlFlow::Break(());
                }
                return ControlFlow::Continue(());
            }

            if discriminator == CREATE_V2_DISCRIMINATOR {
                if !logged_create_v2 {
                    if let Ok(create_v2_event) = CreateV2Event::from_bytes(data) {
                        let elapsed = std::time::Instant::now().duration_since(start_time);
                        handler.on_create_v2_event(
                            &create_v2_event,
                            &EventContext { elapsed, ..base_ctx },
                        );
                        logged_create_v2 = true;
                    }
                }
                if all_logged(logged_create, logged_create_v2, logged_complete, logged_trade,
                        logged_buy, logged_create_pool, logged_sell) {
                    return ControlFlow::Break(());
                }
                return ControlFlow::Continue(());
            }

            if discriminator == COMPLETE_DISCRIMINATOR {
                if !logged_complete {
                    if let Ok(complete_event) = CompleteEvent::from_bytes(data) {
                        let elapsed = std::time::Instant::now().duration_since(start_time);
                        handler.on_complete_event(
                            &complete_event,
                            &EventContext { elapsed, ..base_ctx },
                        );
                        logged_complete = true;
                    }
                }
                if all_logged(logged_create, logged_create_v2, logged_complete, logged_trade,
                        logged_buy, logged_create_pool, logged_sell) {
                    return ControlFlow::Break(());
                }
                return ControlFlow::Continue(());
            }

            if discriminator == CREATE_POOL_DISCRIMINATOR {
                if !logged_create_pool {
                    if let Ok(create_pool_event) = CreatePoolEvent::from_bytes(data) {
                        let elapsed = std::time::Instant::now().duration_since(start_time);
                        handler.on_create_pool_event(
                            &create_pool_event,
                            &EventContext { elapsed, ..base_ctx },
                        );
                        logged_create_pool = true;
                    }
                }
                if all_logged(logged_create, logged_create_v2, logged_complete, logged_trade,
                        logged_buy, logged_create_pool, logged_sell) {
                    return ControlFlow::Break(());
                }
            }

            ControlFlow::Continue(())
        });
        Ok(())
    }
}