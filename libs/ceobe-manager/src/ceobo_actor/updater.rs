use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler, MessageResult};
use chrono::Local;
use tokio::sync::watch;

use super::{cached::Cached, CachedUpdateMsg, NewCeobeIncome};
use crate::{fut_utils::do_fut, models::DataSource};

pub struct Updater {
    sender:
        tokio::sync::watch::Sender<Option<HashMap<DataSource, Addr<Cached>>>>,
}

pub type UpdaterReceiver =
    watch::Receiver<Option<HashMap<DataSource, Addr<Cached>>>>;

impl Updater {
    pub fn new() -> (Self, UpdaterReceiver) {
        let (s, r) = tokio::sync::watch::channel(None);
        (Self { sender: s }, r)
    }
}

impl Actor for Updater {
    type Context = Context<Updater>;
}

impl Handler<NewCeobeIncome> for Updater {
    type Result = MessageResult<NewCeobeIncome>;

    fn handle(
        &mut self, msg: NewCeobeIncome, ctx: &mut Self::Context,
    ) -> Self::Result {
        match msg {
            NewCeobeIncome::Loaded(map) => {
                let now_timestamp = Local::now().timestamp() as u64;
                #[cfg(feature = "log")]
                log_::info!(
                    "Updater Handle DataSource Updating At {} Size : [{}]",
                    now_timestamp,
                    map.len()
                );

                let mut record = self
                    .sender
                    .send_replace(None)
                    .unwrap_or_else(|| HashMap::with_capacity(16));
                map.into_iter().for_each(|(k, v)| {
                    if let Some(addr) = record.get(&k) {
                        #[cfg(feature = "log")]
                        log_::info!(
                            "DataSource `{}` Exist , Update Cached Size:[{}]",
                            &*k,
                            v.len()
                        );

                        let res_timestamp = now_timestamp;
                        let msg = addr.send(CachedUpdateMsg {
                            res_timestamp,
                            data: v,
                        });
                        do_fut(msg, ctx);
                    }
                    else {
                        #[cfg(feature = "log")]
                        log_::info!("New DataSource `{}` Create", &*k);
                        let cached = Cached::new(now_timestamp, v).start();
                        record.insert(k, cached);
                    }
                });
                self.sender.send_replace(Some(record));

                #[cfg(feature = "log")]
                log_::info!("Updater Handle DataSource Updating Done");
                MessageResult(())
            }
        }
    }
}
