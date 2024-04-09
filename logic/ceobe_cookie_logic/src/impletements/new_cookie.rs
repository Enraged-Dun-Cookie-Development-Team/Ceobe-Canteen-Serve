use crate::{
    error::{LogicError, LogicResult},
    impletements::CeobeCookieLogic,
    view::{CombIdToCookieIdReq, NewCookieReq, PushInfo},
};
use ceobe_qiniu_upload::QiniuManager;
use futures::future;
use mob_push_server::PushManager;
use persistence::operate::GetMutDatabaseConnect;
use persistence::{
    ceobe_cookie::ToCeobe,
    ceobe_user::ToCeobeUser,
    fetcher::{
        datasource_combination::DatasourceCombinationOperate,
        datasource_config::DatasourceOperate,
    },
    mongodb::{mongodb::bson::oid::ObjectId, MongoDatabaseOperate},
    mysql::SqlDatabaseOperate,
    operate::GetDatabaseConnect,
    redis::RedisConnect,
};
use qiniu_service::model::DeleteObjectName;
use qq_channel_warning::{LogRequest, LogType, QqChannelGrpcService};
use redis::AsyncCommands;
use redis_global::redis_key::{concat_key, cookie_list::CookieListKey};
use tokio::task::JoinHandle;


impl CeobeCookieLogic {
    /// 分析器新饼通知Rust端接口
    ///     1. 从mysql获取数据源相关的数据源组合
    ///     2. 获取分析器传来饼列表第一个饼
    ///     3. 同时做mob推送 与 redis+七牛云缓存操作
    pub async fn new_cookie(
        mongo: MongoDatabaseOperate, sql: SqlDatabaseOperate,
        mut redis_client: RedisConnect, mut mob: PushManager,
        qq_channel: QqChannelGrpcService, qiniu: QiniuManager,
        new_cookies: Vec<NewCookieReq>,
    ) -> LogicResult<()> {
        // 处理数组为空的情况
        if new_cookies.is_empty() {
            return Ok(());
        }
        let db = sql.get_connect();
        let source = &new_cookies.first().unwrap().source;
        // 查询数据源相关信息
        let datasource_info =
            DatasourceOperate::find_model_by_datasource_and_unique_key(
                db,
                &source.datasource,
                &source.unique,
            )
            .await?;
        let mut qq_channel_tmp = qq_channel.clone();
        // 最新的一个饼
        let newest_cookie_id =
            new_cookies.last().map(|cookie| cookie.cookie_id);

        // redis缓存操作
        let comb_ids =
            DatasourceCombinationOperate::find_comb_id_by_one_datasource_raw(
                db,
                datasource_info.id,
            )
            .await?;

        // 更新最新饼id对象储存
        Self::cache_cookie_redis(
            &mut redis_client,
            newest_cookie_id,
            newest_cookie_id,
            comb_ids.clone(),
            datasource_info.to_combin_id(),
        )
        .await?;

        let (datasource_error, qiniu_err): (_, Result<(), LogicError>) =
            future::join(
                async {
                    // 查询用户列表
                    let result = mongo
                        .ceobe()
                        .user()
                        .property()
                        .find_user_list_by_datasource(
                            datasource_info.unique_id.into(),
                        )
                        .await;
                    match result {
                        Ok(user_list) => {
                            for new_cookie in new_cookies {
                                // mob推送新饼
                                let content = PushInfo::builder()
                                    .content(new_cookie.content.text)
                                    .datasource_name(
                                        datasource_info.nickname.clone(),
                                    )
                                    .image_url(new_cookie.content.image_url)
                                    .icon_url(datasource_info.avatar.clone())
                                    .build();

                                if let Err(err) = mob
                                    .mob_push::<_, String, _>(
                                        &content, &user_list,
                                    )
                                    .await
                                {
                                    qq_channel_tmp
                                        .send_logger(
                                            LogRequest::builder()
                                                .level(LogType::Error)
                                                .info(
                                                    "推送新饼失败"
                                                        .to_string(),
                                                )
                                                .extra(format!("报错：{err}"))
                                                .build(),
                                        )
                                        .await?;
                                }
                            }

                            Ok(())
                        }
                        Err(err) => Err(LogicError::from(err)),
                    }
                },
                {
                    async move {
                        // 删除对象储存中的数据源组合文件
                        qiniu
                            .delete_many(
                                comb_ids
                                    .into_iter()
                                    .map(DeleteObjectName::new)
                                    .collect(),
                            )
                            .await
                            .map_err(LogicError::from)?;
                        Ok(())
                    }
                },
            )
            .await;

        datasource_error?;
        qiniu_err?;

        Ok(())
    }

