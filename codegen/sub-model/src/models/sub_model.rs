use std::collections::{HashMap, HashSet};

use darling::ToTokens;
use proc_macro2::Ident;
use syn::{NestedMeta, Type};

use super::field_filter::FieldMapper;

/// ## sub model type
/// Default Want All Field
/// with A list To Ignore Fields
pub struct WantAll {
    pub name: syn::Ident,
    pub ignores: Vec<FieldMapper>,
    pub having_change: Vec<FieldMapper>,
}

/// ## Sub Model Type
/// Default Is Empty
/// whit a List to declare Want Fields
pub struct DefaultEmpty {
    pub name: syn::Ident,
    pub wants: Vec<FieldMapper>,
}

pub enum SubModel {
    DefaultAll(WantAll),
    DefaultEmpty(DefaultEmpty),
}

impl SubModel {
    pub fn into_def(
        self, parent: Ident, field: impl Iterator<Item = (Ident, Type)>,
    ) -> SubModelDef {
        SubModelDef {
            inner: self,
            parent,
            field: field.collect(),
        }
    }
}

pub struct SubModelDef {
    inner: SubModel,
    parent: Ident,
    field: HashMap<Ident, Type>,
}

impl SubModelDef {
    fn get_extra_map(&self) -> HashMap<Ident, Vec<NestedMeta>> {
        let attrs = {
            match &self.inner {
                SubModel::DefaultAll(WantAll { having_change, .. }) => {
                    having_change
                }
                SubModel::DefaultEmpty(DefaultEmpty { wants, .. }) => wants,
            }
        };

        attrs
            .clone()
            .into_iter()
            .map(|fm| (fm.get_parent_ident().clone(), fm.get_extra()))
            .collect()
    }

    fn get_field_mapping(&self) -> HashMap<Ident, Ident> {
        let attrs = match &self.inner {
            SubModel::DefaultAll(WantAll { having_change, .. }) => {
                having_change
            }
            SubModel::DefaultEmpty(DefaultEmpty { wants, .. }) => wants,
        }
        .to_owned();

        attrs
            .into_iter()
            .map(|fm| {
                (fm.get_parent_ident().clone(), fm.get_sub_ident().clone())
            })
            .collect()
    }

    /// only Parent Field
    fn get_need_fields(&self) -> HashSet<Ident> {
        match &self.inner {
            SubModel::DefaultAll(WantAll { ignores, .. }) => {
                self.field
                    .iter()
                    .filter(|(f, _ty)| {
                        !ignores
                            .into_iter()
                            .map(|f| f.get_parent_ident())
                            .collect::<HashSet<_>>()
                            .contains(f)
                    })
                    .map(|(f, _)| f.to_owned())
                    .collect()
            }
            SubModel::DefaultEmpty(DefaultEmpty { wants, .. }) => {
                self.field
                    .iter()
                    .filter(|(f, _ty)| {
                        wants
                            .into_iter()
                            .map(|f| f.get_parent_ident())
                            .collect::<HashSet<_>>()
                            .contains(f)
                    })
                    .map(|(f, _ty)| f.to_owned())
                    .collect()
            }
        }
    }

    fn get_name(&self) -> Ident {
        match &self.inner {
            SubModel::DefaultAll(WantAll { name, .. }) => name.to_owned(),
            SubModel::DefaultEmpty(DefaultEmpty { name, .. }) => {
                name.to_owned()
            }
        }
    }
}

