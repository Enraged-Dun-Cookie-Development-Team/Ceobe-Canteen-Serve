use sea_orm::{Select, EntityTrait, QuerySelect};

use crate::request::PageSize;

pub trait OffsetLimit {
    fn offset_limit(self, page_size: PageSize) -> Self;
}

impl<E: EntityTrait> OffsetLimit for Select<E> {
    fn offset_limit(self, page_size: PageSize) -> Self {
        self.offset(((page_size.page-1)*page_size.size) as u64).limit((page_size.page*page_size.size) as u64)
    }
}
