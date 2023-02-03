use checker::{
    check_gen,
    prefabs::{
        date_time_format::DateTimeFormatChecker, no_check::NoCheck,
        str_len_checker::StrMaxCharLenChecker,
    },
};
use chrono::NaiveDateTime;
use sea_orm::Set;
use typed_builder::TypedBuilder;

use super::CheckError;
use crate::ceobe_operation::announcement::models::model_announcement;

#[derive(Debug, TypedBuilder)]
pub struct CeobeOpAnnouncement {
    pub start_time: NaiveDateTime,
    pub over_time: NaiveDateTime,
    pub content: String,
    pub img_url: String,
    pub notice: bool,
}
#[check_gen(
    uncheck = CeobeOpAnnouncementUncheck,
    checked = CeobeOpAnnouncement,
    error = CheckError
)]
#[derive(Debug, serde::Deserialize)]
pub struct CeobeOpAnnouncementChecker {
    pub start_time: DateTimeFormatChecker,
    pub over_time: DateTimeFormatChecker,
    pub content: StrMaxCharLenChecker<String, 4096>,
    pub img_url: StrMaxCharLenChecker<String, 256>,
    pub notice: NoCheck<bool>,
}

impl model_announcement::ActiveModel {
    pub fn from_announcement_data_with_order(
        CeobeOpAnnouncement {
            start_time,
            over_time,
            content,
            img_url,
            notice,
        }: CeobeOpAnnouncement,
        order: i32,
    ) -> Self {
        Self {
            start_time: Set(start_time),
            over_time: Set(over_time),
            content: Set(content),
            img_url: Set(img_url),
            order: Set(order),
            notice: Set(notice),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use range_limit::measurable::Measurable;

    #[test]
    fn it_works() {
        let test_str = "<div class='online-area'> <a class='webOpen' href='http://www.ceobecanteen.top/'><img class='online-title-img' src='/assets/image/icon.png'></a> <div> <div>博士，谢谢你使用蹲饼。</div> <div>如果觉得好用的话，希望能去<a class='webOpen' href='https://github.com/Enraged-Dun-Cookie-Development-Team/Dun-Cookie-Vue'>GitHub</a>上点个<span class='online-red'>Star</span>或者</div> <div>去<a class='webOpen' href='https://chrome.google.com/webstore/detail/%E8%B9%B2%E9%A5%BC-%E6%98%8E%E6%97%A5%E6%96%B9%E8%88%9F%E8%B9%B2%E9%A5%BC%E5%99%A8-arknights-cook/gblmdllhbodefkmimbcjpflhjneagkkd?hl=zh-CN'>Chrome商店</a>，<a class='webOpen' href='https://microsoftedge.microsoft.com/addons/detail/%E5%B0%8F%E5%88%BB%E9%A3%9F%E5%A0%82-%E6%98%8E%E6%97%A5%E6%96%B9%E8%88%9F%E8%B9%B2%E9%A5%BC%E5%99%A8-arknight/jimmfliacfpeabcifcghmdankmdnmfmn?hl=zh-CN'>Edge商店</a>或<a class='webOpen' href='https://addons.mozilla.org/zh-CN/firefox/addon/%E5%B0%8F%E5%88%BB%E9%A3%9F%E5%A0%82-%E6%98%8E%E6%97%A5%E6%96%B9%E8%88%9F%E8%B9%B2%E9%A5%BC%E5%99%A8-arknights-cookies/'>Firefox商店</a>里面给个<span class='online-red'>好评</span></div> <div>也可以去<a class='webOpen' href='https://arknightscommunity.drblack-system.com/15386.html'>泰拉通讯枢纽</a>里面<span class='online-red'>回复我们</span>，或者去<a class='webOpen' href='https://www.bilibili.com/video/BV1R44y1M7s5/'>b站视频</a>给个<span class='online-red'>三连</span></div> <div style='display: flex;align-items: center;'>欢迎加群 <a class='webOpen' href='https://jq.qq.com/?_wv=1027&k=Vod1uO13'>【蹲饼组】</a> 一起all，all叫！<img class='online-title-img' style='height:16px;min-width:auto' src='http://api.ceobecanteen.top/canteen/all'></div> </div> </div>";
        println!("测试用公告信息长度为{}", test_str.size());
        assert_eq!(2 + 2, 4);
    }
}