impl ToTokens for SubModelDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let super_name = self.parent.clone();
        let self_name = self.get_name();

        let need_field = self.get_need_fields();

        let field_map = self.get_field_mapping();

        let mut field_extra = self.get_extra_map();

        let create_fields = need_field.iter().map(|field| {
            // 原来的域
            let src = field;
            // 映射的域
            let tgt = field_map.get(&src).cloned().unwrap_or(src.clone());
            // 类型

            let ty = self
                .field
                .get(src)
                .cloned()
                // 这里永远不会到达
                .expect("Src Type Not Found");

            // 附加挂载载这里被消耗
            let field_extra_iter =
                field_extra.remove(src).unwrap_or_default().into_iter();

            // 构造一个域

            quote::quote! {
                #(
                    #[#field_extra_iter]
                )*
                pub #tgt : #ty
            }
        });

        let struct_body = quote::quote! {
            #[derive(Debug,Clone,serde::Serialize,serde::Deserialize,typed_builder::TypedBuilder)]
            #[doc = "通过`SubModel` 构造"]
            #[doc = "---"]
            pub struct #self_name{
                #(
                    #create_fields
                ),*
            }
        };

        tokens.extend(struct_body);

        let impl_from_mapping = need_field.iter().map(|src| {
            let src = src.clone();
            let dst = field_map.get(&src).cloned().unwrap_or(src.clone());

            quote::quote! {
                #dst : __parent.#src
            }
        });

        let self_name = self.get_name();

        let from_impl = quote::quote! {
            impl From<#super_name> for #self_name{
                fn from(__parent: #super_name) -> Self{
                    Self{
                        #(
                            #impl_from_mapping
                        ),*
                    }
                }
            }

        };

        tokens.extend(from_impl);

        // let token = match &self.inner {
        //     SubModel::DefaultAll(WantAll {
        //         name,
        //         ignores,
        //         having_change,
        //     }) => {

        //         let change_map = having_change.iter().map(|f| {
        //             (f.get_parent_ident().clone(),
        // f.get_sub_ident().clone())         });

        //         let ignore = ignores
        //             .iter()
        //             .map(|f| f.get_parent_ident())
        //             .cloned()
        //             .collect::<Vec<_>>();
        //         let (f, ty): (Vec<_>, Vec<_>) = self
        //             .field
        //             .iter()
        //             .filter(|(f, _)| !ignore.contains(f))
        //             .cloned()
        //             .unzip();

        //         let from_f = f.clone().into_iter();
        //         let f = f.into_iter();
        //         let ty = ty.into_iter();

        //         let parent_name = self.parent.clone();

        //         quote::quote! {
        //
        // #[derive(Debug,Clone,serde::Serialize,serde::Deserialize,
        // typed_builder::TypedBuilder)]             #[doc = "SubModel
        // 生成结构"]             pub struct #name {
        //                 #(pub #f:#ty),*
        //             }

        //             impl From<#parent_name> for #name {
        //                 fn from(__parent:#parent_name )->Self{
        //                     Self{
        //                         #(
        //                             #from_f : __parent.#from_f
        //                         ),*
        //                     }
        //                 }
        //             }
        //         }
        //     }
        //     SubModel::DefaultEmpty(DefaultEmpty { name, wants }) => {
        //         let want_src = wants
        //             .iter()
        //             .map(|f| {
        //                 (
        //                     f.get_parent_ident().clone(),
        //                     f.get_sub_ident().clone(),
        //                 )
        //             })
        //             .collect::<HashMap<_, _>>();
        //         let (fs, tys): (Vec<_>, Vec<_>) = self
        //             .field
        //             .iter()
        //             .filter_map(|(f, ty)| {
        //                 want_src
        //                     .get(f)
        //                     .map(|i| ((f.clone(), i.clone()), ty.clone()))
        //             })
        //             .unzip();

        //         let (fs_parent, fs_sub): (Vec<_>, Vec<_>) =
        //             fs.into_iter().unzip();

        //         let fs_parent = fs_parent.into_iter();
        //         let fs_from = fs_sub.clone().into_iter();
        //         let fs_sub = fs_sub.into_iter();
        //         let tys = tys.into_iter();

        //         let parent_name = self.parent.clone();
        //         quote::quote! {
        //
        // #[derive(Debug,Clone,serde::Serialize,serde::Deserialize,
        // typed_builder::TypedBuilder)]             #[doc = "SubModel
        // 生成结构"]             pub struct #name{
        //                 #(
        //                     #fs_sub : #tys
        //                 ),*
        //             }

        //             impl From<#parent_name> for #name{
        //                 fn from(__parent:#parent_name)->Self{
        //                     Self{
        //                         #(
        //                             #fs_from:__parent.#fs_parent
        //                         ),*
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // };
    }
}
