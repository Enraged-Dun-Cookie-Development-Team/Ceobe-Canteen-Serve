#[macro_export]
/// 快速构建简单结构体
macro_rules! quick_struct {
    {
        $(#[$sm:meta])*
        $v:vis $name:ident {
        $(
            $(#[$fm:meta])*
            $f:ident:$t:ty
        )*
    }
    }=>{
        #[derive(Debug,Clone,serde::Serialize,serde::Deserialize,typed_builder::TypedBuilder)]
        /// 该结构体通过宏快速构造
        ///
        /// ---
        ///
        $(#[$sm])*
        $v struct $name{
            $(
                $(#[$fm])*
                pub $f:$t,
            )*
        }
    };
    {
        $(
            $(#[$sm:meta])*
        $v:vis $name:ident {
            $(
                $(#[$fm:meta])*
                $f:ident:$t:ty
            )*
        }
    )*

    } => {
        $(
            $crate::quick_struct!{
                $(#[$sm])*
                $v $name {
                $(
                    $(#[$fm])*
                    $f:$t
                )*
            }
            }
        )*
    };
}
