use mongo_migration::mongo_connection::MongoDatabaseOperate;
use orm_migrate::sql_connection::SqlDatabaseOperate;

use crate::router::CeobeUserFrontend;

use super::error::CeobeUserRResult;
impl CeobeUserFrontend {
    /// 获取平台与数据源类型列表
    #[instrument(ret, skip(db))]
    pub async fn register(
        db: SqlDatabaseOperate, mongo: MongoDatabaseOperate
    ) -> CeobeUserRResult<()> {
        rtry!(
           
        );
        Ok(()).into()
    }
}