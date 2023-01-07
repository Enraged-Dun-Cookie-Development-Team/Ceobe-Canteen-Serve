use sea_orm::{
    sea_query::{self, Alias, Expr, Query, SelectStatement, UnionType},
    ColumnTrait, DbBackend, EntityTrait, Select, SelectModel, SelectorRaw,
    Statement, StatementBuilder, Value,
};

use super::{CountZero, COUNT_NAME};

// 临时表信息
#[derive(sea_query::Iden)]
enum TempTable {
    Table,
    Id,
}

pub trait AllExist<E: EntityTrait> {
    fn gen_statement<C, V>(
        entity: E, primary: C, first: V, values: impl IntoIterator<Item = V>,
        db: &DbBackend,
    ) -> Statement
    where
        C: ColumnTrait,
        V: Into<Value>;

    type RetSelector;
    fn all_exist<C, V>(
        self, entity: E, primary: C, first: V,
        values: impl IntoIterator<Item = V>, db: &DbBackend,
    ) -> Self::RetSelector
    where
        C: ColumnTrait,
        V: Into<Value>;
}

impl<E: EntityTrait> AllExist<E> for Select<E> {
    type RetSelector = SelectorRaw<SelectModel<CountZero>>;

    fn gen_statement<C, V>(
        entity: E, primary: C, first: V, values: impl IntoIterator<Item = V>,
        db: &DbBackend,
    ) -> Statement
    where
        C: ColumnTrait,
        V: Into<Value>,
    {
        let state = gen_statement(entity, primary, first, values);
        StatementBuilder::build(&state, db)
    }

    fn all_exist<C, V>(
        self, entity: E, primary: C, first: V,
        values: impl IntoIterator<Item = V>, db: &DbBackend,
    ) -> Self::RetSelector
    where
        C: ColumnTrait,
        V: Into<Value>,
    {
        self.from_raw_sql(Self::gen_statement(
            entity, primary, first, values, db,
        ))
        .into_model::<CountZero>()
    }
}

fn gen_statement(
    entity: impl EntityTrait, pk: impl ColumnTrait, first: impl Into<Value>,
    residual: impl IntoIterator<Item = impl Into<Value>>,
) -> SelectStatement {
    let mut query = Query::select();

    // select count (B.id)
    // 对所有希望存在但是不存在的行进行计数，
    // 如果为 0 那就 全部都存在
    query.expr_as(
        Expr::tbl(TempTable::Table, TempTable::Id).count(),
        Alias::new(COUNT_NAME),
    );

    // from ()
    query.from_subquery(
        {
            // 取出第一个做base
            let mut query = Query::select();
            query.expr_as(Expr::val(first), TempTable::Id);
            // 剩下的union上, 使用去重union, select from DUAL
            query.unions(residual.into_iter().map(|idx| {
                (UnionType::Distinct, {
                    let mut union_query = Query::select();
                    union_query.expr_as(Expr::val(idx), TempTable::Id);
                    union_query
                })
            }));

            query
        },
        TempTable::Table,
    );

    // join
    // lefe join  与目标表left join
    // 根据left join 的特性，如果 给的序列的ID 在数据库中不存在，那么 就会
    // 填充Null
    query.left_join(
        entity,
        Expr::tbl(entity, pk).equals(TempTable::Table, TempTable::Id),
    );
    // where
    // 根据上面left join 的结果， 通过where 只保留 填充为null 的值
    // 也就是 序列中希望存在但是不存在的 行
    query.and_where(Expr::tbl(entity, pk).is_null());

    query
}
