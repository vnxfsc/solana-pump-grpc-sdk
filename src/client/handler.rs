use crate::models::*;
use solana_sdk::signature::Signature;

/// 事件上下文，包含事件发生的上下文信息
#[derive(Clone, Debug)]
pub struct EventContext {
    /// 区块槽位
    pub slot: u64,
    /// 交易索引
    pub tx_index: u64,
    /// 交易签名
    pub signature: Signature,
    /// 事件处理开始时间戳
    pub timestamp: std::time::Instant,
    /// 从开始处理到当前事件的耗时
    pub elapsed: std::time::Duration,
}

/// 事件处理器trait
/// 
/// 用户需要实现这个trait来处理各种事件。
/// 所有方法都有默认的空实现，用户只需实现感兴趣的事件处理方法。
pub trait EventHandler: Send + Sync {
    /// 处理 CreateEvent
    fn on_create_event(&self, _event: &CreateEvent, _ctx: &EventContext) {}

    /// 处理 CreateV2Event
    fn on_create_v2_event(&self, _event: &CreateV2Event, _ctx: &EventContext) {}

    /// 处理 CompleteEvent
    fn on_complete_event(&self, _event: &CompleteEvent, _ctx: &EventContext) {}

    /// 处理 TradeEvent
    fn on_trade_event(&self, _event: &TradeEvent, _ctx: &EventContext) {}

    /// 处理 BuyEvent
    fn on_buy_event(&self, _event: &BuyEvent, _ctx: &EventContext) {}

    /// 处理 SellEvent
    fn on_sell_event(&self, _event: &SellEvent, _ctx: &EventContext) {}

    /// 处理 CreatePoolEvent
    fn on_create_pool_event(&self, _event: &CreatePoolEvent, _ctx: &EventContext) {}
}

/// 默认的事件处理器实现（什么都不做）
impl EventHandler for () {}

/// 事件过滤器配置
/// 
/// 用于指定要打印哪些事件类型
#[derive(Clone, Debug)]
pub struct EventFilter {
    /// 是否打印 CreateEvent
    pub create: bool,
    /// 是否打印 CreateV2Event
    pub create_v2: bool,
    /// 是否打印 CompleteEvent
    pub complete: bool,
    /// 是否打印 TradeEvent
    pub trade: bool,
    /// 是否打印 BuyEvent
    pub buy: bool,
    /// 是否打印 SellEvent
    pub sell: bool,
    /// 是否打印 CreatePoolEvent
    pub create_pool: bool,
}

impl EventFilter {
    /// 创建新的事件过滤器，默认所有事件都启用
    pub fn all() -> Self {
        Self {
            create: true,
            create_v2: true,
            complete: true,
            trade: true,
            buy: true,
            sell: true,
            create_pool: true,
        }
    }

    /// 创建新的事件过滤器，默认所有事件都禁用
    pub fn none() -> Self {
        Self {
            create: false,
            create_v2: false,
            complete: false,
            trade: false,
            buy: false,
            sell: false,
            create_pool: false,
        }
    }

    /// 只打印 Pump 相关事件（Create, CreateV2, Complete, Trade）
    pub fn pump_only() -> Self {
        Self {
            create: true,
            create_v2: true,
            complete: true,
            trade: true,
            buy: false,
            sell: false,
            create_pool: false,
        }
    }

    /// 只打印 PumpAmm 相关事件（Buy, Sell, CreatePool）
    pub fn pumpamm_only() -> Self {
        Self {
            create: false,
            create_v2: false,
            complete: false,
            trade: false,
            buy: true,
            sell: true,
            create_pool: true,
        }
    }
}

impl Default for EventFilter {
    fn default() -> Self {
        Self::all()
    }
}

/// 日志事件处理器
/// 
/// 一个简单的事件处理器实现，将所有事件记录到日志中。
/// 输出格式：`{EventName} {{ slot:{}, tx_index:{}, signature:{}, event:{:?} }}`
#[derive(Clone, Copy, Default)]
pub struct LoggingEventHandler;

impl EventHandler for LoggingEventHandler {
    fn on_create_event(&self, event: &CreateEvent, ctx: &EventContext) {
        log::info!(
            "CreateEvent {{ elapsed:{:?}, slot:{}, tx_index:{}, signature:{}, event:{:?} }}",
            ctx.elapsed, ctx.slot, ctx.tx_index, ctx.signature, event
        );
    }

    fn on_create_v2_event(&self, event: &CreateV2Event, ctx: &EventContext) {
        log::info!(
            "CreateV2Event {{ elapsed:{:?}, slot:{}, tx_index:{}, signature:{}, event:{:?} }}",
            ctx.elapsed, ctx.slot, ctx.tx_index, ctx.signature, event
        );
    }

