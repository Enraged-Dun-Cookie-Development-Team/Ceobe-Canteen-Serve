/// 蹲饼器相关配置KEY
pub struct FetcherConfigKey;
impl FetcherConfigKey {
    /// 至今蹲饼器存活的最大数量
    pub const LIVE_NUMBER: LiveNumber = LiveNumber;
}

redis_key!(LiveNumber::<i8> => "cookie:fetcher:config:live:number");
