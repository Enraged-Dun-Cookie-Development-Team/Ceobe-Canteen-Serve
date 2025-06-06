use http::StatusCode;
use serde::{Deserialize, Deserializer};

#[derive(Debug,Clone,Deserialize)]
pub struct ErrorType{
    pub mark:char,
    pub ident:String,
    pub description:String,
    #[serde(alias="defaultStatus",deserialize_with ="deserialize_status_code")]
    pub default_status_code:StatusCode,
    pub error:Vec<Error>
}

#[derive(Debug,Clone,Deserialize)]
pub struct Error{
    pub ident:String,
    pub code:Option<u16>,
    #[serde(rename="statusCode",deserialize_with ="deserialize_option_status_code",default)]
    pub http_code:Option<StatusCode>,
    pub description:String
}
#[derive(Debug,Clone,Deserialize)]
pub struct ErrorCfg{
   pub  kind:Vec<ErrorType>
}


fn deserialize_status_code<'de,D:Deserializer<'de>>(d:D)->Result<StatusCode,D::Error>{
    let code = <u16 as Deserialize>::deserialize(d)?;
    StatusCode::from_u16(code).map_err(serde::de::Error::custom)
}

fn deserialize_option_status_code<'de,D:Deserializer<'de>>(d:D)->Result<Option<StatusCode>,D::Error>{
    let code = <Option<u16> as Deserialize>::deserialize(d)?;
    code.map(|code|StatusCode::from_u16(code).map_err(serde::de::Error::custom)).transpose()
}

#[cfg(test)]
mod test{
    use crate::payloads::ErrorCfg;

    #[test]
    fn test_load(){
        let v = include_str!("../../.././example_error_config.toml");
        let payload:ErrorCfg = toml::from_str(v).expect("Error");
        
        println!("{payload:?}")
    }
}