use super::{
    order::{CriteriaLike, Order},
    trade::{TradeSide, TradeType},
    OrderParams,
};
use crate::ttcore::base::TimeIndexed;
use crate::{
    oms::{instruments::ExchangePair, wallets::WalletLike},
    ttcore::errors::TensorTradeError,
};

pub struct OrderSpec {
    id: String,
    side: TradeSide,
    trade_type: TradeType,
    exchange_pair: ExchangePair,
    criteria: Box<dyn CriteriaLike>,
}

impl OrderSpec {
    pub fn create_order(self, order: Order) -> Result<Option<Order>, TensorTradeError> {
        let price = self.exchange_pair.price()?;
        let wallet_instrument = self.side.instrument(&self.exchange_pair.pair);
        let exchange = &self.exchange_pair.exchange;
        let wallet = match order.portfolio.get_wallet(&exchange.id, wallet_instrument) {
            Some(wallet) => wallet,
            None => return Ok(None),
        };

        let quantity = wallet
            .locked(&order.path_id)
            .ok_or(TensorTradeError::QuantityNotLocked {})?;

        Order::new(OrderParams {
            step: exchange.clock().step,
            side: self.side,
            trade_type: self.trade_type,
            exchange_pair: self.exchange_pair,
            quantity,
            portfolio: order.portfolio,
            price,
            criteria: Some(self.criteria),
            path_id: Some(order.path_id),
            start: None,
            end: order.end,
        })
        .map(|order| Some(order))
    }
}
