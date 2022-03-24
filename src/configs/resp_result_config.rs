use std::borrow::Cow;

use resp_result::ConfigTrait;

#[derive(Debug,serde::Deserialize)]
pub struct RespResultConfig {
    // serde configs
    #[serde(alias="body",alias="b")]
    body_name: String,
    #[serde(alias="err",alias="err-msg")]
    err_msg_name: String,
    #[serde(alias="fix-field",default="Default::default")]
    full_field: bool,
    #[serde(alias="bool-status")]
    signed_status: Option<String>,
    #[serde(alias="body-extra-err")]
    body_extra_err: Option<String>,
    // resp configs
    #[serde(alias="header-extra-err")]
    header_extra_err: Option<String>,
}

impl ConfigTrait for RespResultConfig {}

impl resp_result::SerdeConfig for RespResultConfig {
    fn body_name(&self) -> Cow<'static, str> {

        self.body_name.clone().into()
    }

    fn err_msg_name(&self) -> Cow<'static, str> {
        self.err_msg_name.clone().into()
    }

    fn full_field(&self) -> bool {
        self.full_field
    }

    fn signed_base_status(&self) -> Option<Cow<'static, str>> {
        self.signed_status
            .clone()
            .map(Into::<Cow<'static, str>>::into)
    }

    fn extra_code(&self) -> Option<Cow<'static, str>> {
        self.body_extra_err.clone().map(Into::into)

    }
}

impl resp_result::RespConfig for RespResultConfig {
    fn head_extra_code(&self) -> Option<Cow<'static, str>> {
        self.header_extra_err.clone().map(Into::into)
    }
}

#[cfg(test)]
mod test{
    #[test]
    fn test_128() {
        let a =128u8;
        println!("{:b}",a);
    }
}