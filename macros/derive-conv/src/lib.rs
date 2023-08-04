//! help drive macro for type conv
//! see issue #155
//!
//! ```rust norun
//! use model::Model;
//!
//! #[derive(ConvHelper)]
//! #[conv(mode = "IntoActiveModel",mode = "UpdateActiveModel",mode = "FromModel")]
//! #[conv(target="Model")]
//! #[conv(
//!     preprocess(
//!         var="(temp_A,foo_auto)",
//!         process="|foo1,foo2,bar_mut,bar_owned,bar_copy| foo1 + foo2",
//!         requires(
//!             foo_default_ref,
//!             foo = "ref",
//!             bar_mut = "mut",
//!             bar_owned = "owned",
//!             bar_copy = "copy"
//!         ),
//!         skip_mode = "update|into|from"
//!     )
//! )]
//! #[conv(non_exist_use_default)]
//! #[conv(generate_fields(
//!     foo_auto,
//!     super_field = "temp_A"
//! ))]
//! struct Foo{
//!     #[conv(condition_set(
//!     condition = "|this,foo| this > foo",
//!     requires("foo_auto")
//!     ))]
//!     id:i32,
//!     #[conv(ignore)]
//!     foo:i32,
//!     #[conv(project = "Version::from")]
//!     bar:String,
//!     #[conv(rename = "modify_at")]
//!     time:String,
//! }
//! ```
//! - 请参考 [darling](https://github.com/TedDriggs/darling) 文档
//! - 请参考 [syn](https://docs.rs/syn/latest/syn/) 文档
//! - 请牢记：宏为替换抽象语法树节点，宏的参数为抽象语法树，宏的结果也为抽象语法树
mod derive_input_conv;

#[proc_macro_derive(ConvHelper, attributes(conv))]
pub fn derive_conv_helper(
    _input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    todo!()
}
