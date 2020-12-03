use std::boxed::Box;

use crate::{
    oms::exchanges::Exchange,
    ttcore::{decimal::decimal_from_f32, errors::TensorTradeError},
};

use super::{Order, TradeSide};
#[derive(PartialEq)]
pub enum StopDirection {
    Up,
    Down,
}

pub enum Criteria {
    Check {
        condition: fn(&Order, &Exchange) -> bool,
    },
    BinOp {
        operator: String,
        left: Box<Criteria>,
        right: Box<Criteria>,
    },
    Not {
        criteria: Box<Criteria>,
    },
    Limit {
        limit_price: f32,
    },
    Stop {
        direction: StopDirection,
        percent: f32,
    },
    Timed {
        duration: i32,
    },
}

impl Criteria {
    pub fn check(&self, order: &Order, exchange: &Exchange) -> Result<bool, TensorTradeError> {
        let is_valid = match self {
            Criteria::Check { condition } => condition(order, exchange),
            Criteria::BinOp {
                operator,
                left,
                right,
            } => Criteria::check_bin_op(
                operator,
                left.check(order, exchange)?,
                right.check(order, exchange)?,
            ),
            Criteria::Not { criteria } => !criteria.check(order, exchange)?,
            Criteria::Limit { limit_price } => {
                let lprice = decimal_from_f32(limit_price.clone())?;
                let price = exchange.quote_price(order.pair())?;
                let buy_satisfied = order.side == TradeSide::Buy && price <= lprice;
                let sell_satisfied = order.side == TradeSide::Sell && price >= lprice;

                buy_satisfied || sell_satisfied
            }
            Criteria::Stop { direction, percent } => {
                let goal_percent = decimal_from_f32(percent.clone())?;
                let order_price = order.price;
                let price = exchange.quote_price(&order.pair())?;
                let order_percent = (price - order_price).abs();

                let is_take_profit = (direction == &StopDirection::Up) && (price >= order_price);
                let is_stop_loss = (direction == &StopDirection::Down) && (price <= order_price);

                (is_take_profit || is_stop_loss) && order_percent > goal_percent
            }
            Criteria::Timed { duration } => (order.step - order.created_at) <= duration.clone(),
        };

        Ok(is_valid)
    }

    pub fn check_bin_op(operator: &String, left: bool, right: bool) -> bool {
        match operator.as_str() {
            "&&" => left && right,
            "||" => left || right,
            _ => false, // todo: what to do in this case?
        }
    }
}
