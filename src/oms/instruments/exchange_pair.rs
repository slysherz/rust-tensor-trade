use crate::ttcore::decimal::Decimal;
use crate::oms::exchanges::Exchange;
use crate::oms::instruments::trading_pair::TradingPair;

pub struct ExchangePair {
    pub id: String,
    pub exchange: Exchange,
    pub pair: TradingPair
}

impl ExchangePair {
    pub fn price(&self) -> Decimal {
        self.exchange.quote_price(&self.pair)
    }

    pub fn inverse_price(&self) {
        // todo
    }
}