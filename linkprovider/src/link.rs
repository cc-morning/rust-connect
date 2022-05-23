use crossbeam_channel::Receiver;
use protocol::NetworkPacket;

pub trait Link {
    fn detect(&mut self, interval: u64) -> Receiver<OnDevice>;
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
    pub on: OnType,
    pub id: i64,
    pub name: String,
    pub os: String,
    pub last_date: i64,
}
