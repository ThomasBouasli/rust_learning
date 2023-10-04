pub mod logistics;
pub mod transport;
pub mod delivery;

#[cfg(test)]
mod tests {
    use crate::patterns::factory::logistics::{Logistics, road::RoadLogistics, sea::SeaLogistics};
    use crate::patterns::factory::delivery::{Delivery, DeliveryStatus};

    #[test]
    fn test_factory() {
        let mut delivery = Delivery::new("John", "123 Main St");
        let road_logistics = RoadLogistics;
        let sea_logistics = SeaLogistics;

        let truck = road_logistics.create_transport();
        let ship = sea_logistics.create_transport();

        assert_eq!(delivery.status, DeliveryStatus::Pending);

        truck.start_delivery(&mut delivery);

        assert_eq!(delivery.status, DeliveryStatus::Shipped);

        truck.end_delivery(&mut delivery);

        assert_eq!(delivery.status, DeliveryStatus::Delivered);

        let mut delivery2 = Delivery::new("John 2", "1234 Main St");

        assert_eq!(delivery2.status, DeliveryStatus::Pending);

        ship.start_delivery(&mut delivery2);

        assert_eq!(delivery2.status, DeliveryStatus::Shipped);

        ship.end_delivery(&mut delivery2);

        assert_eq!(delivery2.status, DeliveryStatus::Delivered);
    }
}