use select_only_model_derive::SelectOnlyModel;
use sql_models::ceobe_operation::announcement::Entity;
fn main() {}

#[allow(dead_code)]
#[derive(Debug, SelectOnlyModel)]
#[select_only(origin = "Entity")]
struct AnnouncementContentImageOnly {
    content: String,
    #[select_only(from_col = "img_url")]
    image: String,
}
