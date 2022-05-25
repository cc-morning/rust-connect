use std::{sync::atomic::Ordering, time::Duration};

use crate::{
    device::Device,
    link::{Link, OnType, ALL_DEVICE, IS_STOP},
    utils,
};
use async_trait::async_trait;
use crossbeam_channel::{tick, Receiver};
use pnet::{datalink, ipnetwork::IpNetwork};
use protocol::NetworkPacket;

const PORT: i32 = 2780;

pub struct LanLink;

#[async_trait]
impl Link for LanLink {
    async fn detect(interval: u64) {
        let tick = tick(Duration::from_millis(interval));

        while !IS_STOP.load(Ordering::SeqCst) {
            let interface = utils::select_default_interface(&datalink::interfaces());

            if let Ok(_time) = tick.recv() {
                if let Some(interface) = interface {
                    interface.ips.iter().filter(|v| v.is_ipv4()).for_each(|v| {
                        v.iter().for_each(|v| {
                            let ip = v.to_string();
                            println!("{} - {}", interface.name, ip);
                        });
                    });
                }
            }
        }
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
async fn test() {
    tokio::spawn(async move {
        LanLink::detect(1500).await;
    });

    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("!");

    IS_STOP.swap(true, Ordering::SeqCst);
    println!("!");
}
