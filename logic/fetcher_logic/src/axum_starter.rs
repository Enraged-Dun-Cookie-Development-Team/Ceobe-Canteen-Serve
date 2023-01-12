use axum_starter::{prepare, state::AddState};

use crate::{config::ScheduleNotifier, error::PrepareError};
#[prepare(ScheduleNotifierPrepare? 'cfg)]
pub fn prepare_fetcher<'cfg, C>(
    config: &'cfg C,
) -> Result<AddState<ScheduleNotifier>, PrepareError>
where
    C: crate::config::FetcherLogicConfig,
{
    let notifier = ScheduleNotifier::new(config)?;

    Ok(AddState::new(notifier))
}

pub mod starter_state {
    pub use crate::config::ScheduleNotifier;
}
