use std::borrow::Cow;

#[cfg(feature = "bcrypt")]
pub mod bcrypt;

#[cfg(feature="none")]
pub mod none;
/// 可以编码和比对一致性的密码加密器trait
pub trait Encoder {
    /// 当无法使用对应算法编码时使用这种加密算法时的异常内容
    type Error: std::error::Error;
    /// 将原文编码成密码，编码失败时返回`Err(Self::Error)`
    fn encode<'s>(raw: Cow<'s,str>) -> Result<Cow<'s, str>, Self::Error>;
    /// 判定密码是否匹配，如果无法判断返回 `Err(Self::Error)`
    fn verify<'s, S: AsRef<str>>(cryptoed: &Cow<'s, str>, input: &S) -> Result<bool, Self::Error>;
}
