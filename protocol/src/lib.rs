use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

const PROTOCOL_VERSION: i32 = 7;

#[derive(Serialize, Deserialize, Debug)]
pub enum PacketType {
    IDENTITY,
    PAIR,
}

impl Default for PacketType {
    fn default() -> Self {
        Self::IDENTITY
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NetworkPacket {
    id: i64,
    r#type: PacketType,
    body: Value,
    payload_transfer_info: Value,
    payload_size: i64,
}

impl NetworkPacket {
    pub fn id(&self) -> &i64 {
        &self.id
    }

    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }

    pub fn r#type(&self) -> &PacketType {
        &self.r#type
    }

    pub fn set_type(&mut self, r#type: PacketType) {
        self.r#type = r#type;
    }

    pub fn body(&self) -> &Value {
        &self.body
    }

    pub fn set_body(&mut self, body: Value) {
        self.body = body;
    }

    pub fn payload_transfer_info(&self) -> &Value {
        &self.payload_transfer_info
    }

    pub fn set_payload_transfer_info(&mut self, payload_transfer_info: Value) {
        self.payload_transfer_info = payload_transfer_info;
    }

    pub fn payload_size(&self) -> &i64 {
        &self.payload_size
    }

    pub fn set_payload_size(&mut self, payload_size: i64) {
        self.payload_size = payload_size;
    }

    pub fn serialize(&self) -> Result<Bytes, Box<dyn Error>> {
        Ok(Bytes::from(serde_json::to_string(self)?))
    }

    pub fn deserialize(bytes: Bytes) -> Result<Self, Box<dyn Error>> {
        Ok(serde_json::from_str(&String::from_utf8(bytes.to_vec())?)?)
    }
}
