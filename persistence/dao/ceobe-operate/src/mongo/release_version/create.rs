use db_ops_prelude::{
    mongo_connection::MongoDbCollectionTrait,
    mongodb::{
        bson::{doc, to_bson},
        options::FindOneOptions,
    },
};
use serde::{Deserialize, Serialize};

use super::{
    models::{ReleaseVersion, Version},
    Error, ReleaseVersionCreate, Result,
};

impl<'db, Conn> ReleaseVersionCreate<'db, Conn>
where
    Conn: MongoDbCollectionTrait<'db, ReleaseVersion>,
{
    pub async fn one(
        &'db self, release_version: impl Into<ReleaseVersion>,
    ) -> Result<()> {
        #[derive(Debug, Deserialize, Serialize)]
        struct VersionOnly {
            version: Version,
        }
        let release_version = release_version.into();

        let collection = self.get_collection()?;

        // 找到当前平台的最新的发布版本，与当前添加版本比较。新版本必须更新
        let filter = doc! {"platform":to_bson(&release_version.platform)?};
        let exist_latest_version = collection
            .with_mapping::<VersionOnly>()
            .doing(|collection| {
                collection.find_one(
                    filter,
                    FindOneOptions::builder()
                        .sort(doc! {"$natural": -1i32})
                        .projection(doc! {"version":1i32})
                        .build(),
                )
            })
            .await?;
        if let Some(VersionOnly { version }) = exist_latest_version {
            if release_version.version <= version {
                Err(Error::VersionTooOld(version, release_version.platform))?;
            }
        }

        collection
            .doing(|collection| collection.insert_one(release_version, None))
            .await?;
        Ok(())
    }
}
#[cfg(test)]
mod test {
    use abstract_database::ceobe::ToCeobe;
    use db_ops_prelude::{
        database_operates::{operate_trait::OperateTrait, DatabaseOperate},
        mongo_connection::{
            database_traits::initial::connect_db_with_migrate,
            get_mongo_collection, DatabaseManage, MongoConnect,
            MongoDbConfig,
        },
        mongo_models::ceobe::operation::version::models::{
            ForceCtrl, ReleasePlatform::Pocket, ReleaseVersion, Version,
        },
        mongodb::bson::doc,
    };
    use mongo_migration::Migrator;
    use serde::{Deserialize, Serialize};

    use crate::ToCeobeOperation;

    #[tokio::test]
    async fn test_create_order() {
        #[derive(Debug, Deserialize, Serialize)]
        struct VersionOnly {
            version: Version,
        }

        connect_db_with_migrate::<DatabaseManage, _, _>(
            &MongoDbConfig::default(),
            Migrator,
        )
        .await
        .expect("connect to db Error");

        let conn = DatabaseOperate::new(MongoConnect);
        let collection = get_mongo_collection::<ReleaseVersion>()
            .expect("Collection Not Exist");

        // 开始测试前，清空现有的collection内容
        collection
            .delete_many(doc! {}, None)
            .await
            .expect("clear Collection Error");

        // 插入一些版本号，按照从小到大，为了方便，这里只插入版本号
        for major in 1..=5 {
            let version = Version::new(major, 0, 0);
            collection
                .with_mapping::<VersionOnly>()
                .insert_one(&VersionOnly { version }, None)
                .await
                .expect("insert Error");
        }

        // 现在要插入一些新版本

        // 添加6.0版本，这个是合法的，因为最新的版本是5.0
        let ret = conn
            .ceobe()
            .operation()
            .release_version()
            .create()
            .one(
                ReleaseVersion::builder()
                    .version(Version::new(6, 0, 0))
                    .force(
                        ForceCtrl::builder()
                            .previous_force_version(Version::new(1, 0, 0))
                            .build(),
                    )
                    .platform(Pocket)
                    .build(),
            )
            .await;

        assert!(ret.is_ok());

        // 添加5.8.1 版本，这个是非法的，因为最新版本是6.0，必须大于6.0
        let ret = conn
            .ceobe()
            .operation()
            .release_version()
            .create()
            .one(
                ReleaseVersion::builder()
                    .version(Version::new(5, 8, 1))
                    .force(
                        ForceCtrl::builder()
                            .previous_force_version(Version::new(1, 0, 0))
                            .build(),
                    )
                    .platform(Pocket)
                    .build(),
            )
            .await;

        // 返回Err, 同时Err类型为VersionTooOld
        assert!(ret.is_err());
        assert_eq!(
            ret.map_err(|e| e.to_string()),
            Err("新版本的版本号过旧, 新版本号应大于 6.0.0".to_string())
        );
    }
}
