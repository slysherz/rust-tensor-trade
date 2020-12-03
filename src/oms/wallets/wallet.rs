use crate::oms::exchanges::Exchange;
use crate::oms::instruments::{Instrument, Quantity};
use crate::oms::wallets::{Ledger, WalletLike};
use crate::ttcore::decimal::{decimal_from_f32, Decimal};
use crate::ttcore::{base::TimeIndexed, errors::TensorTradeError};
use std::collections::HashMap;

pub type WalletTuple = (Exchange, Instrument, f32);

#[derive(Debug)]
pub struct Wallet {
    exchange: Exchange,
    initial_size: Decimal,
    instrument: Instrument,
    pub balance: Quantity,
    locked: HashMap<String, Decimal>,
}

impl Wallet {
    pub fn new(exchange: Exchange, balance: Quantity) -> Wallet {
        Wallet {
            exchange,
            initial_size: balance.size,
            instrument: balance.instrument.clone(),
            balance: balance.quantize(),
            locked: HashMap::new(),
        }
    }

    pub fn from_tuple(
        (exchange, instrument, value): WalletTuple,
    ) -> Result<Wallet, TensorTradeError> {
        Ok(Wallet::new(
            exchange,
            Quantity {
                instrument: instrument,
                size: decimal_from_f32(value)?,
                path_id: "".to_string(),
            },
        ))
    }

    pub fn lock(
        &mut self,
        quantity: Quantity,
        path_id: String,
        reason: String,
        ledger: &mut Ledger,
    ) -> Result<(), TensorTradeError> {
        if quantity.is_locked() {
            return Err(TensorTradeError::DoubleLockedQuantity {});
        }

        if quantity > self.balance {
            return Err(TensorTradeError::InsufficientFunds {});
        }

        self.balance = (&self.balance - &quantity)?;

        let quantity = quantity.lock_for(path_id);

        let new_value = match self.locked.get(&quantity.path_id) {
            Some(value) => value + quantity.size,
            None => quantity.size,
        };

        self.locked.insert(quantity.path_id.clone(), new_value);
        self.balance = self.balance.quantize();

        ledger.commit(
            Box::new(self),
            quantity,
            format!("{}:{}/free", self.exchange.name, self.instrument),
            format!("{}:{}/locked", self.exchange.name, self.instrument),
            format!("LOCK ({})", reason),
        );

        Ok(())
    }
}

impl WalletLike for Wallet {
    fn balance<'a>(&'a self) -> &'a Quantity {
        &self.balance
    }

    fn locked_balance(&self) -> Quantity {
        // todo: confirm that all quantities have the same resource
        let mut locked_balance = Decimal::new(0, 0);

        for (_, quantity) in &self.locked {
            locked_balance = locked_balance + quantity;
        }

        Quantity::new(self.instrument.clone(), locked_balance, "".to_string()).unwrap()
    }

    fn locked(&self, key: &String) -> Option<Quantity> {
        let size = self.locked.get(key)?;

        Some(Quantity {
            instrument: self.instrument.clone(),
            size: size.clone(),
            path_id: key.clone(),
        })
    }

    fn is_locked(&self, key: &String) -> bool {
        self.locked.contains_key(key)
    }

    fn step(&self) -> i32 {
        self.exchange.clock().step
    }
}
