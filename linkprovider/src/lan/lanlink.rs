use crate::link::{Link, OnDevice};

use crossbeam_channel::Receiver;
use protocol::NetworkPacket;

pub struct LanLink;

impl Link for LanLink {
    fn detect(&self, interval: i32) -> Receiver<OnDevice> {
        todo!()
    }

    fn pair(&self, id: i64) {
        todo!()
    }

    fn unpair(&self, id: i64) {
        todo!()
    }

    fn send_packet(&self, packet: NetworkPacket) {
        todo!()
    }
}
