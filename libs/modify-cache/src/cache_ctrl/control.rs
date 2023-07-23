use std::{ops::Add, time::Duration};

use http::{header::CACHE_CONTROL, HeaderValue};
use resp_result::{ExtraFlag, ExtraFlags};

#[derive(Debug, Default)]
/// 可缓存性
pub enum CacheMode {
    /// 表明响应可以被任何对象（包括：发送请求的客户端，代理服务器，
    /// 等等）缓存，即使是通常不可缓存的内容。（例如：1.
    /// 该响应没有`max-age`指令或`Expires`消息头；2. 该响应对应的请求方法是
    /// `POST` 。）
    #[default]
    Public,
    /// 表明响应只能被单个用户缓存，
    /// 不能作为共享缓存（即代理服务器不能缓存它）。
    /// 私有缓存可以缓存响应内容，比如：对应用户的本地浏览器。
    Private,
    /// 在发布缓存副本之前，强制要求缓存把请求提交给原始服务器进行验证
    /// (协商缓存验证)。
    NoCache,
    /// 缓存不应存储有关客户端请求或服务器响应的任何内容，即不使用任何缓存。
    NoStore,
}

#[derive(Debug, Default)]
/// 一旦资源过期（比如已经超过`max-age`），在成功向原始服务器验证之前，
/// 缓存不能用该资源响应后续请求。
pub enum Revalidate {
    #[default]
    Need,
    None,
}
#[derive(Debug, Default)]
/// 不得对资源进行转换或转变。Content-Encoding、Content-Range、Content-Type等
/// HTTP 头不能由代理修改。
pub enum Transform {
    #[default]
    Allow,
    Deny,
}

#[derive(Debug)]
pub struct CacheControl {
    ty: CacheMode,
    /// 设置缓存存储的最大周期，超过这个时间缓存被认为过期 (单位秒)
    max_age: Duration,
    revalidate: Revalidate,
    transform: Transform,
}

impl CacheControl {
    pub fn set_ty(&mut self, ty: CacheMode) -> &mut Self {
        self.ty = ty;
        self
    }

    pub fn set_max_age(&mut self, max_age: Duration) -> &mut Self {
        self.max_age = max_age;
        self
    }

    pub fn set_revalidate(&mut self, revalidate: Revalidate) -> &mut Self {
        self.revalidate = revalidate;
        self
    }

    pub fn set_transform(&mut self, transform: Transform) -> &mut Self {
        self.transform = transform;
        self
    }
}

impl CacheControl {
    fn to_header_value(&self) -> Option<HeaderValue> {
        let mut info = Vec::with_capacity(4);

        match self.ty {
            CacheMode::Public => info.push("public".to_string()),
            CacheMode::Private => info.push("private".to_string()),
            CacheMode::NoCache => info.push("no-cache".to_string()),
            CacheMode::NoStore => info.push("no-store".to_string()),
        }

        if let CacheMode::NoStore = self.ty {
        }
        else {
            info.push(format!(
                "s-maxage={0}, max-age={0}",
                self.max_age.as_secs()
            ));

            if let Revalidate::Need = self.revalidate {
                info.push("must-revalidate, proxy-revalidate".to_string())
            }

            if let Transform::Deny = self.transform {
                info.push("no-transform".into())
            }
        }

        info.into_iter()
            .reduce(|l, r| format!("{l}, {r}"))
            .and_then(|v| HeaderValue::from_str(&v).ok())
    }
}

impl Default for CacheControl {
    fn default() -> Self {
        Self {
            ty: Default::default(),
            max_age: Duration::from_secs(28800),
            revalidate: Default::default(),
            transform: Default::default(),
        }
    }
}

impl<'c> Add<&'c CacheControl> for ExtraFlags {
    type Output = ExtraFlags;

    fn add(self, rhs: &'c CacheControl) -> Self::Output {
        if let Some(header_value) = rhs.to_header_value() {
            self + ExtraFlag::insert_header(CACHE_CONTROL, header_value)
        }
        else {
            self
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::CacheControl;

    #[test]
    fn test_header_value() {
        let mut cache = CacheControl::default();
        cache
            .set_max_age(Duration::from_secs(60 * 60 * 24))
            .set_transform(super::Transform::Deny)
            .set_revalidate(super::Revalidate::None);

        let h = cache.to_header_value().unwrap();

        println!("{h:?}")
    }
}
