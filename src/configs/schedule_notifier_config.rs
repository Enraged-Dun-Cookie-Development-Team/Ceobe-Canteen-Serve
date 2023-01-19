use scheduler_notifier::SchedulerNotifierConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ScheduleNotifierConfig {
    #[serde(alias = "url")]
    base_url: url::Url,
}


impl SchedulerNotifierConfig for ScheduleNotifierConfig {
    fn base_url(&self) -> url::Url {
        self.base_url.clone()
    }
}
