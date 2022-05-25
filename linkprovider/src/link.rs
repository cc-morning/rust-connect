use crate::device::Device;
use async_trait::async_trait;
use crossbeam_channel::Receiver;
use protocol::NetworkPacket;
use std::sync::atomic::AtomicBool;
use tokio::sync::Mutex;

pub(crate) static IS_STOP: AtomicBool = AtomicBool::new(false);
pub(crate) static ALL_DEVICE: Mutex<Vec<Device>> = Mutex::const_new(vec![]);

#[async_trait]
pub trait Link {
    async fn detect(interval: u64);
    async fn serve() -> Receiver<(OnType, Device)>;
    fn pair(id: i64);
    fn unpair(id: i64);
    fn send_packet(packet: NetworkPacket);
}

#[derive(PartialEq, Eq)]
pub enum OnType {
    PAIR,
    UNPAIR,
    PACKET,
}
