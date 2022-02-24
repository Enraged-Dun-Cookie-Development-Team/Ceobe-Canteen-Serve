
use crate::database::error::DatabaseError;

#[macro_export]
/// 辅助构造枚举形式的Error
/// 
/// ```rust
/// error_generate!(
/// pub GolbalError 
/// Io=std::io::Error
/// );
/// ```
/// 
macro_rules! error_generate {
    ($v:vis $err_name:ident $($v_name:ident=$inner_err:ty)+ ) => {
        #[derive(Debug)]
        $v enum $err_name{
            $(
                $v_name($inner_err)
            ),+
        }
        impl std::error::Error for $err_name{}
        impl std::fmt::Display for $err_name{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self{
                    $(
                        Self::$v_name(err)=>{write!(f, "{} Error : {}",stringify!($v_name), err)}
                    ),+
                }
            }
        }

        $(
            impl From<$inner_err> for $err_name{
                fn from(src: $inner_err) -> Self {
                    Self::$v_name(src)
                }
            }

        )+
    };
    ($v:vis $err_name:ident = $msg:literal)=>{
        #[derive(Debug)]
        $v struct $err_name;
        impl std::error::Error for $err_name{}
        impl std::fmt::Display for $err_name{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
               writeln!(f, "{} Error : {}",stringify!($err_name), $msg)
            }
        }
    };
}

error_generate!(
    pub GlobalError 
    Io=std::io::Error
    Db=DatabaseError
);
