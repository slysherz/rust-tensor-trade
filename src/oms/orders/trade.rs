use crate::oms::instruments::exchange_pair::ExchangePair;
use crate::oms::orders::Quantity;

pub enum TradeType {
    Limit,
    Market
}

pub enum TradeSide {
    Buy,
    Sell
}

pub struct Trade {
    order_id: String,
    step: i32,
    exchange_pair: ExchangePair,
    side: TradeSide,
    trade_type: TradeType,
    quantity: Quantity,
    price: f32,
    comission: Quantity
}

impl Trade {
    pub fn new(
        order_id: String,
        step: i32,
        exchange_pair: ExchangePair,
        side: TradeSide,
        trade_type: TradeType,
        quantity: Quantity,
        price: f32,
        comission: Quantity
    ) -> Trade {
        Trade {
            order_id,
            step,
            exchange_pair,
            side,
            trade_type,
            quantity,
            price,
            comission
        }
    }
}