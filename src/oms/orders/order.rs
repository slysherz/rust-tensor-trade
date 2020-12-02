use crate::oms::instruments::{ExchangePair, Quantity, TradingPair};
use crate::oms::orders::trade::{TradeSide, TradeType};
use crate::oms::wallets::Portfolio;
use crate::oms::{exchanges::Exchange, wallets::WalletLike};
use crate::ttcore::{base::TimeIndexed, clock::Clock, decimal::Decimal, errors::TensorTradeError};

use super::trade::Trade;

pub trait CriteriaLike {
    fn check(&self, _: Order, _: Exchange) -> bool;
}

pub trait OrderListener: std::fmt::Debug {
    fn on_execute(&self, order: &Order) {}
    fn on_cancel(&self, order: &Order) {}
    fn on_fill(&self, order: &Order, trade: &Trade) {}
    fn on_complete(&self, order: &Order) {}
}

#[derive(Debug)]
struct DefaultOrderListener {}

impl OrderListener for DefaultOrderListener {
    fn on_execute(&self, _: &Order) {}
    fn on_cancel(&self, _: &Order) {}
    fn on_fill(&self, _: &Order, trade: &Trade) {}
    fn on_complete(&self, _: &Order) {}
}

pub fn default_order_listener() -> Box<dyn OrderListener> {
    Box::new(DefaultOrderListener {})
}

#[derive(PartialEq)]
pub enum OrderStatus {
    Pending,
    Open,
    Cancelled,
    PartiallyFilled,
    Filled,
}

pub struct Order {
    pub created_at: i32,
    pub step: i32,
    pub side: TradeSide,
    pub trade_type: TradeType,
    pub exchange_pair: ExchangePair,
    pub quantity: Quantity,
    pub remaining: Quantity,
    pub portfolio: Portfolio,
    pub price: Decimal,
    pub criteria: Box<dyn CriteriaLike>,
    pub path_id: String,
    pub start: i32,
    pub end: Option<i32>,
    pub status: OrderStatus,
    specs: Vec<i32>,
    trades: Vec<Trade>,
    listeners: Vec<Box<dyn OrderListener>>,
}

impl TimeIndexed for Order {}

impl Order {
    pub fn new(
        step: i32,
        side: TradeSide,
        trade_type: TradeType,
        exchange_pair: ExchangePair,
        quantity: Quantity,
        portfolio: Portfolio,
        price: Decimal,
        criteria: Box<dyn CriteriaLike>,
        path_id: String,
        start: Option<i32>,
        end: Option<i32>,
    ) -> Option<Order> {
        let wallet = portfolio.get_wallet(
            &exchange_pair.exchange.id,
            side.instrument(&exchange_pair.pair),
        )?;

        let order = Order {
            created_at: Clock::now(),
            step,
            side,
            trade_type,
            exchange_pair,
            quantity: quantity.clone(),
            remaining: quantity,
            portfolio,
            price,
            criteria,
            path_id,
            start: start.unwrap_or(step.clone()),
            end,
            status: OrderStatus::Pending,
            specs: Vec::new(),
            trades: Vec::new(),
            listeners: Vec::new(),
        };

        /*
        let quantity = if wallet.is_locked(&order.path_id) {
            quantity
        } else {
            // wallet.lock(quantity, order, "LOCK FOR ORDER".to_string(), ledger)
        };

        order.quantity = quantity;
        */

        Some(order)
    }

    pub fn pair<'a>(&'a self) -> &'a TradingPair {
        &self.exchange_pair.pair
    }

    pub fn fill(&mut self, trade: Trade) -> Result<(), TensorTradeError> {
        self.status = OrderStatus::PartiallyFilled;
        let filled = (&trade.quantity + &trade.commission)?;

        self.remaining = (&self.remaining - &filled)?;
        self.trades.push(trade);

        /*
        for listener in self.listeners {
            listener.on_fill(self, trade)
        }
        */

        Ok(())
    }
}
