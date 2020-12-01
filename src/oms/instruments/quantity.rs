use std::ops;
use crate::ttcore::decimal::{ Decimal, ToPrimitive };
use crate::ttcore::errors::TensorTradeError;
use crate::oms::instruments::{ ExchangePair, Instrument };

#[derive(Debug)]
pub struct Quantity {
    pub instrument: Instrument,
    pub size: Decimal,
    pub path_id: String
}

impl Quantity {
    pub fn new(
        instrument: Instrument,
        size: Decimal,
        path_id: String
    ) -> Result<Quantity, TensorTradeError>{
        if size < Decimal::new(0, 0) {
            let fsize = match size.to_f32() {
                Some(f) => f,
                None => 0.0
            };

            Err(TensorTradeError::InvalidNegativeQuantity{size: fsize})
        } else {
            Ok(Quantity {
                instrument,
                size,
                path_id
            })
        }
    }

    pub fn clone(&self) -> Quantity {
        Quantity { 
            instrument: self.instrument.clone(),
            size: self.size.clone(),
            path_id: self.path_id.clone()
        }
    }

    fn is_locked(&self) -> bool {
        self.path_id != ""
    }

    fn lock_for(self, path_id: String) -> Quantity {
        Quantity::new(self.instrument, self.size, path_id).unwrap()
    }

    fn convert(self, exchange_pair: ExchangePair) -> Quantity {
        if self.instrument == exchange_pair.pair.base {
            Quantity {
                instrument: exchange_pair.pair.base.clone(), 
                size: self.size / exchange_pair.price(), 
                path_id: self.path_id
            }
        } else {
            Quantity {
                instrument: exchange_pair.pair.base.clone(), 
                size: self.size * exchange_pair.price(), 
                path_id: self.path_id
            }
        }
    }

    fn free(self) -> Quantity {
        Quantity {
            instrument: self.instrument, 
            size: self.size, 
            path_id: "".to_string()
        }
    }

    fn validate(left: Quantity, right: Quantity) -> Result<(Quantity, Quantity), TensorTradeError> {
        if left.instrument != right.instrument {
            return Err(TensorTradeError::IncompatibleInstrumentOperation{});
        }

        if left.path_id != "" && right.path_id != "" && left.path_id != right.path_id {
            return Err(TensorTradeError::QuantityOpPathMismatch{});
        }

        if left.path_id != "" && right.path_id == "" {
            return Ok((
                Quantity { instrument: left.instrument, size: left.size, path_id: left.path_id.clone() },
                Quantity { instrument: right.instrument, size: right.size, path_id: left.path_id }
            ))
        }

        if left.path_id == "" && right.path_id != "" {
            return Ok((
                Quantity { instrument: left.instrument, size: left.size, path_id: right.path_id.clone() },
                Quantity { instrument: right.instrument, size: right.size, path_id: right.path_id }
            ))
        }

        Ok((left, right))
    }

    pub fn quantize(self) -> Quantity {
        let precision = self.instrument.precision.clone();

        Quantity {
            instrument: self.instrument,
            size: self.size.round_dp(precision),
            path_id: self.path_id
        }
    }
}

impl ops::Add<Quantity> for Quantity {
    type Output = Result<Quantity, TensorTradeError>;

    fn add(self, other: Quantity) -> Result<Quantity, TensorTradeError> {
        let (left, right) = Quantity::validate(self, other)?;
        
        Quantity::new(left.instrument, left.size + right.size, left.path_id)
    }
}

impl ops::Sub<Quantity> for Quantity {
    type Output = Result<Quantity, TensorTradeError>;

    fn sub(self, other: Quantity) -> Result<Quantity, TensorTradeError> {
        let (left, right) = Quantity::validate(self, other)?;
        
        Quantity::new(left.instrument, left.size - right.size, left.path_id)
    }
}