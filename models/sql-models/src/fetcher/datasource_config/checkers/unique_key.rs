use sea_orm::{TryGetable, Value};
use sea_query::ValueType;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
)]
#[repr(transparent)]
#[serde(transparent)]
pub struct DatasourceUnique {
    pub identify: String,
}

impl TryGetable for DatasourceUnique {
    fn try_get(
        res: &sea_orm::QueryResult, pre: &str, col: &str,
    ) -> Result<Self, sea_orm::TryGetError> {
        <String as TryGetable>::try_get(res, pre, col).map(Into::into)
    }
}

impl ValueType for DatasourceUnique {
    fn try_from(v: Value) -> Result<Self, sea_query::ValueTypeErr> {
        <String as ValueType>::try_from(v).map(Into::into)
    }

    fn type_name() -> String { String::type_name() }

    fn array_type() -> sea_query::ArrayType { String::array_type() }

    fn column_type() -> sea_query::ColumnType { String::column_type() }
}

impl From<String> for DatasourceUnique {
    fn from(v: String) -> Self { Self { identify: v } }
}

impl From<DatasourceUnique> for Value {
    fn from(val: DatasourceUnique) -> Self {
        Value::String(Some(Box::new(val.identify)))
    }
}
