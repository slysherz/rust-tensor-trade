mod ttcore;
mod order;
mod order_listener;

use order::Order;

struct Broker {
    unexecuted: vec<Order>,
    executed: vec<Order>,
    trades: HashMap<String, None>
}

impl Broker {
    fn new() {
        Broker{}
    }

    /// Submits an order to the broker
    fn submit(&self, order: Order) {
        self.unexecuted.push(Order)
    }

    /// Cancels an order
    fn cancel(&self, order: Order) -> Result<> {
        if order.status == OrderStatus::Cancelled {
            return;
            // todo: Warn cancelled twice
        }

        match self.unexecuted.find(|&o| ttcore::same_object(order, o)) {
            Some(i) =>  self.unexecuted.remove(i),
            None => false
        }
    }
}