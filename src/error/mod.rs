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

        impl rresult::ErrorCode for $err_name{

                fn code(&self) -> i16 {
                    match self{
                        $(
                            Self::$v_name(err)=>{err.code()}
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
    
    ($v:vis $wrap_name:ident($err_ty:ty))=>{
        $crate::error_generate!($v $wrap_name($err_ty)="");
    };

    ($v:vis $wrap_name:ident($err_ty:ty)=$msg:literal)=>{
        #[derive(Debug)]
        $v struct $wrap_name($err_ty);
        impl std::error::Error for $wrap_name{}
        impl std::fmt::Display for $wrap_name{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
               writeln!(f, "{} Error : {}`{}`",stringify!($err_name), $msg, self.0)
            }
        }
        impl From<$err_ty> for $wrap_name{
            fn from(src:$err_ty)->Self{
                Self(src)
            }
        }
    };

}

error_generate!(
    pub GlobalError
    Io=std::io::Error
    Db=DatabaseError
);

error_generate!(pub IoError(std::io::Error));