use darling::{FromDeriveInput, FromField};
use darling_models::DeriveAttrInfo;
use proc_macro::TokenStream;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed,
};

use crate::{
    darling_models::{DeriveField, FieldInfo},
    models::{DefaultEmpty, SubModel, WantAll},
};

mod darling_models;
mod models;

#[proc_macro_derive(SubModel, attributes(sub_model))]
pub fn derive_sub_model(input: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(input as DeriveInput);
    let derive_info =
        <DeriveAttrInfo as FromDeriveInput>::from_derive_input(&derive)
            .expect("Derive Info Cannot Load");

    let mut map = derive_info.inner.to_models_map();

    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { named, .. }),
        ..
    }) = derive.data
    {
        named
    }
    else {
        panic!("only support Named Struct")
    };

    for field in fields.clone() {
        let extra = <DeriveField as FromField>::from_field(&field)
            .expect("Cannot Load Field Info");

        let v = extra.field.inner;

        let field_ident = field.ident.unwrap();

        for field_info in v {
            let ident = field_info.get_ident();
            match (map.get_mut(ident).expect("Unknown SubModel"), &field_info)
            {
                (
                    SubModel::DefaultAll(WantAll { ignores, .. }),
                    FieldInfo::Ignore(_),
                ) => {
                    ignores
                        .push(field_info.to_field_meta(field_ident.clone()))
                }
                (
                    SubModel::DefaultEmpty(DefaultEmpty { wants, .. }),
                    FieldInfo::Want(_),
                ) => {
                    wants.push(field_info.to_field_meta(field_ident.clone()));
                }
                (
                    SubModel::DefaultAll(WantAll { having_change, .. }),
                    FieldInfo::Having(_),
                ) => {
                    having_change
                        .push(field_info.to_field_meta(field_ident.clone()))
                }
                (SubModel::DefaultEmpty(_), FieldInfo::Ignore(_)) => {
                    panic!("Default Empty Can not In None Block")
                }
                (SubModel::DefaultAll(_), FieldInfo::Want(_)) => {
                    panic!("Default All SubModel Can not In Want block")
                }
                (SubModel::DefaultEmpty(_), FieldInfo::Having(_)) => {
                    panic!("`having` is for Want All")
                }
            }
        }
    }

    let fields = fields.into_iter().map(|f| (f.ident.unwrap(), f.ty));

    let sub_models = map
        .into_iter()
        .map(|(_k, v)| v.into_def(derive.ident.clone(), fields.clone()));

    quote::quote! {
        #(
            #sub_models
        )*

    }
    .into()
}

#[cfg(test)]
mod test {
    use darling::{FromDeriveInput, FromField};
    use syn::{DataStruct, DeriveInput};

    use crate::darling_models::{DeriveAttrInfo, DeriveField};

    #[test]
    fn test_none() {
        let code = r#"
            #[derive(SubModel)]
            struct Model{
                a:u32,
                b:u8
            }
        "#;

        let v = syn::parse_str(code).expect("Bad Code");
        let _v = <DeriveAttrInfo as FromDeriveInput>::from_derive_input(&v)
            .unwrap();

            println!("{:?}",&_v)
    }

    #[test]
    fn test_all_only() {
        let code = r#"
            #[derive(SubModel)]
            #[sub_model(all(name="Verified"),all("Basic"))]
            struct Model{
                a:u32,
                b:u8
            }
        "#;

        let v = syn::parse_str(code).expect("Bad Code");
        let _v = <DeriveAttrInfo as FromDeriveInput>::from_derive_input(&v)
            .unwrap();

        println!("{:?}", &_v)
    }

    #[test]
    fn test_field() {
        let code = r#"
        #[derive(SubModel)]
        #[sub_model(all(name="Verified",vis=""),all("Basic"),none("Empty"))]
        struct Model{
            #[sub_model(
                want(
                    for="Empty",
                    rename="c",
                    extra(
                        serde(rename="abab"),
                        serde(alias="ccc")
                    )
                ),
                ignore("Verified")
            )]
            a:u32,
            #[sub_model(having(
                for="Verified",
                rename="cca",
                extra(
                    doc="只有b\n  ",
                    doc="也许不错"
                )
            ))]
            b:String

        }
        "#;
        let v: DeriveInput = syn::parse_str(code).expect("Bad Code");
        let v = v.data;
        let v = match v {
            syn::Data::Struct(DataStruct { fields, .. }) => fields,
            _ => {
                unreachable!()
            }
        };

        for f in v {
            let info = <DeriveField as FromField>::from_field(&f).unwrap();

            println!("value {:#?}", info);
        }
    }
}
