use sea_orm::{Select, EntityTrait, QuerySelect};

pub trait OffsetLimit {
    fn offset_limit(self, page: usize, size: usize) -> Self;
}

impl<E: EntityTrait> OffsetLimit for Select<E> {
    fn offset_limit(self, page: usize, size: usize) -> Self {
        self.offset(((page-1)*size) as u64).limit((page*size) as u64)
    }
}
