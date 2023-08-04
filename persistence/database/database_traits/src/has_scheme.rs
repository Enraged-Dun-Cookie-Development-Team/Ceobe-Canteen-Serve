
pub trait Has<S: Field>{
    type Ty;

    fn get(&self)->Option<&Self::Ty>{None}

    fn set(&mut self,value:Self::Ty);

    fn set_optional(&mut self,value:Option<Self::Ty>){
        if let Some(value) = value{
            self.set(value)
        }
    }
}

pub trait Field {
    const NAME:&'static str;
}

#[macro_export]
macro_rules! has_field {
    ($name:ident:$field:ident) => {
        pub struct $name;

        impl $crate::has_scheme::Field for $name{
            const NAME: & 'static str = stringify!($field);
        }
    };
}

