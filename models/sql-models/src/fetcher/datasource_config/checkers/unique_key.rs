use sea_orm::{TryGetable, Value};
use sea_query::ValueType;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
)]
#[repr(transparent)]
#[serde(transparent)]
pub struct DatasourceUnique {
    pub identify: u64,
}

impl TryGetable for DatasourceUnique {
    fn try_get(
        res: &sea_orm::QueryResult, pre: &str, col: &str,
    ) -> Result<Self, sea_orm::TryGetError> {
        <u64 as TryGetable>::try_get(res, pre, col).map(Into::into)
    }
}

impl ValueType for DatasourceUnique {
    fn try_from(v: Value) -> Result<Self, sea_query::ValueTypeErr> {
        <u64 as ValueType>::try_from(v).map(Into::into)
    }

    fn type_name() -> String { u64::type_name() }

    fn array_type() -> sea_query::ArrayType { u64::array_type() }

    fn column_type() -> sea_query::ColumnType { u64::column_type() }
}

impl From<u64> for DatasourceUnique {
    fn from(v: u64) -> Self { Self { identify: v } }
}

impl From<DatasourceUnique> for Value {
    fn from(val: DatasourceUnique) -> Self {
        Value::BigUnsigned(Some(val.identify))
    }
}
