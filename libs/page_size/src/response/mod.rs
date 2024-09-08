use std::ops::Deref;

use serde::{ser::SerializeStruct, Serialize};

use crate::request::Paginator;

#[derive(Serialize, Debug)]
pub struct PageInfo {
    #[serde(flatten)]
    page_size: Paginator,
    total_count: u64,
    total_page: u64,
}

/// 列表与分页信息
#[derive(Debug)]
pub struct ListWithPageInfo<T>
where
    T: Serialize,
{
    list: Vec<T>,
    page_size: Option<PageInfo>,
}

impl<T: Serialize> Serialize for ListWithPageInfo<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.page_size {
            Some(page) => {
                let mut struc =
                    serializer.serialize_struct("ListWithPageInfo", 2)?;
                struc.serialize_field("list", &self.list)?;
                struc.serialize_field("page_size", page)?;
                struc.end()
            }
            None => <Vec<T> as Serialize>::serialize(&self.list, serializer),
        }
    }
}

pub trait GenerateListWithPageInfo: IntoIterator
where
    Self::Item: Serialize,
{
    fn with_page_info(
        self, page_size: Paginator, count: u64,
    ) -> ListWithPageInfo<Self::Item>;

    fn with_plain(self) -> ListWithPageInfo<Self::Item>;
}

impl<T> GenerateListWithPageInfo for T
where
    T: IntoIterator,
    T::Item: Serialize,
{
    /// 将列表，与分页信息存入一个结构体
    fn with_page_info(
        self, page_size: Paginator, count: u64,
    ) -> ListWithPageInfo<Self::Item> {
        ListWithPageInfo {
            list: self.into_iter().collect(),
            page_size: Some(PageInfo {
                page_size,
                total_count: count,
                total_page: (count as f64 / *page_size.size.deref() as f64)
                    .ceil() as u64,
            }),
        }
    }

    fn with_plain(self) -> ListWithPageInfo<Self::Item> {
        ListWithPageInfo {
            list: self.into_iter().collect(),
            page_size: None,
        }
    }
}

#[cfg(test)]
mod test {
    use checker::{prefabs::num_check::NonZeroUnsignedChecker, Checker};
    use serde_json::json;

    use super::GenerateListWithPageInfo;
    use crate::request::Paginator;
    #[test]
    fn test_serde_paginator() {
        let a = Some("AAA");
        let page = a.with_page_info(
            Paginator::builder()
                .page(
                    NonZeroUnsignedChecker::check(Default::default(), 11)
                        .into_inner()
                        .unwrap(),
                )
                .size(
                    NonZeroUnsignedChecker::check(Default::default(), 2)
                        .into_inner()
                        .unwrap(),
                )
                .build(),
            25,
        );

        let value = serde_json::to_value(&page).unwrap();

        assert_eq!(
            value,
            json!({
                "list":["AAA"],
                "page_size":{

                    "page":11,
                    "size":2,
                    "total_count":25,
                    "total_page":13

                }
            })
        );

        let no_page = Some("AAA").with_plain();
        let value = serde_json::to_value(&no_page).unwrap();
        assert_eq!(value, json!(["AAA"]))
    }
}
