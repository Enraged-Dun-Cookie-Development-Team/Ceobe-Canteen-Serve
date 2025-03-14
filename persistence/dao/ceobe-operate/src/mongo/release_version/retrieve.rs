use db_ops_prelude::{
    futures::TryStreamExt,
    mongo_connection::{
        CollectionGuard, MongoDbCollectionTrait, MongoDbError,
    },
    mongo_models::ceobe::operation::version::models::{OId, ReleasePlatform},
    mongodb::{
        bson::{doc, oid::ObjectId, to_bson, Document},
        options::{FindOneOptions, FindOptions},
    },
};
use page_size::request::Paginator;
use tracing::info;

use super::{
    models::{ReleaseVersion, Version},
    Error, ReleaseVersionRetrieve, Result,
};
use crate::release_version::common::generate_release_version_filter;

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

        let filter = generate_release_version_filter(version, &platform)?;

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
            "platform":to_bson(&platform)?,
            "deleted": false
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

    pub async fn all(
        &'db self, platform: Option<ReleasePlatform>,
        paginate: impl Into<Option<Paginator>>, deleted: bool,
    ) -> Result<Vec<ReleaseVersion>> {
        let collection = self.get_collection()?;
        let filter =
            generate_platform_filter_document(platform, deleted, None)?;
        let sort = doc! {
            "$natural": -1i32
        };

        let options = FindOptions::builder().sort(sort);

        let options = if let Some(paginator) = paginate.into() {
            options
                .limit(Some(paginator.limit() as i64))
                .skip(paginator.offset())
                .build()
        }
        else {
            options.build()
        };

        let ret = collection
            .doing(|collection| collection.find(filter, options))
            .await?
            .try_collect()
            .await
            .map_err(MongoDbError::Mongo)?;

        Ok(ret)
    }

    pub async fn total_num(
        &'db self, platform: Option<ReleasePlatform>, deleted: bool,
    ) -> Result<usize> {
        let collection = self.get_collection()?;
        let filter =
            generate_platform_filter_document(platform, deleted, None)?;

        let ret = collection
            .doing(|collection| collection.count_documents(filter, None))
            .await?;

        Ok(ret as _)
    }

    pub async fn all_by_first_id(
        &'db self, platform: Option<ReleasePlatform>,
        first_id: Option<ObjectId>, deleted: bool, page_size: u64,
    ) -> Result<Vec<ReleaseVersion>> {
        let collection = self.get_collection()?;
        let filter =
            generate_platform_filter_document(platform, deleted, first_id)?;
        let sort = doc! {
            "_id": -1i32
        };

        let options = FindOptions::builder()
            .sort(sort)
            .limit(page_size as i64)
            .build();

        let ret = collection
            .doing(|collection| collection.find(filter, options))
            .await?
            .try_collect()
            .await
            .map_err(MongoDbError::Mongo)?;

        Ok(ret)
    }

    pub async fn get_next_id(
        &'db self, platform: Option<ReleasePlatform>,
        first_id: Option<ObjectId>, deleted: bool, page_size: u64,
    ) -> Result<Option<ObjectId>> {
        let collection = self.get_collection()?;
        let collection: &CollectionGuard<OId> = &collection.with_mapping();
        let filter =
            generate_platform_filter_document(platform, deleted, first_id)?;
        let sort = doc! {
            "_id": -1i32
        };

        let options = FindOneOptions::builder()
            .projection(doc! {"_id": 1})
            .sort(sort)
            .skip(page_size)
            .build();

        let oid = collection
            .doing(|collection| collection.find_one(filter, options))
            .await?;

        let ret = oid.map(|id| id._id);

        Ok(ret)
    }
}

fn generate_platform_filter_document(
    platform: Option<ReleasePlatform>, deleted: bool,
    first_id: Option<ObjectId>,
) -> Result<Document> {
    Ok(match (platform, deleted, first_id) {
        (None, false, None) => {
            doc! {
                "deleted": false
            }
        }
        (Some(plat), false, None) => {
            doc! {
                "platform":to_bson(&plat)?,
                "deleted": false
            }
        }
        (None, true, None) => {
            doc! {}
        }
        (Some(plat), true, None) => {
            doc! {
                "platform":to_bson(&plat)?,
            }
        }
        (None, false, Some(oid)) => {
            doc! {
                "deleted": false,
                "_id": {
                    "$lte":oid
                }
            }
        }
        (Some(plat), false, Some(oid)) => {
            doc! {
                "platform":to_bson(&plat)?,
                "deleted": false,
                "_id": {
                    "$lte":oid
                }
            }
        }
        (None, true, Some(oid)) => {
            doc! {
                "_id": {
                    "$lte":oid
                }
            }
        }
        (Some(plat), true, Some(oid)) => {
            doc! {
                "platform":to_bson(&plat)?,
                "_id": {
                    "$lte":oid
                }
            }
        }
    })
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
            ReleasePlatform::{Desktop, Pocket},
            ReleaseVersion, Version,
        },
        mongodb::bson::doc,
    };
    use mongo_migration::Migrator;

    use crate::{
        release_version::retrieve::generate_platform_filter_document,
        ToCeobeOperation,
    };

    #[test]
    fn test_deleted_filter() {
        let doc =
            generate_platform_filter_document(Some(Desktop), false, None)
                .expect("Err");
        assert_eq!(doc, doc! {"platform": "desktop","deleted":false});

        let doc =
            generate_platform_filter_document(Some(Desktop), true, None)
                .expect("err");
        assert_eq!(doc, doc! {"platform": "desktop"})
    }

    #[tokio::test]
    async fn test_retrieve_version() {
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
            .previous_mandatory_version(Version::new(1, 0, 0))
            .platform(Pocket)
            .build();
        let v2_1 = ReleaseVersion::builder()
            .version(Version::new(2, 1, 0))
            .previous_mandatory_version(Version::new(1, 0, 0))
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
