use crate::oms::exchanges::Exchange;
use crate::oms::instruments::trading_pair::TradingPair;
use crate::ttcore::{decimal::Decimal, errors::TensorTradeError};

pub struct ExchangePair {
    pub id: String,
    pub exchange: Exchange,
    pub pair: TradingPair,
}

impl ExchangePair {
    pub fn price(&self) -> Result<Decimal, TensorTradeError> {
        self.exchange.quote_price(&self.pair)
    }
}
