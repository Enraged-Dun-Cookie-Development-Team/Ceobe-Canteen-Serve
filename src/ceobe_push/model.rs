use std::{borrow::Cow, sync::{atomic::{AtomicU64, Ordering}, Arc}, time::Duration};

use chrono::Local;



pub(super) type DataSourceFilter=Vec<(u64,Cow<'static,str>)>;

pub struct MockTimer(AtomicU64);

impl MockTimer {
    pub fn new()->Arc<Self>{
        let now=Local::now().timestamp() as u64;
        Arc::new(Self(AtomicU64::new(now)))
    }

    pub async fn updating(self :Arc<Self>){
        let mut timer=tokio::time::interval(Duration::from_secs(10));

        loop {
            timer.tick().await;
            let now=Local::now().timestamp() as u64;
            self.0.store(now, Ordering::Relaxed)
        }
    }

    pub fn load(&self)->u64{
        self.0.load(Ordering::Relaxed)
    }
}


