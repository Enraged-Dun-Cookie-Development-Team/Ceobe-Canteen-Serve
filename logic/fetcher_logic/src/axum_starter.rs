use axum_starter::{prepare, state::AddState};

use self::starter_state::FetcherNotifyScheduleUrl;
use crate::error::PrepareError;
#[prepare(ScheduleNotifierPrepare? 'cfg)]
pub fn prepare_fetcher<'cfg, C>(
    config: &'cfg C,
) -> Result<AddState<FetcherNotifyScheduleUrl>, PrepareError>
where
    C: crate::config::FetcherLogicConfig,
{
    let base_url = FetcherNotifyScheduleUrl::new_cfg(config)?;

    Ok(AddState::new(base_url))
}

pub mod starter_state {
    pub use crate::notifier::FetcherNotifyScheduleUrl;
}
