use general_request_client::{
    traits::{RequestBuilder, Requester},
    HeaderValue, Method, Url,
};

pub struct MobPushRequester<'key> {
    /// 推送内容
    pub(crate) content: Vec<u8>,
    pub(crate) md5: String,
    pub(crate) key: &'key str,
}

impl<'key> Requester for MobPushRequester<'key> {
    const METHOD: Method = Method::POST;
    const VERSION: general_request_client::Version = general_request_client::Version::HTTP_11;

    fn get_url(&self) -> Url {
        "http://api.push.mob.com/v3/push/createPush"
            .parse()
            .unwrap()
    }

    fn prepare_request<B: RequestBuilder>(
        self, builder: B,
    ) -> Result<B::Request, B::Error> {
        builder
            .header(|map| {
                map.append("sign", HeaderValue::from_str(&self.md5).unwrap());
                map.append(
                    "content-type",
                    HeaderValue::from_static("application/json"),
                );
                map.append("key", HeaderValue::from_str(self.key).unwrap());
            })
            .body(self.content)
            .build()
    }
}
