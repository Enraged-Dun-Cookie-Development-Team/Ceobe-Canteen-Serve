use std::ops::Deref;

use sea_orm::{EntityTrait, QuerySelect, Select};

use crate::request::PageSize;

pub trait OffsetLimit {
    fn offset_limit(self, page_size: PageSize) -> Self;
}

impl<E: EntityTrait> OffsetLimit for Select<E> {
    fn offset_limit(self, PageSize { page, size }: PageSize) -> Self {
        self.offset(((page.deref() - 1) * size.deref()) as u64)
            .limit((page.deref() * size.deref()) as u64)
    }
}
