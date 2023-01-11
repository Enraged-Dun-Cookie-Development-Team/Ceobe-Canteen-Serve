use std::ops::Deref;

use sea_orm::{EntityTrait, QuerySelect, Select};

use crate::request::Paginator;

pub trait WithPagination {
    fn with_pagination(self, page_size: Paginator) -> Self;
}

impl<E: EntityTrait> WithPagination for Select<E> {
    /// 添加分页查询
    fn with_pagination(self, Paginator { page, size }: Paginator) -> Self {
        self.offset(((page.deref() - 1) * size.deref()) as u64)
            .limit((page.deref() * size.deref()) as u64)
    }
}
