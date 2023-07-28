use sea_orm::{DbErr, FromQueryResult, QueryResult};

pub mod active_or_set;
pub mod check_all_exist;
pub mod select_count;
pub mod soft_delete;

const COUNT_NAME: &str = "count";

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Count(pub u64);

impl Count {
    pub fn is_empty(&self) -> bool { self == 0u64 }
}

impl PartialEq<u64> for Count {
    fn eq(&self, other: &u64) -> bool { &self.0 == other }
}

impl PartialEq<u64> for &Count {
    fn eq(&self, other: &u64) -> bool { &self.0 == other }
}

impl FromQueryResult for Count {
    fn from_query_result(
        res: &QueryResult, pre: &str,
    ) -> Result<Self, DbErr> {
        let count: i64 = res.try_get(pre, COUNT_NAME)?;
        Ok(Self(count as u64))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct CountZero(pub bool);

impl CountZero {
    pub fn take(self) -> bool { self.0 }
}

impl FromQueryResult for CountZero {
    fn from_query_result(
        res: &QueryResult, pre: &str,
    ) -> Result<Self, DbErr> {
        Ok(CountZero(Count::from_query_result(res, pre)? == 0))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct CountNonZero(pub bool);

impl FromQueryResult for CountNonZero {
    fn from_query_result(
        res: &QueryResult, pre: &str,
    ) -> Result<Self, DbErr> {
        Ok(CountNonZero(Count::from_query_result(res, pre)? != 0))
    }
}

impl CountNonZero {
    pub fn take(self) -> bool { self.0 }
}
