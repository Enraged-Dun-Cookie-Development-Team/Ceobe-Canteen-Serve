use sea_orm::FromQueryResult;

pub mod check_all_exist;
pub mod select_count;
const COUNT_NAME: &str = "count";

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Count(pub u64);

impl Count {
    pub fn is_empty(&self) -> bool {
        self == 0u64
    }
}

impl PartialEq<u64> for Count {
    fn eq(&self, other: &u64) -> bool {
        &self.0 == other
    }
}

impl PartialEq<u64> for &Count {
    fn eq(&self, other: &u64) -> bool {
        &self.0 == other
    }
}

impl FromQueryResult for Count {
    fn from_query_result(
        res: &sea_orm::QueryResult, pre: &str,
    ) -> Result<Self, sea_orm::DbErr> {
        let count = res.try_get(pre, COUNT_NAME)?;
        Ok(Self(count))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct CountZero(pub bool);

impl CountZero {
    pub fn take(self) -> bool {
        self.0
    }
}

impl FromQueryResult for CountZero {
    fn from_query_result(
        res: &sea_orm::QueryResult, pre: &str,
    ) -> Result<Self, sea_orm::DbErr> {
        Ok(CountZero(Count::from_query_result(res, pre)? == 0))
    }
}
