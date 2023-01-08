use sea_orm::{
    ColumnTrait, EntityTrait, FromQueryResult, QuerySelect, Select,
    SelectModel, Selector,
};

use super::{Count, CountNonZero, CountZero, COUNT_NAME};

/// 计数指定列的
pub trait QueryCountByColumn: Sized {
    type Selector<Count: FromQueryResult>;
    type Select;

    /// 只修改select 但是不映射模型 （可以用于测试生成的SQL）
    fn select_count_by_colum<C:ColumnTrait>(self, col: C) -> Self::Select;

    /// 修改select 为对指定col count 并映射为指定模型
    fn count_by_column<C, Count>(self, col: C) -> Self::Selector<Count>
    where
        C: ColumnTrait,
        Count: FromQueryResult;

    /// 对指定的col 进行计数，并与0比较
    /// count == 0 ? 即指定列的记录为 0
    fn count_zero_by_column<C>(self, col: C) -> Self::Selector<CountZero>
    where
        C: ColumnTrait,
    {
        self.count_by_column(col)
    }
    /// 对指定的col 进行计数，并与0比较
    /// count != 0 ? 即指定的列计数为 非 0
    fn count_non_zero_by_column<C>(
        self, col: C,
    ) -> Self::Selector<CountNonZero>
    where
        C: ColumnTrait,
    {
        self.count_by_column(col)
    }

    fn count_only_by_column<C: ColumnTrait>(
        self, col: C,
    ) -> Self::Selector<Count> {
        self.count_by_column(col)
    }
}

impl<E: EntityTrait> QueryCountByColumn for Select<E> {
    type Selector<Count: sea_orm::FromQueryResult> =
        Selector<SelectModel<Count>>;

    type Select = Self;

    fn select_count_by_colum<C:ColumnTrait>(self, col: C) -> Self::Select {
        self.select_only().column_as(col.count(), COUNT_NAME)
    }

    fn count_by_column<C, Count>(self, col: C) -> Self::Selector<Count>
    where
        C: ColumnTrait,
        Count: FromQueryResult,
    {
        self.select_count_by_colum(col).into_model::<Count>()
    }
}
