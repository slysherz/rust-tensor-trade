use crate::ttcore::{base, errors::TensorTradeError, base::TimeIndexed};
use crate::oms::instruments::{ Instrument, Quantity, ExchangePair };
use crate::oms::orders::Order;
use crate::oms::exchanges::Exchange;
use crate::oms::wallets::{ Ledger, WalletLike };
use crate::ttcore::decimal::Decimal;
use std::collections::HashMap;

struct Wallet {
    exchange: Exchange,
    initial_size: Decimal,
    instrument: Instrument,
    balance: Quantity,
    locked: HashMap<String, Decimal>
}

impl Wallet {
    pub fn new(exchange: Exchange, balance: Quantity) -> Wallet {
        Wallet {
            exchange,
            initial_size: balance.size,
            instrument: balance.instrument.clone(),
            balance: balance.quantize(),
            locked: HashMap::new()
        }
    }
}

impl WalletLike for Wallet {
    fn balance<'a>(&'a self) -> &'a Quantity {
        &self.balance
    }

    fn locked_balance(&self) -> Quantity {
        let mut locked_balance = Decimal::new(0, 0);

        for (_, quantity) in &self.locked {
            locked_balance = locked_balance + quantity;
        }

        Quantity::new(self.instrument.clone(), locked_balance, "".to_string()).unwrap()
    }

    fn locked<'a>(&'a self, key: &String) -> Option<&'a Decimal> {
        self.locked.get(key)
    }

    fn is_locked(&self, key: &String) -> bool {
        self.locked.contains_key(key)
    }

    fn step(&self) -> i32 {
        self.exchange.clock().step
    }
}