use resp_result::rtry;
use ceobe_operation_logic::impletements::CeobeOperateLogic;
use crate::router::CdnOperateToolLinkFrontend;
use crate::serves::cdn::ceobe::operate::tool_link::error::CeobeToolLinkRResult;
/*use ceobe_operation_logic::view::ToolLinkResp;

use super::error::{CeobeOperateToolLinkError, CeobeToolLinkRResult};*/

impl CdnOperateToolLinkFrontend {

/*    pub async fn tool_link() -> CeobeToolLinkRResult<ToolLinkResp> {

    }*/

    pub async fn create_one() -> CeobeToolLinkRResult<()> {
/*        Ok(rtry!(CeobeOperateLogic::create_tool_link()))*/
        CeobeToolLinkRResult{}
    }

    pub async fn list() -> () {
        print!("xxxxxxxxxxxxxxxxxxxxxxxxxxx")
    }

}