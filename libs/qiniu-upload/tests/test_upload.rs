use ceobe_qiniu_upload::{
    FilePayload, JsonPayload, PayloadLocal, SecretConfig, Uploader,
};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct SecretConfigure {
    access_key: String,
    secret_key: String,
}

impl SecretConfig for SecretConfigure {
    fn access_key(&self) -> &str { &self.access_key }

    fn secret_key(&self) -> &str { &self.secret_key }
}

fn read_config() -> SecretConfigure {
    let f = std::fs::read("./test_payloads/secret_config.json")
        .expect("config File not exist");

    let payload = serde_json::from_slice::<SecretConfigure>(&f)
        .expect("Bad Json format");
    payload
}

#[tokio::test]
async fn test_json_upload() {
    let cfg = read_config();
    let u = {
        let t = Uploader::builder(&cfg)
            .add_bucket("frozen-string")
            .expect("Bucket Used")
            .build();
        t
    };

    struct J;

    impl PayloadLocal for J {
        fn bucket(&self) -> &str { "frozen-string" }

        fn obj_name(&self) -> &str { "foo_json2" }
    }

    impl JsonPayload for J {
        type Payload = Value;

        fn payload(self) -> Self::Payload {
            json! {
                {
                    "foo":11u32,
                    "bar": "foo bar"
                }
            }
        }
    }

    let v = u.upload_json(J).await.expect("Upload error");
    println!("{:?}", v)
}
#[tokio::test]
async fn test_file_upload() {
    let cfg = read_config();
    let u = {
        let t = Uploader::builder(&cfg)
            .add_bucket("frozen-string")
            .expect("Bucket Used")
            .build();
        t
    };

    struct J;

    impl PayloadLocal for J {
        fn bucket(&self) -> &str { "frozen-string" }

        fn obj_name(&self) -> &str { "foo_json_file" }
    }

    impl FilePayload for J {
        type Path = str;

        fn file_path(&self) -> &Self::Path { "./test_payloads/test.json" }
    }

    let v = u.upload_file(J).await.expect("Upload error");
    println!("{:?}", v)
}
