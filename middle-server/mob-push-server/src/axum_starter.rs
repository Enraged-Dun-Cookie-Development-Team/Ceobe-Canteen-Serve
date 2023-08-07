use std::{sync::Arc, time::Duration};

use axum_starter::{prepare, state::AddState};
use secrecy::SecretString;
use tokio::sync::{mpsc, oneshot};

use crate::{push_manager::PartPushManagerState, MobPushConfigTrait};

#[prepare(MobPushPrepare)]
pub async fn init_mob_push<C>(
    config: &C,
) -> AddState<PartPushManagerState>
where
    C: MobPushConfigTrait,
{
    // start
    let push_admission = start_delay().await;

    let manager = PartPushManagerState::new(
        push_admission,
        Arc::new(SecretString::new(config.get_key().to_string())),
        Arc::new(SecretString::new(config.get_secret().to_string())),
    );

    AddState(manager)
}
async fn start_delay() -> mpsc::Sender<oneshot::Sender<()>> {
    let (rx, mut tx) = mpsc::channel::<oneshot::Sender<()>>(8);

    tokio::spawn(async move {
        let mut timer = tokio::time::interval(Duration::from_millis(500));
        while let Some(callback) = tx.recv().await {
            timer.tick().await;
            callback.send(()).ok();
        }
    });
    tokio::task::yield_now().await;

    rx
}
