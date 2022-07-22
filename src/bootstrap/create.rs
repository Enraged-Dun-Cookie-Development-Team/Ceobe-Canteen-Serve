use crypto::{digest::Digest, md5::Md5};
use crypto_str::Encoder;
use orm_migrate::sql_models::user::operate::UserSqlOperate;

use super::default_user::FUserConfig;
use crate::utils::user_authorize::PasswordEncoder;

pub async fn create_default_user<C>(conf: &C)
where
    C: FUserConfig,
{
    let password = conf.password();
    let mut md5 = Md5::new();
    md5.input_str(&password);
    let password = md5.result_str();
    log::debug!("密码通过MD5加密后是-> {:?}", password);

    // 加密密码
    let encode_password = PasswordEncoder::encode(password.into())
        .expect("初始用户密码加密错误！");

    UserSqlOperate::not_exist_then_create_admin(
        conf.username(),
        encode_password.to_string(),
    )
    .await
    .expect("储存初始用户失败");

    log::debug!("成功生成默认用户");
}
