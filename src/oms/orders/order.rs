use crate::{
    oms::{
        exchanges::Exchange,
        instruments::TradingPair,
        instruments::{ExchangePair, Quantity},
        wallets::Portfolio,
        wallets::WalletLike,
    },
    ttcore::{
        base::{generate_id, TimeIndexed},
        clock::Clock,
        decimal::Decimal,
        errors::TensorTradeError,
    },
};

use super::{trade::Trade, TradeSide, TradeType};

pub trait CriteriaLike {
    fn check(&self, _: Order, _: Exchange) -> bool;
}

pub trait OrderListener: std::fmt::Debug {
    fn on_execute(&self, _: &Order) {}
    fn on_cancel(&self, _: &Order) {}
    fn on_fill(&self, _: &Order, _: &Trade) {}
    fn on_complete(&self, _: &Order) {}
}

#[derive(Debug)]
struct DefaultOrderListener {}

impl OrderListener for DefaultOrderListener {
    fn on_execute(&self, _: &Order) {}
    fn on_cancel(&self, _: &Order) {}
    fn on_fill(&self, _: &Order, _: &Trade) {}
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

pub struct OrderParams {
    pub step: i32,
    pub side: TradeSide,
    pub trade_type: TradeType,
    pub exchange_pair: ExchangePair,
    pub quantity: Quantity,
    pub portfolio: Portfolio,
    pub price: Decimal,
    pub criteria: Option<Box<dyn CriteriaLike>>,
    pub path_id: Option<String>,
    pub start: Option<i32>,
    pub end: Option<i32>,
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
    pub criteria: Option<Box<dyn CriteriaLike>>,
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
    pub fn new(params: OrderParams) -> Result<Order, TensorTradeError> {
        let path_id = params.path_id.unwrap_or(generate_id());

        // We use the block to prove to the borrow checker that we only need to borrow portfolio to
        // calculate quantity
        let quantity = {
            let wallet = params
                .portfolio
                .get_wallet(
                    &params.exchange_pair.exchange.id,
                    params.side.instrument(&params.exchange_pair.pair),
                )
                .ok_or(TensorTradeError::WalletNotFound {})?;

            if wallet.is_locked(&path_id) {
                params.quantity
            } else {
                params.quantity
                // todo: get ledger in here
                // order.quantity = wallet.lock(order.quantity, path_id, "LOCK FOR ORDER".to_string(), ledger)
            }
        };

        let order = Order {
            created_at: Clock::now(),
            step: params.step,
            side: params.side,
            trade_type: params.trade_type,
            exchange_pair: params.exchange_pair,
            quantity: quantity.clone(),
            remaining: quantity,
            portfolio: params.portfolio,
            price: params.price,
            criteria: params.criteria,
            path_id: path_id,
            start: params.start.unwrap_or(params.step.clone()),
            end: params.end,
            status: OrderStatus::Pending,
            specs: Vec::new(),
            trades: Vec::new(),
            listeners: Vec::new(),
        };

        Ok(order)
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
