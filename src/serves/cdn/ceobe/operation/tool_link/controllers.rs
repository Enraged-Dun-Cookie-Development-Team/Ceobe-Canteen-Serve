use resp_result::{resp_try, rtry};
use ceobe_operation_logic::impletements::CeobeOperateLogic;
use crate::router::CdnOperateToolLinkFrontend;
use crate::serves::cdn::ceobe::operation::tool_link::error::CeobeToolLinkRResult;
/*use ceobe_operation_logic::view::ToolLinkResp;

use super::error::{CeobeOperateToolLinkError, CeobeToolLinkRResult};*/

impl CdnOperateToolLinkFrontend {

/*    pub async fn tool_link() -> CeobeToolLinkRResult<ToolLinkResp> {

    }*/

    pub async fn create_one() -> CeobeToolLinkRResult<()> {
        resp_try(async {
            Ok(())
        })
            .await
    }

    pub async fn list() -> () {
        print!("xxxxxxxxxxxxxxxxxxxxxxxxxxx")
    }

}