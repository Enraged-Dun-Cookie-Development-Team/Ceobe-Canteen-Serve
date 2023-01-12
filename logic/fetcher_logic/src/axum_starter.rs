use axum_starter::{state::AddState, prepare};

use crate::{error::PrepareError, config::ScheduleNotifier};
#[prepare(ScheduleNotifierPrepare? 'cfg)]
pub fn prepare_fetcher<'cfg, C>(config: &'cfg C) -> Result<AddState<ScheduleNotifier>, PrepareError>
where
    C: crate::config::FetcherLogicConfig,
{
    let notifier = ScheduleNotifier::new(config)?;

    Ok(AddState::new(notifier))
}

pub mod starter_state{
    pub use crate::config::ScheduleNotifier;
}
