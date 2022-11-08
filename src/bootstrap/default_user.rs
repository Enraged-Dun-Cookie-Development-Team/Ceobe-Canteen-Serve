use crypto_str::Encoder;
use md5::{Digest, Md5};
use orm_migrate::{
    sql_connection::sea_orm::TransactionTrait,
    sql_models::admin_user::operate::UserSqlOperate,
};
use tracing::log;

use crate::utils::user_authorize::PasswordEncoder;
pub trait FUserConfig {
    fn username(&self) -> String;
    fn password(&self) -> String;
}

pub async fn create_default_user<C>(db: &impl TransactionTrait, conf: &C)
where
    C: FUserConfig,
{
    let password = conf.password();
    let mut md5 = Md5::new();
    md5.update(&password);
    let password = md5.finalize();
    let password = hex::encode(password);
    log::debug!("密码通过MD5加密后是-> {:?}", password);

    // 加密密码
    let encode_password = PasswordEncoder::encode(password.into())
        .expect("初始用户密码加密错误！");

    UserSqlOperate::not_exist_then_create_admin(
        db,
        conf.username(),
        encode_password.to_string(),
    )
    .await
    .expect("储存初始用户失败");

    log::debug!("成功生成默认用户");
}
