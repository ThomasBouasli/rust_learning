use super::delivery::Delivery;

pub mod ship;
pub mod truck;

pub trait Transport {
    fn start_delivery(&self, delivery: &mut Delivery);
    fn end_delivery(&self, delivery: &mut Delivery);
}