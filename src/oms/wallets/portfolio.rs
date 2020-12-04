use std::{collections::HashMap, rc::Rc};

use crate::{
    oms::{
        instruments::{Instrument, Quantity},
        orders::{default_order_listener, OrderListener},
    },
    ttcore::{clock::Clock, decimal::Decimal, errors::TensorTradeError},
};

use super::{Wallet, WalletTuple};

/// Auxiliary struct in Portfolio. We use it so that we can perform some operations with wallets
/// before the entire portfolio is built
pub struct Wallets(HashMap<(String, String), Wallet>);

impl Wallets {
    fn new() -> Wallets {
        Wallets(HashMap::new())
    }

    fn balance(&self, instrument: &Instrument) -> Result<Quantity, TensorTradeError> {
        let mut balance = Quantity::new(instrument.clone(), Decimal::new(0, 0), "".to_string())?;

        for ((_, symbol), wallet) in &self.0 {
            if symbol == &instrument.symbol {
                balance = (&balance + &wallet.balance)?;
            }
        }

        Ok(balance)
    }

    fn add(&mut self, wallet: WalletTuple) -> Result<(), TensorTradeError> {
        let (exchange, instrument, _) = &wallet;
        let key = (exchange.id.clone(), instrument.symbol.clone());

        self.0.insert(key, Wallet::from_tuple(wallet)?);

        Ok(())
    }

    fn remove(&mut self, wallet: WalletTuple) {
        let (exchange, instrument, _) = wallet;

        self.0.remove(&(exchange.id, instrument.symbol));
    }
}

pub trait PerformanceListener: std::fmt::Debug {}

#[derive(Debug)]
struct DefaultPerformanceListener {}

impl PerformanceListener for DefaultPerformanceListener {}

pub fn default_performance_listener() -> Box<dyn PerformanceListener> {
    Box::new(DefaultPerformanceListener {})
}

pub struct Portfolio {
    pub clock: Rc<Clock>,
    pub base_instrument: Instrument,
    order_listener: Box<dyn OrderListener>,
    performance_listener: Box<dyn PerformanceListener>,
    pub wallets: Wallets,
    pub initial_balance: Quantity,
    // initial_net_worth:,
    // net_worth
    // performance
    // keys
}

impl Portfolio {
    pub fn new(
        base_instrument: Instrument,
        wallets: Vec<WalletTuple>,
        order_listener: Option<Box<dyn OrderListener>>,
        performance_listener: Option<Box<dyn PerformanceListener>>,
    ) -> Result<Portfolio, TensorTradeError> {
        let mut wallets_s = Wallets::new();

        for wallet in wallets {
            wallets_s.add(wallet)?;
        }

        let initial_balance = wallets_s.balance(&base_instrument)?;

        let result = Portfolio {
            clock: Rc::new(Clock::new()), // todo: figure out to pass a clock here
            base_instrument: base_instrument.clone(),
            order_listener: order_listener.unwrap_or(default_order_listener()),
            performance_listener: performance_listener.unwrap_or(default_performance_listener()),
            wallets: wallets_s,
            initial_balance,
        };

        Ok(result)
    }

    pub fn get_wallet<'a>(
        &'a self,
        exchange_id: &String,
        instrument: &Instrument,
    ) -> Option<&'a Wallet> {
        let key = (exchange_id.clone(), instrument.symbol.clone());
        self.wallets.0.get(&key)
    }
}
