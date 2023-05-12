/// 主列表相关配置KEY
pub struct CookieListKey;
impl CookieListKey {
    // hashmap: 每个数据源对应最新的饼id
    pub const NEWEST_COOKIES: &str = "cookie:list:newest:datasources";
    // hashmap: 每个数据源对应最新更新的饼id
    pub const NEW_UPDATE_COOKIES: &str = "cookie:list:new:update:map";
    // string：带时间缓存的更新饼id
    pub const NEW_UPDATE_COOKIE_ID: &str = "cookie:list:new:update:id:";
}
