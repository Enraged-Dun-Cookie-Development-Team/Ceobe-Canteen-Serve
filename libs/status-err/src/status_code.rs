use crate::ErrPrefix;

pub struct StatusCode {
    /// 一个 char 的异常前缀
    prefix: ErrPrefix,
    /// 4 位的异常码
    code: u16,
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:04}", self.prefix, self.code)
    }
}

impl serde::Serialize for StatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let info = format!("{}{:04}", self.prefix, self.code);

        info.serialize(serializer)
    }
}

impl StatusCode {
    pub const fn new(pre: ErrPrefix, code: u16) -> Self {
        Self { prefix: pre, code }
    }

    pub fn http_code(&self) -> http::StatusCode { self.prefix.get_status() }

    pub fn get_prefix(&self) -> ErrPrefix {
        let p = self.prefix;
        p
    }

    pub fn get_code(&self) -> u16 { self.code }
}
