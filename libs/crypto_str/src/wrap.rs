use std::ops::Deref;

use crate::{crypto::CryptoString, Encoder};

pub(crate) mod private {
    pub trait CryptoSrouce {}
}

/// 包装 `CryptoString` 的智能指针，用于提供原始数据来源类型（反序列化和Into时使用）
pub struct CryptoWarp<Src, E>(pub(crate) Src, pub(crate) CryptoString<E>);

impl<Src, E> Deref for CryptoWarp<Src, E> {
    type Target = CryptoString<E>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<Src, E> Into<CryptoString<E>> for CryptoWarp<Src, E>
where
    E: Encoder,
    Src: private::CryptoSrouce,
{
    fn into(self) -> CryptoString<E> {
        self.1
    }
}

pub struct Raw;

impl private::CryptoSrouce for Raw {}

pub struct Crypto;
impl private::CryptoSrouce for Crypto {}
