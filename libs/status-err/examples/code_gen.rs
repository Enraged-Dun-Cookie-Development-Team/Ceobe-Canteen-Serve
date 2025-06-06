use status_err::{ErrPrefix, HttpCode, StatusErr};
use status_err::generated_error::haaaa_kind::HumError;
fn main() {
    let e = TestErr::Else {
        start: String::from("Abc"),
    };

    println!("{}", StatusErr::code(&e));
    println!("{}", StatusErr::prefix(&e));
    
    let e = TestErr::Binding;
    
    println!("{}{:04x}",StatusErr::prefix(&e),StatusErr::code(&e));
}

#[derive(Debug, status_err::ThisError, status_err::StatusErr)]
#[status_err(resp_err)]
pub enum TestErr {
    #[error("UTF8 编码解析异常 {0}")]
    Parse(#[from] std::string::FromUtf8Error),

    #[error("其他异常 {start:?}")]
    #[status_err(err(
        resp_msg = "其他异常",
        err_code = 12,
        prefix = "ErrPrefix::CHECKER",
        http_code = "HttpCode::NOT_FOUND"
    ))]
    Else { start: String },
    #[error("binding Error")]
    #[status_err(err(bind = "HumError"))]
    Binding
}
