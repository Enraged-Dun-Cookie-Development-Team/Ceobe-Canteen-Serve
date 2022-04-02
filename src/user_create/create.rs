use crypto::digest::Digest;
use crypto_str::Encoder;
use db_entity::sea_orm_active_enums::Auth;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, QuerySelect,
    Set,
};

use super::default_user::FUserConfig;
use crate::{
    database::ServeDatabase, utils::user_authorize::PasswordEncoder,
};

#[derive(FromQueryResult)]
struct UserCounts {
    count: i64,
}

pub async fn create_default_user<C>(conf: &C, db: &ServeDatabase)
where
    C: FUserConfig,
{
    let user_counts = db_entity::user::Entity::find()
        .column_as(db_entity::user::Column::Id.count(), "count")
        .into_model::<UserCounts>()
        .one(db)
        .await
        .expect("查询初始数据库User表失败")
        .unwrap()
        .count
        == 0;

    if user_counts {
        log::debug!("无用户，生成默认用户");
        let password = conf.password();
        let mut md5 = crypto::md5::Md5::new();
        md5.input_str(&password);
        let password = md5.result_str();
        log::debug!("密码通过MD5加密后是： {:?}", password);

        // 加密密码
        let encode_password = PasswordEncoder::encode(password.into())
            .expect("初始用户密码加密错误！");

        let default_user = db_entity::user::ActiveModel {
            username: Set(conf.username()),
            password: Set(encode_password.to_string()),
            auth: Set(Auth::Chef),
            ..Default::default()
        };

        default_user.save(db).await.expect("储存初始用户失败");
        log::debug!("成功生成默认用户");
    }
}
