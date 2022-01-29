
#[derive(Deserialize, Debug,Serialize)]
pub struct DataItem {
    #[serde(rename = "dataSource")]
    pub(crate) data_source: String,

    pub(crate) id: String,
    #[serde(rename = r#"timeForSort"#)]
    time_for_sort: u64,
    #[serde(rename = r#"timeForDisplay"#)]
    time_for_display: String,

    content: String,
    #[serde(rename = r#"jumpUrl"#)]
    jump_url: String,
    #[serde(rename = r#"coverImage"#)]
    cover_image: Option<String>,
    #[serde(rename = r#"imageList"#,default="Default::default")]
    image_list: Option<Vec<String>>,
    #[serde(rename = r#"imageHttpList"#,default="Default::default")]
    image_http_list: Option<Vec<String>>,
    #[serde(rename = r#"isTop"#, default = "default_top")]
    is_top: bool,

    retweeted: Option<serde_json::Value>,
    #[serde(rename = r#"componentData"#)]
    component_data: Option<serde_json::Value>,
}

fn default_top() -> bool {
    false
}


impl DataItem {
    pub fn get_id(&self)->&str{&self.id}
}