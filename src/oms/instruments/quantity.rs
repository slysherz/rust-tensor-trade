use crate::oms::instruments::{ExchangePair, Instrument};
use crate::ttcore::decimal::{Decimal, ToPrimitive};
use crate::ttcore::errors::TensorTradeError;
use std::cmp::Ordering;
use std::ops;

#[derive(Debug)]
pub struct Quantity {
    pub instrument: Instrument,
    pub size: Decimal,
    pub path_id: String,
}

impl Quantity {
    pub fn new(
        instrument: Instrument,
        size: Decimal,
        path_id: String,
    ) -> Result<Quantity, TensorTradeError> {
        if size < Decimal::new(0, 0) {
            let fsize = match size.to_f32() {
                Some(f) => f,
                None => 0.0,
            };

            Err(TensorTradeError::InvalidNegativeQuantity { size: fsize })
        } else {
            Ok(Quantity {
                instrument,
                size,
                path_id,
            })
        }
    }

    pub fn clone(&self) -> Quantity {
        Quantity {
            instrument: self.instrument.clone(),
            size: self.size.clone(),
            path_id: self.path_id.clone(),
        }
    }

    pub fn is_locked(&self) -> bool {
        self.path_id != ""
    }

    pub fn lock_for(self, path_id: String) -> Quantity {
        Quantity::new(self.instrument, self.size, path_id).unwrap()
    }

    pub fn convert(self, exchange_pair: ExchangePair) -> Result<Quantity, TensorTradeError> {
        let quantity = if self.instrument == exchange_pair.pair.base {
            Quantity {
                instrument: exchange_pair.pair.base.clone(),
                size: self.size / exchange_pair.price()?,
                path_id: self.path_id,
            }
        } else {
            Quantity {
                instrument: exchange_pair.pair.base.clone(),
                size: self.size * exchange_pair.price()?,
                path_id: self.path_id,
            }
        };

        Ok(quantity)
    }

    pub fn free(self) -> Quantity {
        Quantity {
            instrument: self.instrument,
            size: self.size,
            path_id: "".to_string(),
        }
    }

    pub fn validate(
        left: &Quantity,
        right: &Quantity,
    ) -> Result<(Quantity, Quantity), TensorTradeError> {
        if left.instrument != right.instrument {
            return Err(TensorTradeError::IncompatibleInstrumentOperation {});
        }

        if left.path_id != "" && right.path_id != "" && left.path_id != right.path_id {
            return Err(TensorTradeError::QuantityOpPathMismatch {});
        }

        if left.path_id != "" && right.path_id == "" {
            return Ok((
                Quantity {
                    instrument: left.instrument.clone(),
                    size: left.size,
                    path_id: left.path_id.clone(),
                },
                Quantity {
                    instrument: right.instrument.clone(),
                    size: right.size,
                    path_id: left.path_id.clone(),
                },
            ));
        }

        if left.path_id == "" && right.path_id != "" {
            return Ok((
                Quantity {
                    instrument: left.instrument.clone(),
                    size: left.size,
                    path_id: right.path_id.clone(),
                },
                Quantity {
                    instrument: right.instrument.clone(),
                    size: right.size,
                    path_id: right.path_id.clone(),
                },
            ));
        }

        Ok((left.clone(), right.clone()))
    }

    pub fn quantize(&self) -> Quantity {
        let precision = self.instrument.precision;

        Quantity {
            instrument: self.instrument.clone(),
            size: self.size.round_dp(precision),
            path_id: self.path_id.clone(),
        }
    }
}

impl ops::Add<&Quantity> for &Quantity {
    type Output = Result<Quantity, TensorTradeError>;

    fn add(self, other: &Quantity) -> Result<Quantity, TensorTradeError> {
        let (left, right) = Quantity::validate(self, other)?;

        Quantity::new(left.instrument, left.size + right.size, left.path_id)
    }
}

impl ops::Sub<&Quantity> for &Quantity {
    type Output = Result<Quantity, TensorTradeError>;

    fn sub(self, other: &Quantity) -> Result<Quantity, TensorTradeError> {
        let (left, right) = Quantity::validate(self, other)?;

        Quantity::new(left.instrument, left.size - right.size, left.path_id)
    }
}

impl PartialOrd for Quantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.size.partial_cmp(&other.size)
    }
}

impl PartialEq for Quantity {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}
