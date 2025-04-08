
pub trait JwtTokenConv:Sized{
    fn from_jwt_token(token:&str)->jsonwebtoken::errors::Result<Self>;
    
    fn to_jwt_token(&self)->jsonwebtoken::errors::Result<String>;
}