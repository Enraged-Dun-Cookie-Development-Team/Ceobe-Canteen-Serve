use sea_orm::{
    sea_query::{Expr, SimpleExpr},
    ColumnTrait, EntityTrait, FromQueryResult, QuerySelect, Select,
    SelectModel, Selector,
};

use super::{Count, CountNonZero, CountZero, COUNT_NAME};

/// 计数指定列的
pub trait QueryCountByColumn: Sized {
    type Selector<Count: FromQueryResult>;
    type Select;

    /// 只修改select 但是不映射模型 （可以用于测试生成的SQL）
    fn select_count_by_colum<C>(self, col: C) -> Self::Select
    where
        ColumnExpr: From<C>;

    /// 修改select 为对指定col count 并映射为指定模型
    fn count_by_column<C, Count>(self, col: C) -> Self::Selector<Count>
    where
        ColumnExpr: From<C>,
        Count: FromQueryResult;

    /// 对指定的col 进行计数，并与0比较
    /// count == 0 ? 即指定列的记录为 0
    fn count_zero_by_column<C>(self, col: C) -> Self::Selector<CountZero>
    where
        ColumnExpr: From<C>,
    {
        self.count_by_column(col)
    }
    /// 对指定的col 进行计数，并与0比较
    /// count != 0 ? 即指定的列计数为 非 0
    fn count_non_zero_by_column<C>(
        self, col: C,
    ) -> Self::Selector<CountNonZero>
    where
        ColumnExpr: From<C>,
    {
        self.count_by_column(col)
    }

    fn count_only_by_column<C>(self, col: C) -> Self::Selector<Count>
    where
        ColumnExpr: From<C>,
    {
        self.count_by_column(col)
    }
}

impl<E: EntityTrait> QueryCountByColumn for Select<E> {
    type Selector<Count: sea_orm::FromQueryResult> =
        Selector<SelectModel<Count>>;

    type Select = Self;

    fn select_count_by_colum<C>(self, col: C) -> Self::Select
    where
        ColumnExpr: From<C>,
    {
        self.select_only()
            .column_as(ColumnExpr::from(col).count(), COUNT_NAME)
    }

    fn count_by_column<C, Count>(self, col: C) -> Self::Selector<Count>
    where
        ColumnExpr: From<C>,
        Count: FromQueryResult,
    {
        self.select_count_by_colum(col).into_model::<Count>()
    }
}
#[derive(Debug)]
#[repr(transparent)]
/// 用于 select count 中被计数的表达式
pub struct ColumnExpr(Expr);

impl ColumnExpr {
    fn count(self) -> SimpleExpr {
        self.0.count()
    }
    /// count (*)
    /// 可参考 [Expr::asterisk]
    pub fn asterisk() -> Self {
        Self(Expr::asterisk())
    }
}

impl<C: ColumnTrait> From<C> for ColumnExpr {
    fn from(col: C) -> Self {
        Self(col.into_expr())
    }
}
