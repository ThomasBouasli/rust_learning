use crate::patterns::factory::transport::{Transport, truck::Truck};
use super::Logistics;


pub struct RoadLogistics;

impl Logistics for RoadLogistics {
    fn create_transport(&self) -> Box<dyn Transport> {
        Box::new(Truck{id: rand::random::<u32>()})
    }
}