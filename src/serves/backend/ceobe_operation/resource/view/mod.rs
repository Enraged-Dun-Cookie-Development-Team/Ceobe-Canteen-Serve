mod all_available;
mod countdown;

use orm_migrate::sql_models::ceobe_operation::resource;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Resource {
    #[serde(rename = "resources")]
    resource_all_available: AllAvailable,
    countdown: Vec<Countdown>,
}

pub use self::{all_available::AllAvailable, countdown::Countdown};

impl
    From<(
        resource::all_available::Model,
        Vec<resource::countdown::Model>,
    )> for Resource
{
    fn from(
        (raa, cd): (
            resource::all_available::Model,
            Vec<resource::countdown::Model>,
        ),
    ) -> Self {
        Self {
            resource_all_available: raa.into(),
            countdown: cd.into_iter().map(Into::into).collect(),
        }
    }
}
