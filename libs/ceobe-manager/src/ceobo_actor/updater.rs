use actix::{Actor, Addr, AsyncContext, Context, Handler, MessageResult};
use chrono::Local;
use dashmap::DashMap;

use crate::models::DataSource;

use super::{cached::Cached, CachedUpdateMsg, NewCeobeIncome};

pub struct Updater {
    ceobe_record: DashMap<DataSource, Addr<Cached>>,
}

impl Updater {
    pub fn new() -> Self {
        Self {
            ceobe_record: DashMap::with_capacity(16),
        }
    }
}

impl Actor for Updater {
    type Context = Context<Updater>;
}

impl Handler<NewCeobeIncome> for Updater {
    type Result = MessageResult<NewCeobeIncome>;

    fn handle(&mut self, msg: NewCeobeIncome, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            NewCeobeIncome::Nil => MessageResult(()),
            NewCeobeIncome::Loaded(map) => {
                let now_timestamp = Local::now().timestamp() as u64;
                map.into_iter().for_each(|(k, v)| {
                    if let Some(addr) = self.ceobe_record.get(&k) {
                        let addr = addr.value();
                        let res_timestamp = now_timestamp;
                        let msg = addr.send(CachedUpdateMsg {
                            res_timestamp,
                            data: v,
                        });
                        let no_return_msg = async move {
                            msg.await.ok();
                            ()
                        };
                        let fut_warp = actix::fut::wrap_future::<_, Self>(no_return_msg);
                        ctx.spawn(fut_warp);
                    } else {
                        let cached = Cached::new(now_timestamp, v).start();
                        self.ceobe_record.insert(k, cached);
                    }

                });
                println!("updated {:#?}",self.ceobe_record);
                MessageResult(())
            }
        }
    }
}
