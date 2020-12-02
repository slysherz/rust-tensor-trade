use super::order::{CriteriaLike, Order};
use super::trade::{TradeSide, TradeType};
use crate::oms::instruments::ExchangePair;
use crate::ttcore::base::TimeIndexed;

pub struct OrderSpec {
    id: String,
    side: TradeSide,
    trade_type: TradeType,
    exchange_pair: ExchangePair,
    criteria: Box<dyn CriteriaLike>,
}

impl OrderSpec {
    pub fn create_order(self, order: Order) -> Option<Order> {
        let price = self.exchange_pair.price();
        let wallet_instrument = self.side.instrument(&self.exchange_pair.pair);
        let exchange = &self.exchange_pair.exchange;
        let wallet = order.portfolio.get_wallet(&exchange.id, wallet_instrument);

        match None {
            // wallet.locked.get(order.path_id, None) {
            None => return None,
            Some(quantity) => Order::new(
                exchange.clock().step,
                self.side,
                self.trade_type,
                self.exchange_pair,
                quantity,
                order.portfolio,
                price,
                self.criteria,
                order.path_id,
                None,
                order.end,
            ),
        }
    }
}
