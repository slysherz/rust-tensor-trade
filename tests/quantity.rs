use rust_tensortrade::oms::instruments::{Quantity};
use rust_tensortrade::ttcore::{
    decimal::Decimal,
    errors::TensorTradeError
};
use rust_tensortrade::oms::instruments::Instrument;
#[test]
fn test_quantity() -> Result<(), TensorTradeError> {
    let a = Quantity::new(
        Instrument::jpy(),
        Decimal::new(123, 2),
        "somepath".to_string()
    )?;

    let b = Quantity::new(
        Instrument::jpy(),
        Decimal::new(111, 2),
        "somepath".to_string()
    )?;

    println!("{:?}", a + b);
    Ok(())
}