#[macro_export]
/// 辅助生成成对已检查 / 未检查 结构体对
/// ```rust
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
        /// 这是未检查的struct
        $(#[$uc_attr])*
        $v struct $uc_name
        where
        $(
            $f_ty : $crate::utils::data_checker::DataChecker
        ),*
        {
            $(
                $(#[$f_attr])*
                $f_n : $crate::utils::data_checker::CheckRequire<$f_ty>
            ),*
        }

        /// 这是通过检查的struct
        $(#[$c_attr])*
        $v struct $c_name
        where
        $(
            $f_ty : $crate::utils::data_checker::DataChecker
        ),*
        {
            $(
                $(#[$f_attr])*
                $fv $f_n : <$f_ty as $crate::utils::data_checker::DataChecker>::Checked
            ),*
        }

        $v struct $checker;
        
        #[allow(unused_parents)]
        impl $crate::utils::data_checker::DataChecker for $checker
        where
        $(
            $f_ty : $crate::utils::data_checker::DataChecker,
            <$f_ty as $crate::utils::data_checker::DataChecker>::Err: Into<$err>
        ),*
        {
            type Unchecked =$uc_name;
            type Args=(
                $(<$f_ty as $crate::utils::data_checker::DataChecker>::Args),*
            );
            type Checked=$c_name ;
            type Err=$err;
            type Fut=impl futures::Future<Output = Result<Self::Checked,Self::Err>>;
            fn checker(($($f_n),*): Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
               $( let $f_n = uncheck.$f_n.checking($f_n); )*

               async move{
                   Ok(
                       $c_name{
                           $(
                            $f_n:$f_n.await.map_err(Into::<$err>::into)?
                           ),*
                       }
                   )
               }
            }
        }
    };
}
