#[macro_export]
/// 辅助生成成对已检查 / 未检查 结构体对
/// ```rust, ignore
/// use checker::check_obj;
/// check_obj! {
/// {
///    #[derive(Deserialize)]        // 所有要挂载到 Unchecked 上的Attr
/// }
/// {
///    #[derive(serde::Serialize)]  // 所有要挂载到 Checked 上的Attr
/// }
/// //| --- 标识 Unchecked 和 Checked 和 Checker 的 可见模式
/// //|         | ---- Unchecked 的标识符   
/// //|         |     |---- Checker 的标识符
/// //|         |     |           | ----- Checked 的标识符
/// pub struct (Ac =(AcBChecker)> B){
///    #[serde(rename="abb")] //------- Field Attr 同时挂载到Unchecked 和 Check 的同名字段上
///    pub b:NoCheck<u32>,
/// //  |  |   |---------------当前field 使用的 DataChecker ,!! 注意，不是Unchecked 类型
/// //  |  |-------------------当前field 的字段名称
/// //  |---------------------当前Field 的可见性，Checked 和 Unchecked 相同
///    c:NoCheck<String>
///      }
/// err: Infallible
/// }  //    |----------------- 构造的DataChecker 检查时的异常类型 ，要求
///  //所有使用的 DataChecker::Err 实现转换函数Into::into()   
/// ```
macro_rules! check_obj {
    {
        {$(#[$uc_attr:meta])*}
        {$(#[$c_attr:meta])*}
        $v:vis struct $uc_name:ident = $checker:ident > $c_name:ident{
            $( $(#[$f_attr:meta])*
                $fv:vis $f_n:ident : $f_ty:ty
            ),*
        }
        err: $err:ty
    } => {
        $crate::__check_struct!{
            $(#[$c_attr])*
            $v $c_name[
                $(
                    $(#[$f_attr])*
                    $fv $f_n : $f_ty
                )*
            ]
        }

        $crate::__uncheck_struct!{
            $(#[$uc_attr])*
            $v $uc_name[
                $(
                    $(#[$f_attr])*
                    $f_n : $f_ty
                )*
            ]
        }

        $crate::__checker_generate!($uc_name => $v $checker[$($f_n:$f_ty),*]=>$c_name | $err);
    };
    {
        $(#[$uc_attr:meta])*
        $v:vis struct $uc_name:ident = $checker:ident > $c_name:ty{
            $(
            $(#[$f_attr:meta])*
            $fv:vis $f_n:ident : $f_ty:ty
        ),*
    }
    err: $err:ty
    }=>{
        $crate::__uncheck_struct!{
            $(#[$uc_attr])*
            $v $uc_name[
                $(
                    $(#[$f_attr])*
                    $f_n : $f_ty
                )*
            ]
        }

        $crate::__checker_generate!($uc_name => $v $checker[$($f_n:$f_ty),*] => $c_name | $err);
    }
}

#[macro_export]
macro_rules! __uncheck_struct {
    {
        $(#[$m:meta])*
        $v:vis $name:ident
        [
            $(
                $(#[$f_m:meta])*
                $f_n:ident:$f_ty:ty
            )*
        ]
    } => {
        /// 这是未检查的struct
        $(#[$m])*
        $v struct $name
        where
        $(
            $f_ty : $crate::Checker
        ),*
        {
            $(
                $(#[$f_m])*
                $f_n : $crate::CheckRequire<$f_ty>
            ),*
        }
    };
}

#[macro_export]
macro_rules! __check_struct {
    {
        $(#[$m:meta])*
        $v:vis $name:ident[
            $(
                $(#[$fm:meta])*
                $fv:vis $f_n:ident:$f_ty:ty
            )*
        ]
    } => {
        /// 这是通过检查的struct
        $(#[$m])*
        #[derive(typed_builder::TypedBuilder)]
        $v struct $name
        where
        $(
            $f_ty : $crate::AsyncChecker
        ),*
        {
            $(
                $(#[$fm])*
                $fv $f_n : <$f_ty as $crate::AsyncChecker>::Checked
            ),*
        }
    };
}

#[macro_export]
macro_rules! __checker_generate {
    ( $uc:ty => $v:vis $name:ident[$($f_n:ident:$f_ty:ty),*] => $cd:ty | $err:ty ) => {

        $v struct $name;

        impl $crate::Checker for $name
        where
        $(
            $f_ty : $crate::Checker,
            <$f_ty as $crate::Checker>::Err: Into<$err>,
        )*
        {
            type Unchecked = $uc;
            type Args = (
                $(<$f_ty as $crate::Checker>::Args),*,
            );
            type Checked = $cd ;
            type Err = $err;
            type Fut = std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = Result<Self::Checked,Self::Err>> + std::marker::Send>>;
            fn check(($($f_n),*,): Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
                std::boxed::Box::pin(
                    async move{
                        $( let $f_n = uncheck.$f_n.checking($f_n); )*
                        let __resp = <$cd>::builder();
                        $(
                            let __resp = __resp.$f_n($f_n.await.map_err(Into::<$err>::into)?);
                        )*
                        let __resp =__resp.build();

                        Ok(
                            __resp
                        )
                    }
                )
            }
        }
    };
}

#[cfg(test)]
mod test {
    use std::convert::Infallible;

    use typed_builder::TypedBuilder;

    use crate::{check_obj, prefabs::no_check::NoCheck, CheckRequire};

    #[derive(Debug, TypedBuilder, PartialEq, Eq)]
    pub struct TestChecked {
        a: i32,
        b: String,
    }

    check_obj! {
        pub struct TestUncheck = TestChecker > TestChecked{
            a: NoCheck<i32>,
            b: NoCheck<String>
        }
        err:Infallible
    }

    #[tokio::test]
    async fn test_pre_lite_check() {
        let uncheck = TestUncheck {
            a: CheckRequire::new(NoCheck::new(), 112),
            b: CheckRequire::new(NoCheck::new(), "121212".into()),
        };

        let init = CheckRequire::new(TestChecker, uncheck);

        let resp = init.lite_checking().await.unwrap();

        assert_eq!(
            resp,
            TestChecked {
                a: 112,
                b: "121212".into()
            }
        )
    }
}
