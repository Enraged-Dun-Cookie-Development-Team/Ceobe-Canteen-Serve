use sea_orm::{Select, EntityTrait, QuerySelect};

use crate::request::PageSize;

pub trait OffsetLimit {
    fn offset_limit(self, page_size: PageSize) -> Self;
}

impl<E: EntityTrait> OffsetLimit for Select<E> {
    fn offset_limit(self, PageSize{page, size}: PageSize) -> Self {
        self.offset(((page-1)*size) as u64).limit((page*size) as u64)
    }
}