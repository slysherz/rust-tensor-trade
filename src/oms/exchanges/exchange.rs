use rust_decimal::prelude::*;
use crate::oms::instruments::trading_pair::TradingPair;
use std::collections::HashMap;

trait StreamLike {
    fn rename(&mut self, new_name: String);
    fn value(&self) -> f32;
}

pub struct ExchangeOptions {
    /// The percentage of the order size taken by the exchange
    commission: f32,
    /// The minimum trade size an order can have
    min_trade_size: f32,
    /// The maximum trade size an order can have
    max_trade_size: f32,
    /// The minimum price an exchange can have
    min_trade_price: f32,
    /// The maximum price an exchange can have
    max_trade_price: f32,
    /// Whether live orders should be submitted to the exchange
    is_live: bool
}

impl ExchangeOptions {
    fn new() -> ExchangeOptions {
        ExchangeOptions {
            commission: 0.0003,
            min_trade_size: 1e-6,
            max_trade_size: 1e6,
            min_trade_price: 1e-8,
            max_trade_price: 1e8,
            is_live: false
        }
    }
}

pub struct Exchange {
    name: String,
    // service: Arc<Service>,
    options: ExchangeOptions,
    price_streams: HashMap<String, Box<dyn StreamLike>>
}

impl Exchange {
    pub fn quote_price(&self, trading_pair: &TradingPair) -> Decimal {
        let stream_name = trading_pair.to_string();
        let value = match self.price_streams.get(&stream_name) {
            Some(stream) => stream.value(),
            None => 0.0
        };

        // Todo: consider type and rounding failures
        Decimal::from_f32(value).unwrap().round_dp(trading_pair.base.precision)
    }
}