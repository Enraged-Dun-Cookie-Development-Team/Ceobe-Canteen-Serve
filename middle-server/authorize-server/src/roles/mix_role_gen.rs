/// 生成组合模式的用户权限等级
///
/// ## example
///
/// ```rust
/// use authorize_server::mix_role_gen;
/// use authorize_server::roles::{Architect,Cooker};
/// mix_role_gen! {
///   //|-------------------可见性
///   //|   |---------------权限名称
///    pub MansionAuth=>[
///        Architect // 内部的具体权限，可以嵌套
///        Cooker
///    ]
/// }
/// ```
#[macro_export]
macro_rules! mix_role_gen {
    {
        $v:vis $name:ident
        =>
        [
            $(
                $ex:ty
            )*
        ]
    } => {
        #[derive(Debug, Clone)]
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