    fn on_complete_event(&self, event: &CompleteEvent, ctx: &EventContext) {
        log::info!(
            "CompleteEvent {{ elapsed:{:?}, slot:{}, tx_index:{}, signature:{}, event:{:?} }}",
            ctx.elapsed, ctx.slot, ctx.tx_index, ctx.signature, event
        );
    }

    fn on_trade_event(&self, event: &TradeEvent, ctx: &EventContext) {
        log::info!(
            "TradeEvent {{ elapsed:{:?}, slot:{}, tx_index:{}, signature:{}, event:{:?} }}",
            ctx.elapsed, ctx.slot, ctx.tx_index, ctx.signature, event
        );
    }

    fn on_buy_event(&self, event: &BuyEvent, ctx: &EventContext) {
        log::info!(
            "BuyEvent {{ elapsed:{:?}, slot:{}, tx_index:{}, signature:{}, event:{:?} }}",
            ctx.elapsed, ctx.slot, ctx.tx_index, ctx.signature, event
        );
    }

    fn on_sell_event(&self, event: &SellEvent, ctx: &EventContext) {
        log::info!(
            "SellEvent {{ elapsed:{:?}, slot:{}, tx_index:{}, signature:{}, event:{:?} }}",
            ctx.elapsed, ctx.slot, ctx.tx_index, ctx.signature, event
        );
    }

    fn on_create_pool_event(&self, event: &CreatePoolEvent, ctx: &EventContext) {
        log::info!(
            "CreatePoolEvent {{ elapsed:{:?}, slot:{}, tx_index:{}, signature:{}, event:{:?} }}",
            ctx.elapsed, ctx.slot, ctx.tx_index, ctx.signature, event
        );
    }
}

/// 可过滤的日志事件处理器
/// 
/// 根据 `EventFilter` 配置选择性打印事件
#[derive(Clone)]
pub struct FilteredLoggingEventHandler {
    filter: EventFilter,
}

impl FilteredLoggingEventHandler {
    /// 创建新的过滤器日志事件处理器
    pub fn new(filter: EventFilter) -> Self {
        Self { filter }
    }

    /// 使用默认过滤器（所有事件都启用）创建处理器
    pub fn default() -> Self {
        Self {
            filter: EventFilter::default(),
        }
    }
}

impl EventHandler for FilteredLoggingEventHandler {
    fn on_create_event(&self, event: &CreateEvent, ctx: &EventContext) {
        if self.filter.create {
            log::info!(
                "CreateEvent {{ elapsed:{:?}, slot:{}, tx_index:{}, signature:{}, event:{:?} }}",
                ctx.elapsed, ctx.slot, ctx.tx_index, ctx.signature, event
            );
        }
    }

    fn on_create_v2_event(&self, event: &CreateV2Event, ctx: &EventContext) {
        if self.filter.create_v2 {
            log::info!(
                "CreateV2Event {{ elapsed:{:?}, slot:{}, tx_index:{}, signature:{}, event:{:?} }}",
                ctx.elapsed, ctx.slot, ctx.tx_index, ctx.signature, event
            );
        }
    }

    fn on_complete_event(&self, event: &CompleteEvent, ctx: &EventContext) {
        if self.filter.complete {
            log::info!(
                "CompleteEvent {{ elapsed:{:?}, slot:{}, tx_index:{}, signature:{}, event:{:?} }}",
                ctx.elapsed, ctx.slot, ctx.tx_index, ctx.signature, event
            );
        }
    }

    fn on_trade_event(&self, event: &TradeEvent, ctx: &EventContext) {
        if self.filter.trade {
            log::info!(
                "TradeEvent {{ elapsed:{:?}, slot:{}, tx_index:{}, signature:{}, event:{:?} }}",
                ctx.elapsed, ctx.slot, ctx.tx_index, ctx.signature, event
            );
        }
    }

    fn on_buy_event(&self, event: &BuyEvent, ctx: &EventContext) {
        if self.filter.buy {
            log::info!(
                "BuyEvent {{ elapsed:{:?}, slot:{}, tx_index:{}, signature:{}, event:{:?} }}",
                ctx.elapsed, ctx.slot, ctx.tx_index, ctx.signature, event
            );
        }
    }

    fn on_sell_event(&self, event: &SellEvent, ctx: &EventContext) {
        if self.filter.sell {
            log::info!(
                "SellEvent {{ elapsed:{:?}, slot:{}, tx_index:{}, signature:{}, event:{:?} }}",
                ctx.elapsed, ctx.slot, ctx.tx_index, ctx.signature, event
            );
        }
    }

    fn on_create_pool_event(&self, event: &CreatePoolEvent, ctx: &EventContext) {
        if self.filter.create_pool {
            log::info!(
                "CreatePoolEvent {{ elapsed:{:?}, slot:{}, tx_index:{}, signature:{}, event:{:?} }}",
                ctx.elapsed, ctx.slot, ctx.tx_index, ctx.signature, event
            );
        }
    }
}
