use crate::{
    oms::{
        instruments::{ExchangePair, Quantity},
        wallets::Portfolio,
    },
    ttcore::{decimal::decimal_from_f32, errors::TensorTradeError},
};

use super::{Order, OrderParams, TradeSide, TradeType};

fn market_order(
    side: TradeSide,
    exchange_pair: ExchangePair,
    price: f32,
    size: f32,
    portfolio: Portfolio,
) -> Result<Order, TensorTradeError> {
    let instrument = side.instrument(&exchange_pair.pair);
    let quantity = Quantity::new(instrument.clone(), decimal_from_f32(size)?, "".to_string())?;
    let price = decimal_from_f32(price)?;

    Order::new(OrderParams {
        step: portfolio.clock.step,
        side,
        trade_type: TradeType::Market,
        exchange_pair,
        price,
        quantity,
        portfolio,
        criteria: None,
        path_id: None,
        start: None,
        end: None,
    })
}
