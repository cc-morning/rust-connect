use crate::{
    device::Device,
    link::{Link, OnDevice, OnType},
};
use crossbeam_channel::{tick, unbounded, Receiver};
use protocol::NetworkPacket;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::task::JoinHandle;

pub struct LanLink {
    devices: Vec<Device>,
    task: Option<JoinHandle<()>>,
    is_drop: Arc<AtomicBool>,
}

impl LanLink {
    fn new() -> Self {
        Self {
            devices: vec![],
            task: None,
            is_drop: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Link for LanLink {
    fn detect(&mut self, interval: u64) -> Receiver<OnDevice> {
        let (tx, rx) = unbounded::<OnDevice>();

        let is_drop = self.is_drop.clone();
        self.task = Some(tokio::spawn(async move {
            let ticker = tick(Duration::from_millis(interval));
            let timeout = Duration::from_secs(1);

            while is_drop.load(Ordering::SeqCst) {
                if let Ok(_time) = ticker.recv_timeout(timeout) {
                    let _ = tx.send(OnDevice {
                        on: OnType::ADD,
                        id: 0,
                        name: String::from(""),
                        os: String::from(""),
                        last_date: 0,
                    });
                    todo!();
                }
            }
        }));

        rx
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

impl Drop for LanLink {
    fn drop(&mut self) {
        self.is_drop.store(true, Ordering::SeqCst);
    }
}
