use std::collections::HashMap;

use darling::ToTokens;
use proc_macro2::Ident;
use syn::Type;

use super::field_filter::FieldMapper;

/// ## sub model type
/// Default Want All Field
/// with A list To Ignore Fields
pub struct WantAll {
    pub name: syn::Ident,
    pub ignores: Vec<FieldMapper>,
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
        self, parent: Ident, field: Vec<(Ident, Type)>,
    ) -> SubModelDef {
        SubModelDef {
            inner: self,
            parent,
            field,
        }
    }
}

pub struct SubModelDef {
    inner: SubModel,
    parent: Ident,
    field: Vec<(Ident, Type)>,
}

impl ToTokens for SubModelDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let token = match &self.inner {
            SubModel::DefaultAll(WantAll { name, ignores }) => {
                let ignore = ignores
                    .iter()
                    .map(|f| f.get_parent_ident())
                    .cloned()
                    .collect::<Vec<_>>();
                let (f, ty): (Vec<_>, Vec<_>) = self
                    .field
                    .iter()
                    .filter(|(f, _)| !ignore.contains(f))
                    .cloned()
                    .unzip();

                let from_f = f.clone().into_iter();
                let f = f.into_iter();
                let ty = ty.into_iter();

                let parent_name = self.parent.clone();

                quote::quote! {
                    #[derive(Debug,Clone,serde::Serialize,serde::Deserialize,typed_builder::TypedBuilder)]
                    #[doc = "SubModel 生成结构"]
                    pub struct #name {
                        #(pub #f:#ty),*
                    }

                    impl From<#parent_name> for #name {
                        fn from(__parent:#parent_name )->Self{
                            Self{
                                #(
                                    #from_f : __parent.#from_f
                                ),*
                            }
                        }
                    }
                }
            }
            SubModel::DefaultEmpty(DefaultEmpty { name, wants }) => {
                let want_src = wants
                    .iter()
                    .map(|f| {
                        (
                            f.get_parent_ident().clone(),
                            f.get_sub_ident().clone(),
                        )
                    })
                    .collect::<HashMap<_, _>>();
                let (fs, tys): (Vec<_>, Vec<_>) = self
                    .field
                    .iter()
                    .filter_map(|(f, ty)| {
                        want_src
                            .get(f)
                            .map(|i| ((f.clone(), i.clone()), ty.clone()))
                    })
                    .unzip();

                let (fs_parent, fs_sub): (Vec<_>, Vec<_>) =
                    fs.into_iter().unzip();

                let fs_parent = fs_parent.into_iter();
                let fs_from = fs_sub.clone().into_iter();
                let fs_sub = fs_sub.into_iter();
                let tys = tys.into_iter();

                let parent_name = self.parent.clone();
                quote::quote! {
                    #[derive(Debug,Clone,serde::Serialize,serde::Deserialize,typed_builder::TypedBuilder)]
                    #[doc = "SubModel 生成结构"]
                    pub struct #name{
                        #(
                            #fs_sub : #tys
                        ),*
                    }

                    impl From<#parent_name> for #name{
                        fn from(__parent:#parent_name)->Self{
                            Self{
                                #(
                                    #fs_from:__parent.#fs_parent
                                ),*
                            }
                        }
                    }
                }
            }
        };

        tokens.extend(token);
    }
}
