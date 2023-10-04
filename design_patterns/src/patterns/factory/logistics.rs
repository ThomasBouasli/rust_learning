use super::transport::Transport;

pub mod road;
pub mod sea;

pub trait Logistics {
    fn create_transport(&self) -> Box<dyn Transport>;
}