pub mod impls;
pub mod codegen;
use http::StatusCode;


pub trait StatusErr:std::error::Error {
    /// 异常码前缀标识符
    /// 用于唯一标记某一类型异常
    fn prefix(&self)->ErrPrefix;
    /// 4位的异常码，指明具体异常
    fn code(&self)->u16;
    /// 对应的http状态码
    fn http_code(&self)->StatusCode;

}

#[derive(Debug,Clone)]
pub struct ErrPrefix(char);

impl ErrPrefix {
    pub fn new(sign:char)->Self{
        ErrPrefix(sign)
    }
    pub fn into_inner(self)->char{
        self.0
    }
    /// actix 框架产生的异常
    pub const ACTIX:Self=Self('F');
    /// 数据库产生的异常
    pub const SEA_ORM:Self=Self('D');
    /// IO 过程中异常
    pub const IO:Self=Self('I');
    ///  类型钻换时出现的异常
    pub const PARSE:Self=Self('P');
    /// 数据检查时产生的异常
    pub const CHECKER:Self=Self('C');
}



