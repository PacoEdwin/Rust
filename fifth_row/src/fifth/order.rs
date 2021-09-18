#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Order {
    pub id: usize,
    pub order_type: OrderType,
    pub price: u32,
    pub quantity: u32,
}
