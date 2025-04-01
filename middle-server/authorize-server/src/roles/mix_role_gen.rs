#[macro_export]
macro_rules! mix_role_gen {
    {
        $vis:vis $name:ident
        =>
        [
            $(
                $ex:ty
            ),*$(,)?
        ]
    } => {
        #[derive(Debug,Clone)]
        $v struct $name;
        
        impl $crate::roles::UserRoleVerify for $name{
            const ROLE_NAME: &'static str = stringify!($name);
            
            fn access_verify(level: &$crate::AuthLevel) -> bool {
                $(
                    <$ex as $crate::roles::UserRoleVerify>::access_verify(level)
                )||*
            }
        }
    };
}