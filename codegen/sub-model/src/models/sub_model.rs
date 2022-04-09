use std::collections::{HashMap, HashSet};

use darling::ToTokens;
use proc_macro2::Ident;
use syn::{NestedMeta, Type, Visibility};

use super::FieldMeta;
use crate::darling_models::pub_vis;
#[derive(Debug)]
/// ## sub model type
/// Default Want All Field
/// with A list To Ignore Fields
pub struct WantAll {
    pub name: syn::Ident,
    pub ignores: Vec<FieldMeta>,
    pub having_change: Vec<FieldMeta>,
    pub extra: Vec<NestedMeta>,
    pub vis: Visibility,
}
#[derive(Debug)]
/// ## Sub Model Type
/// Default Is Empty
/// whit a List to declare Want Fields
pub struct DefaultEmpty {
    pub name: syn::Ident,
    pub wants: Vec<FieldMeta>,
    pub extra: Vec<NestedMeta>,
    pub vis: Visibility,
}

#[derive(Debug)]
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
    fn get_self_extra(&self) -> Vec<NestedMeta> {
        match &self.inner {
            SubModel::DefaultAll(WantAll { extra, .. }) => extra,
            SubModel::DefaultEmpty(DefaultEmpty { extra, .. }) => extra,
        }
        .to_owned()
    }

    fn get_self_vis(&self) -> Visibility {
        match &self.inner {
            SubModel::DefaultAll(w) => &w.vis,
            SubModel::DefaultEmpty(e) => &e.vis,
        }
        .to_owned()
    }

    fn get_map_vis(&self) -> HashMap<Ident, Visibility> {
        match &self.inner {
            SubModel::DefaultAll(w) => &w.having_change,
            SubModel::DefaultEmpty(e) => &e.wants,
        }
        .into_iter()
        .map(|v| (v.src_ident(), v.vis.clone().unwrap_or(pub_vis())))
        .collect()
    }

    fn get_extra_map(&self) -> HashMap<Ident, Vec<NestedMeta>> {
        (match &self.inner {
            SubModel::DefaultAll(WantAll { having_change, .. }) => {
                having_change
            }
            SubModel::DefaultEmpty(DefaultEmpty { wants, .. }) => wants,
        })
        .into_iter()
        .map(|fm| (fm.src_ident().clone(), fm.extra.clone()))
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
            .map(|fm| (fm.src_ident(), fm.dst_ident()))
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
                            .map(|f| f.src_ident())
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
                            .map(|f| f.src_ident())
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

        let self_vis = self.get_self_vis();
        let self_extra = self.get_self_extra().into_iter();

        let need_field = self.get_need_fields();

        let field_map = self.get_field_mapping();
        let mut vis_map = self.get_map_vis();

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
            let vis = vis_map.remove(src).unwrap_or(pub_vis());
            // 构造一个域

            quote::quote! {
                #(
                    #[#field_extra_iter]
                )*
                #vis #tgt : #ty
            }
        });

        let struct_body = quote::quote! {
            #[derive(Debug,Clone,serde::Serialize,serde::Deserialize,typed_builder::TypedBuilder)]
            #[doc = "> 通过`SubModel` 构造"]
            #[doc = "---"]
            #(
                #[#self_extra]
            )*
            #self_vis struct #self_name{
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
    }
}
