use ceobe_qiniu_upload::{
    FilePayload, JsonPayload, PayloadLocal, SecretConfig, Uploader,
};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct SecretConfigure {
    #[serde(alias = "access")]
    access_key: String,
    #[serde(alias = "secret")]
    secret_key: String,
}

impl SecretConfig for SecretConfigure {
    fn access_key(&self) -> &str {
        &self.access_key
    }

    fn secret_key(&self) -> &str {
        &self.secret_key
    }
}

fn read_config() -> SecretConfigure {
    let f = 'file: {
        #[cfg(test)]
        break 'file include_bytes!("../../../qiniu_example.json");
        #[cfg(not(test))]
        &[0; 0]
    };

    let payload = serde_json::from_slice::<SecretConfigure>(&*f)
        .expect("Bad Json format");
    payload
}

#[tokio::test]
async fn test_json_upload() {
    let cfg = read_config();
    let u = {
        let t = Uploader::builder(&cfg, "frozen-string").build();
        t
    };

    struct J(Value, &'static str);

    impl PayloadLocal for J {


        fn obj_name(&self) -> &str {
            self.1
        }

        fn file_name(&self) -> &str {
            self.1
        }
    }

    impl JsonPayload for J {
        type Payload = Value;

        fn payload(self) -> Self::Payload {
            self.0
        }
    }

    let v = u
        .upload_json(J(
            json! {
                {
                    "name" : "value1",
                    "name" : "value2",
                    "type" : 1
                }
            },
            "data/Data A",
        ))
        .await
        .expect("Upload error");
    println!("{:?}", v);
    let v = u
        .upload_json(J(
            json! {
                {
                    "name" : "value3",
                    "name" : "value4",
                    "type" : 2
                }
            },
            "data/Data B",
        ))
        .await
        .expect("Upload error");
    println!("{:?}", v)
}
#[tokio::test]
async fn test_file_upload() {
    let cfg = read_config();
    let u = {
        let t = Uploader::builder(&cfg, "frozen-string").build();
        t
    };

    struct J;

    impl PayloadLocal for J {


        fn obj_name(&self) -> &str {
            "foo_json_file"
        }
    }

    impl FilePayload for J {
        type Path = str;

        fn file_path(&self) -> &Self::Path {
            "./test_payloads/test.json"
        }
    }

    let v = u.upload_file(J).await.expect("Upload error");
    println!("{:?}", v)
}
