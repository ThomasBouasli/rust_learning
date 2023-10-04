use crate::patterns::factory::transport::{Transport, ship::Ship};
use super::Logistics;


pub struct SeaLogistics;

impl Logistics for SeaLogistics {
    fn create_transport(&self) -> Box<dyn Transport> {
        Box::new(Ship{id: rand::random::<u32>()})
    }
}