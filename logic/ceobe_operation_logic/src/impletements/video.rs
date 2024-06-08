use persistence::{ceobe_operate::{video::{self, bv::Bv}, ToCeobeOperation}, ceobe_user::ToCeobe, help_crates::tracing::{event, Level}, mysql::SqlDatabaseOperate};
use request_clients::bili_client::QueryBiliVideo;

use crate::{error::LogicResult, view::VideoItem};

use super::CeobeOperateLogic;



impl CeobeOperateLogic {
    /// 获取视频详细
    pub async fn get_video_detail(
        bv: Bv, query: QueryBiliVideo,
    ) -> LogicResult<String> {
        let body = query.fetch(bv).await??;
        event!(Level::INFO, response.len = body.len());
        Ok(String::from_utf8(body.to_vec())?)
    }

    /// 获取所有视频
    pub async fn list_all_video(
        sql: SqlDatabaseOperate
    ) -> LogicResult<Vec<VideoItem>> {
        Ok(sql.ceobe()
            .operation()
            .video()
            .find_all_not_delete()
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    /// 更新列表
    pub async fn update_list(
        sql: SqlDatabaseOperate, videos: Vec<video::Checked>,
    ) -> LogicResult<()> {
        sql.ceobe().operation().video().update_all(videos).await?;
        Ok(())
    }
}

