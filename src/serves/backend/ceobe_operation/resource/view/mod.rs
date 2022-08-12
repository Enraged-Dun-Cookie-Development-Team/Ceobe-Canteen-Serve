mod all_available;
mod countdown;

use orm_migrate::sql_models::ceobe_operation::resource::models::model_resource;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Resource {
    #[serde(rename = "resources")]
    resource_all_available: AllAvailable,
    countdown: Vec<Countdown>,
}

pub use all_available::AllAvailable;
pub use countdown::Countdown;

impl
    From<(
        model_resource::ResourceAllAvailable,
        Vec<model_resource::Countdown>,
    )> for Resource
{
    fn from(
        (raa, cd): (
            model_resource::ResourceAllAvailable,
            Vec<model_resource::Countdown>,
        ),
    ) -> Self {
        Self {
            resource_all_available: raa.into(),
            countdown: cd.into_iter().map(Into::into).collect(),
        }
    }
}
