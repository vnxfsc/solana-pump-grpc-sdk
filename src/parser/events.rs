use crate::models::{
    BuyEvent, CompleteEvent, CreateEvent, CreatePoolEvent, CreateV2Event, SellEvent, TradeEvent,
};
use base64::{engine::general_purpose, Engine};
use borsh::BorshDeserialize;
use std::{cell::RefCell, error::Error, ops::ControlFlow};

const PROGRAM_DATA: &str = "Program data: ";

// 导出所有事件类型的 discriminator 常量，供性能关键路径使用
// 这样可以避免在运行时调用函数获取 discriminator
pub const CREATE_DISCRIMINATOR: &[u8] = &[27, 114, 169, 77, 222, 235, 99, 118];
pub const CREATE_V2_DISCRIMINATOR: &[u8] = &[214, 144, 76, 236, 95, 139, 49, 180];
pub const COMPLETE_DISCRIMINATOR: &[u8] = &[95, 114, 97, 156, 212, 46, 152, 8];
pub const TRADE_DISCRIMINATOR: &[u8] = &[189, 219, 127, 211, 78, 230, 97, 238];
pub const BUY_DISCRIMINATOR: &[u8] = &[103, 244, 82, 31, 44, 245, 119, 119];
pub const CREATE_POOL_DISCRIMINATOR: &[u8] = &[177, 49, 12, 210, 160, 118, 167, 116];
pub const SELL_DISCRIMINATOR: &[u8] = &[62, 47, 55, 10, 165, 3, 220, 42];

thread_local! {
    static PROGRAM_LOG_BUFFER: RefCell<Vec<u8>> = RefCell::new(Vec::with_capacity(1024));
}

pub fn visit_program_logs<F>(logs: &[String], mut visitor: F)
where
    F: FnMut(&[u8], &[u8]) -> ControlFlow<()>,
{
    PROGRAM_LOG_BUFFER.with(|buffer_cell| {
        let mut buffer = buffer_cell.borrow_mut();

        for log in logs.iter().rev() {
            let payload = match log.strip_prefix(PROGRAM_DATA) {
                Some(p) => p,
                None => continue,
            };

            buffer.clear();
            if general_purpose::STANDARD
                .decode_vec(payload, &mut buffer)
                .is_err()
            {
                continue;
            }

            if buffer.len() < 8 {
                continue;
            }

            let (discriminator, data) = buffer.split_at(8);

            if visitor(discriminator, data).is_break() {
                break;
            }
        }
    });
}

pub trait EventTrait: Sized + std::fmt::Debug {
    fn discriminator() -> [u8; 8];
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>>;
    fn valid_discrminator(head: &[u8]) -> bool;

    #[allow(dead_code)]
    fn parse_logs<T: EventTrait>(logs: &[String]) -> Option<T> {
        let mut result = None;

        visit_program_logs(logs, |discriminator, data| {
            if T::valid_discrminator(discriminator) {
                if let Ok(event) = T::from_bytes(data) {
                    result = Some(event);
                    return ControlFlow::Break(());
                }
            }
            ControlFlow::Continue(())
        });

        result
    }
}

impl EventTrait for CreateEvent {
    fn discriminator() -> [u8; 8] {
        CREATE_DISCRIMINATOR.try_into().unwrap()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::try_from_slice(bytes).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn valid_discrminator(discr: &[u8]) -> bool {
        discr == CREATE_DISCRIMINATOR
    }
}

impl EventTrait for CreateV2Event {
    fn discriminator() -> [u8; 8] {
        CREATE_V2_DISCRIMINATOR.try_into().unwrap()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::try_from_slice(bytes).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn valid_discrminator(discr: &[u8]) -> bool {
        discr == CREATE_V2_DISCRIMINATOR
    }
}

impl EventTrait for CompleteEvent {
    fn discriminator() -> [u8; 8] {
        COMPLETE_DISCRIMINATOR.try_into().unwrap()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::try_from_slice(bytes).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn valid_discrminator(discr: &[u8]) -> bool {
        discr == COMPLETE_DISCRIMINATOR
    }
}

impl EventTrait for TradeEvent {
    fn discriminator() -> [u8; 8] {
        TRADE_DISCRIMINATOR.try_into().unwrap()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::try_from_slice(bytes).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn valid_discrminator(discr: &[u8]) -> bool {
        discr == TRADE_DISCRIMINATOR
    }
}

impl EventTrait for BuyEvent {
    fn discriminator() -> [u8; 8] {
        BUY_DISCRIMINATOR.try_into().unwrap()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::try_from_slice(bytes).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn valid_discrminator(discr: &[u8]) -> bool {
        discr == BUY_DISCRIMINATOR
    }
}

impl EventTrait for CreatePoolEvent {
    fn discriminator() -> [u8; 8] {
        CREATE_POOL_DISCRIMINATOR.try_into().unwrap()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::try_from_slice(bytes).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn valid_discrminator(discr: &[u8]) -> bool {
        discr == CREATE_POOL_DISCRIMINATOR
    }
}

impl EventTrait for SellEvent {
    fn discriminator() -> [u8; 8] {
        SELL_DISCRIMINATOR.try_into().unwrap()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::try_from_slice(bytes).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    fn valid_discrminator(discr: &[u8]) -> bool {
        discr == SELL_DISCRIMINATOR
    }
}
