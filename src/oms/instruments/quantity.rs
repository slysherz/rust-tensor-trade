use rust_decimal::prelude::*;

use crate::ttcore::errors::TensorTradeError;
use crate::oms::instruments::exchange_pair::ExchangePair;
use crate::oms::orders::instruments::Instrument;

pub struct Quantity {
    instrument: Instrument,
    size: Decimal,
    path_id: String
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


}