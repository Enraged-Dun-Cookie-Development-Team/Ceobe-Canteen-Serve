use _private::NextIdTrait;
use mongodb::bson::oid::ObjectId;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ListWithNextId<T, N>
where
    T: Serialize,
    N: Serialize + NextIdTrait,
{
    list: Vec<T>,
    next_id: Option<N>,
}

mod _private {
    pub trait NextIdTrait {}
}

pub trait GenerateListWithNextId<N>: IntoIterator
where
    Self::Item: Serialize,
    N: Serialize + NextIdTrait,
{
    fn with_page_next_id_info(
        self, next_id: Option<N>,
    ) -> ListWithNextId<Self::Item, N>;
}

impl<N, T> GenerateListWithNextId<N> for T
where
    T: IntoIterator,
    T::Item: Serialize,
    N: Serialize + NextIdTrait,
{
    fn with_page_next_id_info(
        self, next_id: Option<N>,
    ) -> ListWithNextId<Self::Item, N> {
        ListWithNextId {
            list: self.into_iter().collect(),
            next_id,
        }
    }
}

// 支持成为NextId的类型
impl NextIdTrait for String {}
