fn main() {
    let e = TestErr::Else {
        start: String::from("Abc"),
    };

    println!("{}", e.information());
    println!("{}", e.respond_msg())
}
use status_err::{ErrPrefix, HttpCode, StatusErr};
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
}
