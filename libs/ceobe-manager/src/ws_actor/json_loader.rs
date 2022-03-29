use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler, MessageResult};
use actix_web::web::Bytes;

use crate::{
    ceobo_actor::{NewCeobeIncome, Updater, UpdaterReceiver},
    fut_utils::do_fut,
    models::{DataItem, DataSource},
};

pub struct JsonLoader {
    updater: Addr<Updater>,
}

impl Actor for JsonLoader {
    type Context = Context<JsonLoader>;
}

impl JsonLoader {
    pub(crate) fn start() -> (Addr<Self>, UpdaterReceiver) {
        let (updater, rec) = Updater::new();
        (
            Self {
                updater: updater.start(),
            }
            .start(),
            rec,
        )
    }
}

#[derive(actix::Message)]
#[rtype(result = "Result<(),serde_json::error::Error>")]
pub(crate) struct JsonData(Bytes);

impl From<Bytes> for JsonData {
    fn from(b: Bytes) -> Self { Self(b) }
}

impl Handler<JsonData> for JsonLoader {
    type Result = MessageResult<JsonData>;

    fn handle(
        &mut self, msg: JsonData, ctx: &mut Self::Context,
    ) -> Self::Result {
        let bytes = msg.0;
        match serde_json::from_slice::<HashMap<DataSource, Vec<DataItem>>>(
            &bytes,
        ) {
            Ok(data) => {
                #[cfg(feature = "log")]
                log_::info!(
                    "Loading New Cached From Json String size:[{}]",
                    bytes.len()
                );

                let req = self.updater.send(NewCeobeIncome::new_loaded(data));
                do_fut(req, ctx);
                MessageResult(Ok(()))
            }
            Err(e) => {
                log_::error!("Loading Json String Error :`{}`", e);
                MessageResult(Err(e))
            }
        }
    }
}
