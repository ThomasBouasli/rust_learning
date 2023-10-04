use super::Transport;

pub struct Truck{
    pub id: u32,
}

impl Transport for Truck {
    fn start_delivery(&self, delivery: &mut crate::patterns::factory::delivery::Delivery) {
        delivery.ship();
        println!("Starting delivery by truck with id {} of {} to {}", self.id, delivery.address, delivery.name);
    }

    fn end_delivery(&self, delivery: &mut crate::patterns::factory::delivery::Delivery) {
        delivery.deliver();
        println!("Ending delivery by truck with id {} of {} to {}", self.id, delivery.address, delivery.name);
    }
}