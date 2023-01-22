mod checkers;
mod models;
mod operate;

pub use checkers::{
    resource_data::{
        CeobeOperationResource as Checked,
        CeobeOperationResourceChecker as Checker,
        CeobeOperationResourceUncheck as Uncheck,
    },
    CheckError,
};

pub mod all_available {
    pub use super::{
        checkers::resource_all_available::{
            ResourceAllAvailableCheck as Checked,
            ResourceAllAvailableChecker as Checker,
            ResourceAllAvailableUncheck as Uncheck,
        },
        models::model_resource::ResourceAllAvailable as Model,
    };
}
pub mod countdown {
    pub use super::{
        checkers::countdown::{
            CountdownCheck as Checked, CountdownChecker as Checker,
            CountdownUncheck as Uncheck,
        },
        models::model_resource::Countdown as Model,
    };
}

pub use models::{
    model_resource::{ActiveModel, Column, Entity, Model, Relation},
    resource_type::ResourceType,
};
pub use operate::{OperateError, ResourceOperate as Operate};
use sql_connection::database_traits::{
    database_operates::sub_operate::SuperOperate,
    get_connect::GetDatabaseConnect,
};

use super::SqlCeobeOperation;

impl<'c, C: GetDatabaseConnect> SqlCeobeOperation<'c, C> {
    pub fn resource(&'c mut self) -> Operate<'c, C> { self.child() }
}
