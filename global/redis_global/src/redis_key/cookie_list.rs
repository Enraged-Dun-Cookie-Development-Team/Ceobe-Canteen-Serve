/// 主列表相关配置KEY
pub struct CookieListKey;
impl CookieListKey {
    /// hashmap: 每个数据源组合对应最新的饼id
    #[deprecated]
    pub const NEWEST_COOKIES: &'static str = "cookie:list:newest:combId";
    /// hashmap: 每个数据源组合对应最新的饼id
    pub const NEW_COMBID_INFO: &'static str = "cookie:list:new:combId:info";
    /// hashmap: 每个数据源对应最新更新的饼id
    pub const NEW_UPDATE_COOKIES: &'static str = "cookie:list:new:update:map";
    /// string：带时间缓存的更新饼id
    pub const NEW_UPDATE_COOKIE_ID: &'static str =
        "cookie:list:new:update:id:";
}
