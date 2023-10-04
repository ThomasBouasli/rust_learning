#[derive(PartialEq)]
#[derive(Debug)]
pub enum DeliveryStatus {
    Pending,
    Shipped,
    Delivered,
}

pub struct Delivery {
    pub name: String,
    pub address: String,
    pub status: DeliveryStatus,
}

impl Delivery {
    pub fn new(name: &str, address: &str) -> Self {
        Self {
            name: name.to_string(),
            address: address.to_string(),
            status: DeliveryStatus::Pending,
        }
    }

    pub fn ship(&mut self) {
        match self.status {
            DeliveryStatus::Pending => {
                self.status = DeliveryStatus::Shipped;
            },
            _ => {
                println!("Delivery is already shipped");
            }
        }
    }

    pub fn deliver(&mut self) {
        match self.status {
            DeliveryStatus::Shipped => {
                self.status = DeliveryStatus::Delivered;
            },
            _ => {
                println!("Delivery is not shipped yet");
            }
        }
    }

   
}