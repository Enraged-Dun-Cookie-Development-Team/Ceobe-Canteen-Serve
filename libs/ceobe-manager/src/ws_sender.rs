use std::sync::Arc;

use actix::Addr;

use crate::ws_actor::CeoboWebsocket;

pub struct WsSender(Addr<CeoboWebsocket>);

impl  WsSender {
    pub(crate) fn new(addr:Addr<CeoboWebsocket>)->Arc<Self>{
        Arc::new(Self(addr))
    }
}

