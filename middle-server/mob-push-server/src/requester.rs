use general_request_client::{
    http::header::CONTENT_TYPE,
    traits::{RequestBuilder, Requester},
    HeaderValue, Method, Url, Version,
};
use md5::Digest;

pub struct MobPushRequester<'key> {
    /// 推送内容
    pub(crate) content: Vec<u8>,
    pub(crate) md5: String,
    pub(crate) key: &'key str,
}

impl<'key> Requester for MobPushRequester<'key> {
    const METHOD: Method = Method::POST;
    const VERSION: Version = Version::HTTP_11;

    fn get_url(&self) -> Url {
        "http://api.push.mob.com/v3/push/createPush"
            .parse()
            .unwrap()
    }

    fn prepare_request<B: RequestBuilder>(
        self,
        builder: B,
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

pub struct FetchDeviceInfoRequester<'key, 'mid> {
    pub(crate) key: &'key str,
    pub(crate) mob_id: &'mid str,
    pub(crate) md5: String,
}

impl<'key, 'mid> FetchDeviceInfoRequester<'key, 'mid> {
    pub(crate) fn new(
        mob_id: &'mid str,
        secret: &str,
        key: &'key str,
    ) -> Self {
        let mut digit = <md5::Md5 as Digest>::new();
        digit.update(secret);
        let md5 = digit.finalize();
        let md5 = format!("{md5:x}");

        Self { key, mob_id, md5 }
    }
}

impl<'key, 'mid> Requester for FetchDeviceInfoRequester<'key, 'mid> {
    const METHOD: Method = Method::GET;
    const VERSION: Version = Version::HTTP_11;
    fn get_url(&self) -> Url {
        Url::parse(&format!(
            "http://api.push.mob.com/device-v3/getById/{}",
            urlencoding::encode(self.mob_id)
        ))
        .unwrap()
    }

    fn prepare_request<B: RequestBuilder>(
        self,
        builder: B,
    ) -> Result<B::Request, B::Error> {
        builder
            .header(|header_map| {
                header_map.insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/json"),
                );
                header_map.insert(
                    "key",
                    HeaderValue::from_str(self.key).expect("bad app key"),
                );
                header_map.insert(
                    "sign",
                    HeaderValue::from_str(&self.md5).expect("Bad Md5 result"),
                );
            })
            .build()
    }
}
