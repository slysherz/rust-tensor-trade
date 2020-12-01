use crate::oms::instruments::{
    exchange_pair::ExchangePair, 
    trading_pair::TradingPair, 
    quantity::Quantity};

use crate::ttcore::base::TimeIndexed;
use super::instruments::Instrument;

pub enum TradeType {
    Limit,
    Market
}

#[derive(PartialEq)]
pub enum TradeSide {
    Buy,
    Sell
}

impl TradeSide {
    pub fn instrument<'a>(&self, pair: &'a TradingPair) -> &'a Instrument {
        if self == &TradeSide::Buy {
            &pair.base
        } else {
            &pair.quote
        }
    }
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

impl TimeIndexed for Trade {}

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