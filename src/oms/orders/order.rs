use crate::oms::{instruments::trading_pair::TradingPair, orders::trade::{TradeSide, TradeType}};
use crate::oms::instruments::quantity::Quantity;
use crate::oms::instruments::exchange_pair::ExchangePair;
use crate::oms::exchanges::exchange::Exchange;
use crate::ttcore::clock::Clock;
use crate::ttcore::base::TimeIndexed;
use rust_decimal::prelude::*;

pub trait CriteriaLike {
    fn check(&self, _: Order, _: Exchange) -> bool;
}

pub enum OrderStatus {
    Pending,
    Open,
    Cancelled,
    PartiallyFilled,
    Filled
}

pub struct Order {
    pub created_at: i32,
    pub step: i32,
    pub side: TradeSide,
    pub trade_type: TradeType,
    pub exchange_pair: ExchangePair,
    pub quantity: Quantity,
    // portfolio: Portfolio,
    pub price: Decimal,
    pub criteria: Box<dyn CriteriaLike>,
    pub path_id: String,
    pub start: i32,
    pub end: Option<i32>,
    specs: Vec<i32>,
    trades: Vec<i32>
}

impl TimeIndexed for Order {}

impl Order {
    pub fn new(
        step: i32,
        side: TradeSide,
        trade_type: TradeType,
        exchange_pair: ExchangePair,
        quantity: Quantity,
        // portfolio: Portfolio,
        price: Decimal,
        criteria: Box<dyn CriteriaLike>,
        path_id: String,
        start: Option<i32>,
        end: Option<i32>
    ) -> Option<Order> {
        /*wallet = portfolio.get_wallet(
            exchange_pair.exchange.id,
            side.instrument(exchange_pair.pair)
        );*/

        Some(Order {
            created_at: Clock::now(),
            step,
            side,
            trade_type,
            exchange_pair,
            quantity,
            // portfolio,
            price,
            criteria,
            path_id,
            start: start.unwrap_or(step.clone()),
            end,
            specs: Vec::new(),
            trades: Vec::new()
        })

    }

    pub fn pair<'a>(&'a self) -> &'a TradingPair {
        &self.exchange_pair.pair
    }
}