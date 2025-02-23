use crate::data::DatabasePool;
use crate::service::{self, ServiceError};
use crate::ShortCode;
use crossbeam_channel::TryRecvError;
use crossbeam_channel::{unbounded, Receiver, Sender};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Handle;
use tokio::sync::mpsc::error;

type HitStore = Arc<Mutex<HashMap<ShortCode, u32>>>;

#[derive(Debug, thiserror::Error)]
enum HitCounterError {
    #[error("service error: {0}")]
    Service(#[from] ServiceError),
    #[error("communication error: {0}")]
    Channel(#[from] crossbeam_channel::SendError<HitCountMsg>),
}

enum HitCountMsg {
    Commit,
    Hit(ShortCode, u32),
}

pub struct HitCounter {
    tx: Sender<HitCountMsg>,
}

impl HitCounter {
    fn commit_hits(
        hits: HitStore,
        handle: Handle,
        pool: DatabasePool,
    ) -> Result<(), HitCounterError> {
        let hits = Arc::clone(&hits);
        let hits: Vec<(ShortCode, u32)> = {
            let mut hits = hits.lock();
            let hits_vec = hits.iter().map(|(k, v)| (k.clone(), *v)).collect();
            hits.clear();
            hits_vec
        };

        handle.block_on(async move {
            let transaction = service::action::begin_transaction(&pool).await?;
            for (shortcode, hits) in hits {
                if let Err(e) = service::action::increase_hit_count(&shortcode, hits, &pool).await {
                    eprintln!("error increasing hit count: {}", e);
                }
            }
            Ok(service::action::end_transaction(transaction).await?)
        })
    }

    fn proccess_msg(
        msg: HitCountMsg,
        hits: HitStore,
        handle: Handle,
        pool: DatabasePool,
    ) -> Result<(), HitCounterError> {
        match msg {
            HitCountMsg::Commit => Self::commit_hits(hits.clone(), handle.clone(), pool.clone())?,
            HitCountMsg::Hit(shortcode, count) => {
                let mut hitcount = hits.lock();
                let hitcount = hitcount.entry(shortcode).or_insert(0);
                *hitcount += count;
            }
        }
        Ok(())
    }

    pub fn new(pool: DatabasePool, handle: Handle) -> Self {
        let (tx, rx) = unbounded();
        let tx_clone = tx.clone();
        let rx_clone = rx.clone();

        let _ = std::thread::spawn(move || {
            println!("HitCounter thread spawned");
            let store: HitStore = Arc::new(Mutex::new(HashMap::new()));

            loop {
                match rx_clone.try_recv() {
                    Ok(msg) => {
                        if let Err(e) =
                            Self::proccess_msg(msg, store.clone(), handle.clone(), pool.clone())
                        {
                            eprintln!("message proccessing error: {}", e);
                        }
                    }
                    Err(e) => match e {
                        TryRecvError::Empty => {
                            std::thread::sleep(Duration::from_secs(5));
                            if let Err(e) = tx_clone.send(HitCountMsg::Commit) {
                                eprintln!("error sending commit msg to hits channel: {}", e);
                            }
                        }
                        _ => break,
                    },
                }
            }
        });
        Self { tx }
    }

    pub fn hit(&self, shorcode: ShortCode, count: u32) {
        if let Err(e) = self.tx.send(HitCountMsg::Hit(shorcode, count)) {
            eprintln!("hit count error: {}", e);
        }
    }
}
