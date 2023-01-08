use std::ops::Deref;

use sea_orm::{EntityTrait, QuerySelect, Select};

use crate::request::PageSize;

pub trait OffsetLimit {
    fn with_pagination(self, page_size: PageSize) -> Self;
}

impl<E: EntityTrait> OffsetLimit for Select<E> {
    /// 添加分页查询
    fn with_pagination(self, PageSize { page, size }: PageSize) -> Self {
        self.offset(((page.deref() - 1) * size.deref()) as u64)
            .limit((page.deref() * size.deref()) as u64)
    }
}
