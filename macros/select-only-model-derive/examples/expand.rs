use select_only_model_derive::SelectOnlyModel;
use sql_connection::{
    ext_traits::select_only_model::SelectPartial,
    sea_orm,
    sea_orm::{sea_query::MySqlQueryBuilder, FromQueryResult, QueryTrait},
    EntityTrait,
};
use sql_models::ceobe_operation::announcement::Entity;
fn main() {
    let sql = Entity::find()
        .select_for_cols::<AnnouncementContentImageOnly>()
        .into_query();

    println!("{}", sql.to_string(MySqlQueryBuilder::default()))
}

#[allow(dead_code)]
#[derive(Debug, SelectOnlyModel, FromQueryResult)]
#[select_only(origin = "Entity")]
struct AnnouncementContentImageOnly {
    content: String,
    #[select_only(from_col = "img_url")]
    image: String,
}
