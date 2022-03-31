/// 生成组合模式的用户权限等级
/// 
/// ## example
/// 
/// ```rust
///new_auth_level! {
///   //|-------------------可见性
///   //|   |---------------权限名称
///    pub MansionAuth=>[
///        Architect // 内部的具体权限，可以嵌套
///        Cooker
///    ]
///}
/// ```
#[macro_export]
macro_rules! new_auth_level {
    {
        $v:vis $name:ident =>
        // accept levels
        [
            $(
                $ex:ty
            )*
        ]
    } => {
        $v struct $name;

        impl $crate::utils::user_authorize::auth_level::AuthLevelVerify for $name{
            fn auth_name()->&'static str{
                stringify!($name)
            }

            fn verify(token:&$crate::utils::user_authorize::AuthLevel)->bool{
                        $(
                            <$ex as $crate::utils::user_authorize::auth_level::AuthLevelVerify>::verify(token)
                        )||*
                }
            }
    };
}

#[cfg(test)]
mod test {
    use crate::utils::user_authorize::{
        auth_level::{
            prefabs::{Architect, Chef, Cooker},
            AuthLevelVerify,
        },
        AuthLevel,
    };

    new_auth_level! {
        pub MansionAuth=>[
            Architect
            Cooker
        ]
    }

    new_auth_level! {
        pub MockAuth=>[
            MansionAuth
            Chef
        ]
    }

    #[test]
    fn test_base() {
        let l = AuthLevel::Chef;

        let resp = MansionAuth::verify(&l);

        assert_eq!(resp, false);
        let resp = MansionAuth::verify(&AuthLevel::Cooker);

        assert_eq!(resp, true);
        let resp = MansionAuth::verify(&AuthLevel::Architect);

        assert_eq!(resp, true);
    }
}
