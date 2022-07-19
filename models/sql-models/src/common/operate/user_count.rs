use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, QueryFilter,
    QuerySelect,
};

use super::{CommonSqlOperate, UserCounts};
use crate::common::sql_models::user;

impl CommonSqlOperate {
    pub async fn user_count_raw(
        filter: impl Into<Option<Condition>>, db: &impl ConnectionTrait,
    ) -> Result<bool, DbErr> {
        let condition = filter.into().unwrap_or_else(|| Condition::all());

        let resp = user::Entity::find()
            .filter(condition)
            .select_only()
            .column_as(user::Column::Id.count(), "count")
            .into_model::<UserCounts>()
            .one(db)
            .await;
        unimplemented!()
    }
}
