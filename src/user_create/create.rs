use crypto::digest::Digest;
use crypto_str::Encoder;
use db_entity::sea_orm_active_enums::Auth;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, QuerySelect,
    Set,
};
use time_usage::{async_time_usage_with_name, sync_time_usage_with_name};

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
    let user_counts = async_time_usage_with_name(
        "检查是否需要创建基本用户",
        db_entity::user::Entity::find()
            .column_as(db_entity::user::Column::Id.count(), "count")
            .into_model::<UserCounts>()
            .one(db),
    )
    .await
    .expect("查询初始数据库User表失败")
    .unwrap()
    .count
        == 0;

    if user_counts {
        let default_user =
            sync_time_usage_with_name("生成默认用户信息", || {
                log::debug!("无用户，生成默认用户");
                let password = conf.password();
                let mut md5 = crypto::md5::Md5::new();
                md5.input_str(&password);
                let password = md5.result_str();
                log::debug!("密码通过MD5加密后是： {:?}", password);

                // 加密密码
                let encode_password =
                    PasswordEncoder::encode(password.into())
                        .expect("初始用户密码加密错误！");

                db_entity::user::ActiveModel {
                    username: Set(conf.username()),
                    password: Set(encode_password.to_string()),
                    auth: Set(Auth::Chef),
                    ..Default::default()
                }
            });

        async_time_usage_with_name(
            "保存生成的默认用户信息",
            default_user.save(db),
        )
        .await
        .expect("储存初始用户失败");
        log::debug!("成功生成默认用户");
    }
}
