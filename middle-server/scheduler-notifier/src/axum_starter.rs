use axum_starter::{prepare, state::AddState};

use crate::{SchedulerNotifierConfig, SchedulerUrl};

pub mod starter_state {
    pub use crate::SchedulerUrl;
}

#[prepare(ScheduleNotifierPrepare)]
pub fn prepare_fetcher<C>(config: &C) -> AddState<SchedulerUrl>
where
    C: SchedulerNotifierConfig,
{
    let base_url = SchedulerUrl::new_cfg(config);

    AddState::new(base_url)
}
