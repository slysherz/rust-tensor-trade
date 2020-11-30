use crate::oms::orders::trade::{TradeSide, TradeType};
use crate::oms::orders::quantity::Quantity;
use crate::oms::instruments::exchange_pair::ExchangePair;
use crate::oms::exchanges::exchange::Exchange;

pub enum OrderStatus {
    Pending,
    Open,
    Cancelled,
    PartiallyFilled,
    Filled
}

pub struct Order {
    step: i32,
    side: TradeSide,
    trade_type: TradeType,
    exchange_pair: ExchangePair,
    quantity: Quantity,
    // portfolio: Portfolio,
    price: f32,
    criteria: fn(Order, Exchange) -> bool,
    path_id: String,
    start: i32,
    end: Option<i32>
}

impl Order {
    fn new() {}
}