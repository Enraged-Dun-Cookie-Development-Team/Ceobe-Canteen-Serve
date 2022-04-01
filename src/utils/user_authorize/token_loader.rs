crate::quick_trait! {
    /// 从请求信息中加载Token信息
    pub TokenLoader{
        fn form_req(req:&actix_web::HttpRequest)->Option<String>;
    }
}