    /// 缓存饼id信息到redis
    ///     1. 并发更新NEW_COMBID_INFO redis表 
    ///         - 判断NEW_COMBID_INFO的combid field是否存在。
    ///         - 如果存在，需要判断当前饼id和数据库里饼id那个大。接口cookie_id要取大的那个（最新）【这边没办法批量操作的原因就是因为每个得单独判断，每个下面数据源是不一样的】
    ///         - 写入NEW_COMBID_INFO的对应combid的值，cookie_id为最新，update_cookie_id为当前传入
    ///     2. 更新update_cookie_id缓存，这个缓存是提供官方绕过cdn而设计，因为列表cdn设计2小时缓存，所以被换下的id也是设置2小时ttl缓存
    ///         - 表介绍
    ///             - NEW_UPDATE_COOKIES: hash, 储存最新的更新饼id
    ///             - NEW_UPDATE_COOKIE_ID: string，给更新饼id判断存不存在的，可以让查询时候列表命中缓存
    ///         - 过程
    ///             - 在NEW_UPDATE_COOKIE_ID表中，设置传入的更新饼id，不设置ttl（过期时间）
    ///             - 在NEW_UPDATE_COOKIES表的combid field中取出更新饼id，如果有才会做接下来操作，使用取出的更新饼id，在NEW_UPDATE_COOKIE_ID表中设置2小时缓存。
    ///             - 将当前传入更新饼id赋值到NEW_UPDATE_COOKIES表的combid field中，作为最新的更新饼id记录
    async fn cache_cookie_redis(
        redis_client: &mut RedisConnect, cookie_id: Option<ObjectId>,
        update_cookie_id: Option<ObjectId>, comb_ids: Vec<String>,
        datasource: Option<String>,
    ) -> LogicResult<()> {
            let mut handles =
                Vec::<JoinHandle<Result<(), LogicError>>>::new();
            for comb_id in comb_ids {
                let mut redis_client_temp = redis_client.clone();
                handles.push(tokio::spawn(async move {
                    let redis = redis_client_temp.mut_connect();
                    // 如果传入cookie_id和redis都有信息
                    let cookie_id = if let (
                        Some(mut newest_cookie_id),
                        true,
                    ) = (
                        cookie_id,
                        redis
                            .hexists(CookieListKey::NEW_COMBID_INFO, &comb_id)
                            .await?,
                    ) {
                        let last_comb_info: CombIdToCookieIdReq =
                            serde_json::from_str(
                                &redis
                                    .hget::<'_, _, _, String>(
                                        CookieListKey::NEW_COMBID_INFO,
                                        &comb_id,
                                    )
                                    .await?,
                            )?;
                        // 这边一定保证redis这个hash field存在就有这个值。 结构体中Option只是为了兼容接口返回结构
                        let last_cookie_id = last_comb_info
                            .cookie_id
                            .unwrap()
                            .parse()?;
                        // 判断数据库和传入的cookie_id哪个新，用新的那个id
                        newest_cookie_id =
                            newest_cookie_id.max(last_cookie_id);
                        Some(newest_cookie_id.to_string())
                    } else {
                        cookie_id.map(|id| id.to_string())
                    };

                    if cookie_id.is_some() {
                        let comb_info = CombIdToCookieIdReq {
                            cookie_id,
                            update_cookie_id: update_cookie_id
                                .map(|id| id.to_string()),
                        };
                        // 接口信息写入redis，等待七牛云回源
                        redis
                            .hset(
                                CookieListKey::NEW_COMBID_INFO,
                                &comb_id,
                                serde_json::to_string(&comb_info)?,
                            )
                            .await?;
                    }
                    Ok(())
                }));
            }
            futures::future::join_all(handles).await;
        

        
        let redis = redis_client.mut_connect();
        if let Some(update_id) = update_cookie_id {
            // 更新[更新最新饼id]到redis
            redis
                .set_nx(
                    concat_key(
                        CookieListKey::NEW_UPDATE_COOKIE_ID,
                        &update_id.to_string(),
                    ),
                    true,
                )
                .await?;

            if redis
                .hexists(CookieListKey::NEW_UPDATE_COOKIES, &datasource)
                .await?
            {
                // 从hash update_cookie表获取上一个的update_cookie_id
                let update_cookie: String = redis
                    .hget(CookieListKey::NEW_UPDATE_COOKIES, &datasource)
                    .await?;
                if update_id.to_string() != update_cookie {
                    // 对已经被替换下的饼id设置ttl，2小时
                    redis
                        .set_ex(
                            concat_key(
                                CookieListKey::NEW_UPDATE_COOKIE_ID,
                                &update_cookie,
                            ),
                            true,
                            2 * 60 * 60,
                        )
                        .await?;
                }
            }
            // 对hash update_cookie表写入最新的更新饼id
            redis
                .hset(
                    CookieListKey::NEW_UPDATE_COOKIES,
                    &datasource,
                    &update_id.to_string(),
                )
                .await?;
        }
        Ok(())
    }
}
