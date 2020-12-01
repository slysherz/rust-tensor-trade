use crate::ttcore::errors::TensorTradeError;
use crate::oms::orders::instruments::Instrument;

#[derive(PartialEq)]
pub struct TradingPair {
    pub base: Instrument,
    pub quote: Instrument
}

impl TradingPair {
    pub fn new(base: Instrument, quote: Instrument) -> Result<TradingPair, TensorTradeError> {
        if base == quote {
            Err(TensorTradeError::InvalidTradingPair{})
        } else {
            Ok(TradingPair{base, quote})
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}", self.base.symbol, self.quote.symbol)
    }
}