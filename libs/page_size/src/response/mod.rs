use std::ops::Deref;

use serde::Serialize;

use crate::request::PageSize;

#[derive(Serialize, Debug)]
pub struct PageInfo {
    #[serde(flatten)]
    page_size: PageSize,
    total_count: u64,
    total_page: u64,
}

/// 列表与分页信息
#[derive(Serialize, Debug)]
pub struct ListWithPageInfo<T>
where
    T: Serialize,
{
    list: Vec<T>,
    page_size: PageInfo,
}

pub trait GenerateListWithPageInfo: IntoIterator
where
    Self::Item: Serialize,
{
    fn with_page_info(
        self, page_size: PageSize, count: u64,
    ) -> ListWithPageInfo<Self::Item>;
}

impl<T> GenerateListWithPageInfo for T
where
    T: IntoIterator,
    T::Item: Serialize,
{
    /// 将列表，与分页信息存入一个结构体
    fn with_page_info(
        self, page_size: PageSize, count: u64,
    ) -> ListWithPageInfo<Self::Item> {
        ListWithPageInfo {
            list: self.into_iter().collect(),
            page_size: PageInfo {
                page_size,
                total_count: count,
                total_page: (count as f64 / *page_size.size.deref() as f64)
                    .ceil() as u64,
            },
        }
    }
}
