use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Link {
    async fn on_start(&self) -> Result<(), Box<dyn Error>>;
    async fn on_stop(&self) -> Result<(), Box<dyn Error>>;
    async fn on_network_change(&self) -> Result<(), Box<dyn Error>>;
    fn get_name(&self) -> &'static str;
}
