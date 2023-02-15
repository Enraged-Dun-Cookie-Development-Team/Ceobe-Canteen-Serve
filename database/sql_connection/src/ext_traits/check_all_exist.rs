use sea_orm::{
    sea_query::{self, Alias, Expr, Query, SelectStatement, UnionType, Func, SimpleExpr},
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

pub trait QueryAllExist<E: EntityTrait> {
    fn gen_statement<C, V, I>(
        entity: E, primary: C, first: V, values: I, db: &DbBackend,
    ) -> Statement
    where
        C: ColumnTrait,
        V: Into<SimpleExpr>,
        I: IntoIterator<Item = V>;

    type RetSelector;
    fn all_exist<C, V, I>(
        self, entity: E, primary: C, first: V, residual: I, db: &DbBackend,
    ) -> Self::RetSelector
    where
        C: ColumnTrait + Send,
        V: Into<SimpleExpr> + Send,
        I: IntoIterator<Item = V> + Send;
}

impl<E: EntityTrait> QueryAllExist<E> for Select<E> {
    type RetSelector = SelectorRaw<SelectModel<CountZero>>;

    fn gen_statement<C, V, I>(
        entity: E, primary: C, first: V, values: I, db: &DbBackend,
    ) -> Statement
    where
        C: ColumnTrait,
        V: Into<SimpleExpr>,
        I: IntoIterator<Item = V>,
    {
        let state = gen_statement(entity, primary, first, values);
        StatementBuilder::build(&state, db)
    }

    fn all_exist<C, V, I>(
        self, entity: E, primary: C, first: V, residual: I, db: &DbBackend,
    ) -> Self::RetSelector
    where
        C: ColumnTrait + Send,
        V: Into<SimpleExpr> + Send,
        I: IntoIterator<Item = V> + Send,
    {
        let a = Self::gen_statement(
            entity, primary, first, residual, db,
        );
        println!("{:#?}",a.to_string());
        self.from_raw_sql(a)
        .into_model::<CountZero>()
    }
}

fn gen_statement(
    entity: impl EntityTrait, pk: impl ColumnTrait, first: impl Into<SimpleExpr>,
    residual: impl IntoIterator<Item = impl Into<SimpleExpr>>,
) -> SelectStatement {
    let mut query = Query::select();

    // select count (B.id)
    // 对所有希望存在但是不存在的行进行计数，
    // 如果为 0 那就 全部都存在
    query.expr_as(Expr::asterisk().count(), Alias::new(COUNT_NAME));

    // from ()
    query.from_subquery(
        {
            // 取出第一个做base
            let mut query = Query::select();
            query.expr_as(first, TempTable::Id);
            // 剩下的union上, 使用去重union, select from DUAL
            query.unions(residual.into_iter().map(|idx| {
                (UnionType::Distinct, {
                    let mut union_query = Query::select();
                    union_query.expr_as(idx, TempTable::Id);
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
