use std::boxed::Box;
use crate::oms::orders::order::Order;
use crate::oms::orders::trade::*;
use crate::oms::exchanges::exchange::Exchange;
pub enum StopDirection {
    Up,
    Down
}

pub enum Criteria {
    Check { condition: fn(&Order, &Exchange) -> bool },
    BinOp { operator: String, left: Box<Criteria>, right: Box<Criteria> },
    Not { criteria: Box<Criteria> },
    Limit { limit_price: f32 },
    Stop { direction: StopDirection, percent: f32 },
    Timed { duration: f32 }
}

impl Criteria {
    pub fn check(&self, order: &Order, exchange: &Exchange) -> bool {
        match self {
            Criteria::Check { condition } => condition(order, exchange),
            Criteria::BinOp { operator, left, right } => {
                Criteria::check_bin_op(operator, left.check(order, exchange), right.check(order, exchange))
            },
            Criteria::Not { criteria } => !criteria.check(order, exchange),
            /*Criteria::Limit { limit_price } => {
                let price = exchange.quote_price(order.pair);
                let buy_satisfied = (order.side == TradeSide::Buy && price <= limit_price);
                let sell_satisfied = (order.side == TradeSide::Sell && price >= limit_price);

                buy_satisfied || sell_satisfied
            },
            Criteria::Stop { direction, percent } => {

            },
            Criteria::Timed { duration } => (order.clock.step - order.created_at) <= duration
            */
            _ => false
        }
    }

    pub fn check_bin_op(
        operator: &String, left: bool, right: bool) -> bool {
        match operator.as_str() {
            "&&" => left && right,
            "||" => left || right,
            _ => false  // todo: what to do in this case?
        }
    }
}
