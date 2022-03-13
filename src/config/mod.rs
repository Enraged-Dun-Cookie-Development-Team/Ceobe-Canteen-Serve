mod resp;
pub use self::resp::RespConfig;
pub use self::serde::SerdeConfig;

mod serde;


pub trait ConfigTrait: Sync + 'static
where
    Self: SerdeConfig,
    Self:RespConfig
{
}

pub struct  DefaultConfig;

impl SerdeConfig for DefaultConfig {
    
}

impl RespConfig for DefaultConfig {
    
}

impl ConfigTrait for DefaultConfig {
    
}