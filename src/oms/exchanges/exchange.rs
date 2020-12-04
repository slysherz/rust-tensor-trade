use crate::{
    oms::wallets::Wallet,
    ttcore::{
        clock::Clock,
        decimal::{decimal_from_f32, Decimal},
        errors::TensorTradeError,
    },
};
use crate::{
    oms::{instruments::TradingPair, orders::Order},
    ttcore::base::generate_id,
};
use crate::{
    oms::{orders::Trade, wallets::Portfolio},
    ttcore::base::TimeIndexed,
};
use std::collections::HashMap;

pub trait StreamLike: Iterator {
    fn rename(&mut self, new_name: String);
    fn name(&self) -> String;
    fn value(&self) -> Option<Self::Item>;
}

pub trait ServiceLike: std::fmt::Debug {
    fn execute_order(
        &self,
        order: &Order,
        base_wallet: &Wallet,
        quote_wallet: &Wallet,
        current_price: Decimal,
        options: &ExchangeOptions,
        clock: &Clock,
    ) -> Option<Trade>;
}

#[derive(Debug)]
pub struct ExchangeOptions {
    /// The percentage of the order size taken by the exchange
    commission: f32,
    /// The minimum trade size an order can have
    min_trade_size: f32,
    /// The maximum trade size an order can have
    max_trade_size: f32,
    /// The minimum price an exchange can have
    min_trade_price: f32,
    /// The maximum price an exchange can have
    max_trade_price: f32,
    /// Whether live orders should be submitted to the exchange
    is_live: bool,
}

impl ExchangeOptions {
    pub fn new() -> ExchangeOptions {
        ExchangeOptions {
            commission: 0.0003,
            min_trade_size: 1e-6,
            max_trade_size: 1e6,
            min_trade_price: 1e-8,
            max_trade_price: 1e8,
            is_live: false,
        }
    }
}

pub struct Exchange {
    pub id: String,
    pub name: String,
    pub service: Box<dyn ServiceLike>,
    pub options: ExchangeOptions,
    pub price_streams: HashMap<String, Box<dyn StreamLike<Item = f32>>>,
    pub start: i32,
    pub step: i32,
}

impl TimeIndexed for Exchange {}

impl Exchange {
    pub fn new(
        name: String,
        service: Box<dyn ServiceLike>,
        options: ExchangeOptions,
        streams: Vec<Box<dyn StreamLike<Item = f32>>>,
    ) -> Exchange {
        let mut result = Exchange {
            id: generate_id(),
            name,
            service,
            options,
            price_streams: HashMap::new(),
            start: 0,
            step: 0,
        };

        for stream in streams {
            result.price_streams.insert(stream.name(), stream);
        }

        result
    }

    pub fn quote_price(&self, trading_pair: &TradingPair) -> Result<Decimal, TensorTradeError> {
        let stream_name = trading_pair.to_string();
        let value = self
            .price_streams
            .get(&stream_name)
            .ok_or(TensorTradeError::PriceNotFound {})?
            .value()
            .ok_or(TensorTradeError::PriceNotFound {})?;

        Ok(decimal_from_f32(value)?.round_dp(trading_pair.base.precision))
    }

    pub fn is_pair_tradable(&self, trading_pair: &TradingPair) -> bool {
        self.price_streams.contains_key(&trading_pair.to_string())
    }

    pub fn execute_order(
        &self,
        order: &mut Order,
        portfolio: &Portfolio,
    ) -> Result<(), TensorTradeError> {
        let base_wallet = portfolio
            .get_wallet(&self.id, &order.pair().base)
            .ok_or(TensorTradeError::WalletNotFound {})?;
        let quote_wallet = portfolio
            .get_wallet(&self.id, &order.pair().quote)
            .ok_or(TensorTradeError::WalletNotFound {})?;

        let price = self.quote_price(order.pair())?;

        let trade = self.service.execute_order(
            order,
            base_wallet,
            quote_wallet,
            price,
            &self.options,
            self.clock(),
        );

        trade.map(|t| order.fill(t));

        Ok(())
    }
}
