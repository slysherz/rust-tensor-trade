use super::errors::TensorTradeError;
use rust_decimal::prelude::FromPrimitive;
pub use rust_decimal::prelude::{Decimal, ToPrimitive};

pub fn decimal_from_f32(value: f32) -> Result<Decimal, TensorTradeError> {
    Decimal::from_f32(value).ok_or(TensorTradeError::InvalidNonNumericQuantity { value })
}
