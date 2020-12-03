use rust_tensortrade::oms::exchanges::{Exchange, ExchangeOptions, ServiceLike};
use rust_tensortrade::oms::instruments::Instrument;
use rust_tensortrade::oms::orders::{Order, Trade};
use rust_tensortrade::oms::wallets::{Portfolio, Wallet};
use rust_tensortrade::ttcore::clock::Clock;
use rust_tensortrade::ttcore::decimal::Decimal;
use rust_tensortrade::ttcore::errors::TensorTradeError;

#[test]
fn test_portfolio() -> Result<(), TensorTradeError> {
    #[derive(Debug)]
    struct Service {}

    impl ServiceLike for Service {
        fn execute_order(
            &self,
            _: &Order,
            _: &Wallet,
            _: &Wallet,
            _: Decimal,
            _: &ExchangeOptions,
            _: &Clock,
        ) -> Option<Trade> {
            None
        }
    }

    let coinbase1 = Exchange::new(
        "coinbase".to_string(),
        Box::new(Service {}),
        ExchangeOptions::new(),
    );
    let coinbase2 = Exchange::new(
        "coinbase".to_string(),
        Box::new(Service {}),
        ExchangeOptions::new(),
    );

    let wallets = Vec::from([
        (coinbase1, Instrument::usd(), 1000.0),
        (coinbase2, Instrument::usd(), 10.0),
    ]);

    let portfolio = Portfolio::new(Instrument::usd(), wallets, None, None);

    println!("{:#?}", portfolio);

    Ok(())
}
