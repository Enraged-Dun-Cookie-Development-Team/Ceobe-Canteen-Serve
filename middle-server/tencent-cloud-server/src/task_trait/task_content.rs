use std::{ io, io::Write};
use serde::Serialize;
use crate::task_trait::serde_content::{Empty, SerializeContentTrait};

/// 调用接口的荷载内容，包括Query部分和Body部分
pub trait TaskContent {
    
    /// Query序列化并写入到W中，返回写入字符数量
    fn query(&self) -> &impl Serialize { &Empty }

    /// Payload 的 序列化荷载类型
    type Payload: SerializeContentTrait;
    /// 获得荷载的引用
    fn payload(&self) -> &Self::Payload;
}

#[cfg(test)]
mod test{
    use crate::task_trait::serde_content::Empty;

    #[test]
    fn test_serde_empty(){
        let out = serde_qs::to_string(&Empty).unwrap();
        assert_eq!(out,"");
    }
}