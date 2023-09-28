use super::Secret;


pub trait AuthorizeConfig{
    fn editor(&self,config:&mut ());
}

pub(crate) struct AuthorizeCof{
    secret:Secret,
    header:()
}

