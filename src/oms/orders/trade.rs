use crate::oms::instruments::{
    ExchangePair, 
    TradingPair, 
    Quantity,
    Instrument
};

use crate::ttcore::base::TimeIndexed;

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
    pub order_id: String,
    pub step: i32,
    pub exchange_pair: ExchangePair,
    pub side: TradeSide,
    pub trade_type: TradeType,
    pub quantity: Quantity,
    pub price: f32,
    pub commission: Quantity
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
        commission: Quantity
    ) -> Trade {
        Trade {
            order_id,
            step,
            exchange_pair,
            side,
            trade_type,
            quantity,
            price,
            commission
        }
    }
}