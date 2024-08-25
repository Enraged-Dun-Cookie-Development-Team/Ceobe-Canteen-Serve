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
            .limit(*size.deref() as u64)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Paginate {
    pub offset: u64,
    pub limit: u64,
}

impl Paginator {
    pub fn offset(&self) -> u64 {
        ((self.page.deref() - 1) * self.size.deref()) as u64
    }

    pub fn limit(&self) -> u64 { *self.size.deref() as u64 }

    pub fn to_paginate(&self) -> Paginate {
        Paginate {
            offset: self.offset(),
            limit: self.limit(),
        }
    }
}
