use std::collections::HashMap;

use crate::ttcore::{base::same_object, errors::TensorTradeError};

use super::{Order, OrderStatus, Trade};

struct Broker {
    unexecuted: Vec<Order>,
    executed: Vec<Order>,
    trades: HashMap<String, Trade>,
}

impl Broker {
    fn new() -> Broker {
        Broker {
            unexecuted: Vec::new(),
            executed: Vec::new(),
            trades: HashMap::new(),
        }
    }

    /// Submits an order to the broker
    fn submit(&mut self, order: Order) {
        self.unexecuted.push(order)
    }

    /// Cancels an order
    fn cancel(&mut self, order: Order) -> Result<(), TensorTradeError> {
        if order.status == OrderStatus::Cancelled {
            return Err(TensorTradeError::DoubleCancelOrder {});
        }

        match self.unexecuted.iter().position(|o| same_object(&order, &o)) {
            Some(i) => {
                self.unexecuted.remove(i);
            }
            None => {}
        };

        Ok(())
    }
}
