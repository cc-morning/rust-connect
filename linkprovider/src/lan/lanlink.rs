use crate::{link::Link, utils};
use async_trait::async_trait;
use futures_util::StreamExt;
use protocol::NetworkPacket;
use quinn::{ClientConfig, Endpoint, EndpointConfig, NewConnection, ServerConfig};
use serde_json::json;
use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    sync::Arc,
};
use tokio::sync::mpsc::{self, UnboundedReceiver};

const MIN_PORT: u16 = 1716;
const MAX_PORT: u16 = 1764;

struct SkipServerVerification;

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

#[derive(Default)]
pub struct LanLink;

impl LanLink {
    pub fn detect(&self) {}

    fn configure_client() -> Result<ClientConfig, Box<dyn Error>> {
        let crypto = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(SkipServerVerification::new())
            .with_no_client_auth();

        Ok(ClientConfig::new(Arc::new(crypto)))
    }

    fn configure_server() -> Result<ServerConfig, Box<dyn Error>> {
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
        let priv_key = rustls::PrivateKey(cert.serialize_private_key_der());
        let cert_chain = vec![rustls::Certificate(cert.serialize_der()?)];

        let mut server_config = ServerConfig::with_single_cert(cert_chain, priv_key)?;
        Arc::get_mut(&mut server_config.transport)
            .unwrap()
            .max_concurrent_uni_streams(0_u8.into());

        Ok(server_config)
    }

    pub async fn client(&self, addr: SocketAddr) -> Result<NewConnection, Box<dyn Error>> {
        let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
        endpoint.set_default_client_config(LanLink::configure_client()?);
        let conn = endpoint.connect(addr, "localhost")?;

        Ok(conn.await?)
    }

    pub async fn server(&self) -> Result<UnboundedReceiver<NetworkPacket>, Box<dyn Error>> {
        let port = utils::free_port_in_range(Ipv4Addr::new(0, 0, 0, 0), MIN_PORT, MAX_PORT);
        if port.is_none() {}

        let (sender, receiver) = mpsc::unbounded_channel();
        let (_, mut incoming) = Endpoint::new(
            EndpointConfig::default(),
            Some(Self::configure_server()?),
            UdpSocket::bind(format!("0.0.0.0:{}", port.unwrap()))?,
        )?;

        tokio::spawn(async move {
            let sender = Arc::new(sender);

            while let Some(conn) = incoming.next().await {
                if let Ok(conn) = conn.await {
                    let sender = sender.clone();

                    tokio::spawn(async move {
                        let mut datagrams = conn.datagrams;

                        if let Some(Ok(bytes)) = datagrams.next().await {
                            let packet = NetworkPacket::deserialize(bytes).expect("");
                            sender.send(packet).expect("");
                        }
                    });
                }
            }
        });

        Ok(receiver)
    }
}

#[async_trait]
impl Link for LanLink {
    async fn on_start(&self) -> Result<(), Box<dyn Error>> {
        let receiver = self.server().await?;
        Ok(())
    }

    async fn on_stop(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn on_network_change(&self) -> Result<(), Box<dyn Error>> {
        self.detect();
        Ok(())
    }

    fn get_name(&self) -> &'static str {
        "lan"
    }
}

#[tokio::test]
async fn test() -> Result<(), Box<dyn Error>> {
    let lan = LanLink::default();

    let mut receiver = lan.server().await?;
    let task = tokio::spawn(async move {
        loop {
            if let Some(packet) = receiver.recv().await {
                println!("{:?}", packet);
            }
        }
    });

    let client = lan
        .client(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), MIN_PORT))
        .await?;

    let mut packet = NetworkPacket::default();
    packet.set_id(1);
    packet.set_body(json!({
        "a": 1,
        "b": 2,
        "c": vec![ "a", "b", "c" ]
    }));

    client
        .connection
        .send_datagram(packet.serialize()?)
        .unwrap();

    task.await?;
    Ok(())
}
