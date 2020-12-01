use crate::oms::orders::trade::{TradeSide, TradeType};
use crate::oms::exchanges::Exchange;
use crate::oms::instruments::{
    TradingPair,
    Quantity, 
    ExchangePair
};
use crate::ttcore::{
    errors::TensorTradeError,
    clock::Clock,
    base::TimeIndexed,
    decimal::Decimal
};

use super::trade::Trade;

pub trait CriteriaLike {
    fn check(&self, _: Order, _: Exchange) -> bool;
}

pub trait OrderListener {
    fn on_execute(&self, order: &Order) {}
    fn on_cancel(&self, order: &Order) {}
    fn on_fill(&self, order: &Order, trade: &Trade) {}
    fn on_complete(&self, order: &Order) {}
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
    pub remaining: Quantity,
    // portfolio: Portfolio,
    pub price: Decimal,
    pub criteria: Box<dyn CriteriaLike>,
    pub path_id: String,
    pub start: i32,
    pub end: Option<i32>,
    pub status: OrderStatus,
    specs: Vec<i32>,
    trades: Vec<Trade>,
    listeners: Vec<Box<dyn OrderListener>>
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
        );
        
        if self.path_id not in wallet.locked.keys():
            self.quantity = wallet.lock(quantity, self, "LOCK FOR ORDER")
        */

        Some(Order {
            created_at: Clock::now(),
            step,
            side,
            trade_type,
            exchange_pair,
            quantity: quantity.clone(),
            remaining: quantity,
            // portfolio,
            price,
            criteria,
            path_id,
            start: start.unwrap_or(step.clone()),
            end,
            status: OrderStatus::Pending,
            specs: Vec::new(),
            trades: Vec::new(),
            listeners: Vec::new()
        })

    }

    pub fn pair<'a>(&'a self) -> &'a TradingPair {
        &self.exchange_pair.pair
    }

    pub fn fill(&mut self, trade: Trade) -> Result<(), TensorTradeError> {
        self.status = OrderStatus::PartiallyFilled;
        let filled = (trade.quantity.clone() + trade.commission.clone())?;

        self.remaining = (self.remaining.clone() - filled)?;
        self.trades.push(trade);

        /*
        for listener in self.listeners {
            listener.on_fill(self, trade)
        }
        */

        Ok(())
    }
}