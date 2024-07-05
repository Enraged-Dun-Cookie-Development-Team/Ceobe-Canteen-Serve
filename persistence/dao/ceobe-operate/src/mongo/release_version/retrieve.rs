use tracing::info;

use db_ops_prelude::{
    mongo_connection::MongoDbCollectionTrait,
    mongo_models::ceobe::operation::version::models::ReleasePlatform,
    mongodb::{
        bson::{doc, to_bson},
        options::FindOneOptions,
    },
};

use super::{
    Error,
    models::{ReleaseVersion, Version}, ReleaseVersionRetrieve, Result,
};

impl<'db, Conn> ReleaseVersionRetrieve<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, ReleaseVersion>,
{
    /// 根据指定版本号和平台返回对应的版本信息
    pub async fn by_version_platform(
        &'db self, version: &Version, platform: ReleasePlatform,
    ) -> Result<ReleaseVersion> {
        info!(release.version = %version, release.platform = ?platform);
        let collection = self.get_collection()?;

        let filter = doc! {
            "version":to_bson(version)?,
            "platform":to_bson(&platform)?
        };

        let ret = collection
            .doing(|collection| collection.find_one(filter, None))
            .await?
            .ok_or_else(|| {
                Error::VersionNotFind(version.to_owned(), platform)
            })?;
        Ok(ret)
    }

    /// 根据平台返回最新的版本信息
    /// FIXME: 这里直接取最新的一个，理论上没问题/测试上也没问题
    pub async fn latest_by_platform(
        &'db self, platform: ReleasePlatform,
    ) -> Result<ReleaseVersion> {
        info!( release.platform = ?platform);
        let collection = self.get_collection()?;
        let filter = doc! {
            "platform":to_bson(&platform)?
        };
        let sort = doc! {
            "$natural": -1i32
        };

        let ret = collection
            .doing(|collection| {
                collection.find_one(
                    filter,
                    FindOneOptions::builder().sort(sort).build(),
                )
            })
            .await?
            .ok_or(Error::VersionInfoNoExist)?;

        Ok(ret)
    }
}

#[cfg(test)]
mod test {
    use abstract_database::ceobe::ToCeobe;
    use db_ops_prelude::{
        database_operates::{DatabaseOperate, operate_trait::OperateTrait},
        mongo_connection::{
            database_traits::initial::connect_db_with_migrate,
            DatabaseManage, get_mongo_collection, MongoConnect,
            MongoDbConfig,
        },
        mongo_models::ceobe::operation::version::models::{
            ForceCtrl,
            ReleasePlatform::{Desktop, Pocket},
            ReleaseVersion, Version,
        },
        mongodb::bson::doc,
    };
    use mongo_migration::Migrator;

    use crate::ToCeobeOperation;

    #[tokio::test]
    async fn test_retrieve_version() {

        connect_db_with_migrate::<DatabaseManage, _, _>(
            &MongoDbConfig::default(),
            Migrator,
        )
        .await
        .expect("connect to db Error");

        let conn = DatabaseOperate::test_new(MongoConnect);
        let collection = get_mongo_collection::<ReleaseVersion>()
            .expect("Collection Not Exist");

        // 开始测试前，清空现有的collection内容
        collection
            .delete_many(doc! {}, None)
            .await
            .expect("clear Collection Error");

        // 当前无任何版本，最新版本返回空
        let latest = conn
            .ceobe()
            .operation()
            .release_version()
            .retrieve()
            .latest_by_platform(Pocket)
            .await;

        assert_eq!(
            latest.map_err(|e| e.to_string()),
            Err("暂没有版本信息".into())
        );

        // 现在要插入一些新版本
        let v1 = ReleaseVersion::builder()
            .version(Version::new(1, 0, 0))
            .force(
                ForceCtrl::builder()
                    .previous_force_version(Version::new(1, 0, 0))
                    .force_update()
                    .build(),
            )
            .platform(Pocket)
            .build();
        let v2_1 = ReleaseVersion::builder()
            .version(Version::new(2, 1, 0))
            .force(
                ForceCtrl::builder()
                    .previous_force_version(Version::new(1, 0, 0))
                    .build(),
            )
            .platform(Pocket)
            .build();

        // 添加1.0版本
        conn.ceobe()
            .operation()
            .release_version()
            .create()
            .one(v1.clone())
            .await
            .unwrap();

        // 添加2.1 版本
        conn.ceobe()
            .operation()
            .release_version()
            .create()
            .one(v2_1.clone())
            .await
            .unwrap();

        // 现在获得最新版本，得到2.1
        let latest = conn
            .ceobe()
            .operation()
            .release_version()
            .retrieve()
            .latest_by_platform(Pocket)
            .await;
        assert!(latest.is_ok());
        assert_eq!(latest.unwrap(), v2_1);

        // 指定版本1.0，获得1.0
        let get_v1 = conn
            .ceobe()
            .operation()
            .release_version()
            .retrieve()
            .by_version_platform(&Version::new(1, 0, 0), Pocket)
            .await;
        assert!(get_v1.is_ok());
        assert_eq!(get_v1.unwrap(), v1);

        // 指定一个不存在的版本，返回Err
        let version_not_exist = conn
            .ceobe()
            .operation()
            .release_version()
            .retrieve()
            .by_version_platform(&Version::new(1, 0, 0), Desktop)
            .await;

        assert_eq!(
            version_not_exist.map_err(|e| e.to_string()),
            Err("版本信息不存在 Desktop:1.0.0".into())
        )
    }
}
