/// 构造一个可以产生特定域的结构体特征
///
/// ##example
/// ```rust
/// quick_trait! {
/// pub Mock{
///     // 在字段前使用 `*` 将会标记函数返回所有权而不是引用
///     // 以下将生成 `fn max_size(&self)->u32;`
///     trait_field!{*max_size:u32}
///     // 在类型后面 使用 `=` 提供默认实现
///     trait_field!{*port: u16 = 3360}
///     // 字段前不前缀`*` 接口将返回引用类型
///     // 以下将生成 `fn host(&self)->&& 'static str;`
///     trait_field!{host: & 'static str}
///     // 同样可以指定默认值
///     trait_field!{host2: u32 = &11}
///     }
/// }
/// ```

#[macro_export]
macro_rules! quick_trait {
    {

        $(#[$m:meta])*
        $v:vis $name:ident{
            $(
                $t:tt
            )*
        }
    } => {
        $(#[$m])*
        $v trait $name{
            $(
                $t
            )*
        }
    };

    {$(
        $(#[$m:meta])*
        $v:vis $name:ident{
        $(
            $t:tt
        )*
    })*}=>{
        $(
            $crate::quick_trait!{
                $(#[$m])*
                $v $name {
                    $(
                        $t
                    )*
                }
            }
        )*
    }
}
/// 为 [quick_trait] 提供trait 接口构造信息
#[macro_export]
macro_rules! trait_field {
    {$f:ident : $t:ty} => {
        fn $f(&self) -> &$t;
    };
    {$f:ident : $t:ty = $e:expr} => {
        #[inline]
        fn $f(&self) -> &$t { $e }
    };
    {*$f:ident : $t:ty} => {
        fn $f(&self) -> $t;
    };
    {*$f:ident : $t:ty = $e:expr} => {
        #[inline]
        fn $f(&self) -> $t { $e }
    };
}

quick_trait! {
    #[allow(dead_code)]
    pub Mock{
        trait_field!{*max_size:u32}
        trait_field!{*port: u16 = 3360}
        trait_field!{host: & 'static str}
        trait_field!{host2: u32 = &11}
    }
    #[allow(dead_code)]
    pub Next{

    }
}
