use ceobe_operation_logic::{impletements::CeobeOperateLogic, view::VideoItem};
use checker::{
    prefabs::collect_checkers::iter_checkers::IntoIterChecker, CheckExtract,
    JsonCheckExtract, QueryCheckExtract,
};
use persistence::{
    ceobe_operate::{
        models::video::{self, bv::query::Checked as BvQuery},
        ToCeobe, ToCeobeOperation,
    },
    mysql::SqlDatabaseOperate,
};
use request_clients::bili_client::QueryBiliVideo;
use resp_result::{resp_try, rtry, RespResult};
use tencent_cloud_server::cloud_manager::CloudManager;
use tracing::{event, instrument, Level};

use super::{
    error::{CeobeOperationVideoError, VideoRespResult},
};
use crate::router::CeobeOperationVideo;

type BvQueryCheck =
    QueryCheckExtract<video::bv::query::Checker, CeobeOperationVideoError>;

type UpdateVideoCheck = JsonCheckExtract<
    IntoIterChecker<Vec<video::Uncheck>, video::Checker, Vec<video::Checked>>,
    CeobeOperationVideoError,
>;

impl CeobeOperationVideo {
    #[instrument(skip(query))]
    pub async fn get_video_detail(
        CheckExtract(BvQuery { bv }): BvQueryCheck, query: QueryBiliVideo,
    ) -> VideoRespResult<String> {
        resp_try(async {
            Ok(CeobeOperateLogic::get_video_detail(bv, query).await?)
        })
        .await
    }

    #[instrument(ret, skip(database))]
    pub async fn list_all(
        database: SqlDatabaseOperate,
    ) -> VideoRespResult<Vec<VideoItem>> {
        resp_try(async {
            Ok(CeobeOperateLogic::list_all_video(database).await?)
        })
        .await
    }

    #[instrument(ret, skip(db, tc_cloud))]
    pub async fn update_list(
        db: SqlDatabaseOperate, tc_cloud: CloudManager, CheckExtract(videos): UpdateVideoCheck,
    ) -> VideoRespResult<()> {
        rtry!(CeobeOperateLogic::update_list(db, tc_cloud, videos).await);
        RespResult::ok(())
    }
}
