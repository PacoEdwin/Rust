use fehler::{throw, throws};
use std::{cmp::Ordering, collections::BinaryHeap, fmt};

use crate::fifth::{
    errors::{self, Error},
    order::{Order, OrderType},
};

struct Trade {
    buy_id: usize,
    sell_id: usize,
    price: u32, // this should be the sell price.
    quantity_traded: u32,
}

impl fmt::Display for Trade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Trade {} BTC @ {} USD between {} and {}",
            self.quantity_traded, self.price, self.buy_id, self.sell_id
        )
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> Ordering {
        other.price.cmp(&self.price)
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Accountant {
    heap: BinaryHeap<Order>,
}

impl Accountant {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    #[throws]
    pub fn add(self: &mut Self, order: Order) {
        match order.order_type {
            OrderType::Sell => {}
            order_type => throw!(errors::UnexpectedOrderType { order_type }.build()),
        }

        self.heap.push(order);
    }

    pub fn process_order(self: &mut Self, order: Order) {
        match order.order_type {
            OrderType::Sell => self.add(order).unwrap(),
            OrderType::Buy => self.match_orders(order),
        }
    }

    fn match_orders(self: &mut Self, mut order: Order) {
        let heap = &mut self.heap;
        while !heap.is_empty() {
            {
                let mut sell_order = heap.peek_mut().unwrap();

                if sell_order.price > order.price || order.quantity == 0 {
                    break;
                }

                let min = std::cmp::min(sell_order.quantity, order.quantity);
                sell_order.quantity -= min;
                order.quantity -= min;

                println!(
                    "{}",
                    Trade {
                        sell_id: sell_order.id,
                        buy_id: order.id,
                        price: sell_order.price,
                        quantity_traded: min
                    }
                );
            }

            if heap.peek().unwrap().quantity == 0 {
                heap.pop();
            }
        }
    }
}
