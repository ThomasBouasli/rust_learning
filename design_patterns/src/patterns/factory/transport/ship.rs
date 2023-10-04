use super::Transport;

pub struct Ship{
    pub id: u32,
}

impl Transport for Ship {
    fn start_delivery(&self, delivery: &mut crate::patterns::factory::delivery::Delivery) {
        delivery.ship();
        println!("Starting delivery by ship with id {} of {} to {}", self.id, delivery.address, delivery.name);
    }

    fn end_delivery(&self, delivery: &mut crate::patterns::factory::delivery::Delivery) {
        delivery.deliver();
        println!("Ending delivery by ship with id {} of {} to {}", self.id, delivery.address, delivery.name);
    }
}