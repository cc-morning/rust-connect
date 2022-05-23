use crossbeam_channel::Receiver;
use protocol::NetworkPacket;

pub trait Link {
    fn detect(&self, interval: i32) -> Receiver<OnDevice>;
    fn pair(&self, id: i64);
    fn unpair(&self, id: i64);
    fn send_packet(&self, packet: NetworkPacket);
}

#[derive(PartialEq, Eq)]
pub enum OnType {
    ADD,
    DEL,
    PAIR,
    UNPAIR,
    PACKET,
}

pub struct OnDevice {
    on: OnType,
    id: i64,
    name: String,
    os: String,
    last_date: i64,
}
