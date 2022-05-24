use crate::{
    device::Device,
    link::{Link, OnType},
};
use async_trait::async_trait;
use crossbeam_channel::Receiver;
use protocol::NetworkPacket;

pub struct LanLink;

#[async_trait]
impl Link for LanLink {
    async fn detect(interval: u64) {
        todo!()
    }

    async fn serve() -> Receiver<(OnType, Device)> {
        todo!()
    }

    fn pair(id: i64) {
        todo!()
    }

    fn unpair(id: i64) {
        todo!()
    }

    fn send_packet(packet: NetworkPacket) {
        todo!()
    }
}

#[tokio::test]
async fn test() {}
