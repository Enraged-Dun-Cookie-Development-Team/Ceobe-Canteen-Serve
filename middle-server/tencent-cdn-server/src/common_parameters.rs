use chrono::Utc;
use typed_builder::TypedBuilder;



#[derive(Debug, Clone, TypedBuilder)]
pub struct CommonParameter {
    pub service: String,
    pub version: String,
    pub action: String,
    pub region: Option<String>,
    #[builder(default = String::from("TC3-HMAC-SHA256"))]
    pub algorithm: String,
    #[builder(default = Utc::now().timestamp())]
    pub timestamp: i64,
}
