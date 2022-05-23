use compact_str::CompactStr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkPacket {
    id: i64,
    r#type: CompactStr,
    body: Vec<u8>,
    payload_transfer_info: Vec<u8>,
    payload_size: i64,
}
