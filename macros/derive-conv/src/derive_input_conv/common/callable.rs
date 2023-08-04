
/// 需要实现 [FromMeta](darling::FromMeta)
/// 
/// 表示一个可以作为函数调用的表达式，有以下的情况
/// 
/// 1. 用户提供一个 [`Path`](syn::Path), 这个路径是一个函数的路径
/// 2. 用户提供一个 [`Closure`](syn::ExprClosure), 这是一个闭包，在原地声明函数的操作
pub enum Callable{
    // TODO： 完成可调用对象解析
}