use serde::{Deserialize, Serialize};

const PROTOCOL_VERSION: i32 = 7;

#[derive(Serialize, Deserialize, Debug)]
pub enum PacketType {
    IDENTITY,
    PAIR,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkPacket {
    id: i64,
    r#type: PacketType,
    body: Vec<u8>,
    payload_transfer_info: Vec<u8>,
    payload_size: i64,
}

impl NetworkPacket {
    fn id(&self) -> &i64 {
        &self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = id;
    }

    fn r#type(&self) -> &PacketType {
        &self.r#type
    }

    fn set_type(&mut self, r#type: PacketType) {
        self.r#type = r#type;
    }

    fn body(&self) -> &Vec<u8> {
        &self.body
    }

    fn set_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    fn payload_transfer_info(&self) -> &Vec<u8> {
        &self.payload_transfer_info
    }

    fn set_payload_transfer_info(&mut self, payload_transfer_info: Vec<u8>) {
        self.payload_transfer_info = payload_transfer_info;
    }

    fn payload_size(&self) -> &i64 {
        &self.payload_size
    }

    fn set_payload_size(&mut self, payload_size: i64) {
        self.payload_size = payload_size;
    }
}
