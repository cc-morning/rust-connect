pub enum DeviceType {
    Unknown,
    Desktop,
    Laptop,
    Phone,
    Tablet,
    Tv,
}

pub struct Device {
    id: i64,
    name: String,
    r#type: DeviceType,
    last_date: i64,
}
