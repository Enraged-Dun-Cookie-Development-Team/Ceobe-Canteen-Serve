/// 主列表相关配置KEY
pub struct CookieListKey;
impl CookieListKey {
    /// hashmap: 每个数据源组合对应最新的饼id
    #[deprecated]
    pub const NEWEST_COOKIES: NewestCookies = NewestCookies;
    /// hashmap: 每个数据源组合对应最新的饼id
    pub const NEW_COMBID_INFO: NewCombIdInfo = NewCombIdInfo;
    /// hashmap: 每个数据源对应最新更新的饼id
    pub const NEW_UPDATE_COOKIES: NewUpdateCookies = NewUpdateCookies;
    /// string：带时间缓存的更新饼id
    pub const NEW_UPDATE_COOKIE_ID: NewUpdateCookieId = NewUpdateCookieId;
}
use bson::oid::ObjectId;
use crate::CookieId;
redis_key!(hash NewestCookies::<String> => "cookie:list:newest:combId");
redis_key!(hash NewCombIdInfo::<String> => "cookie:list:new:combId:info");
redis_key!(hash NewUpdateCookies::<CookieId> => "cookie:list:new:update:map");
redis_key!(NewUpdateCookieId::<bool> => "cookie:list:new:update:id:{}" [cookie_id:ObjectId]);
