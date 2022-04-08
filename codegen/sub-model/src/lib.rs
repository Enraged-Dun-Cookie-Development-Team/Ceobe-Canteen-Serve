use std::collections::HashMap;

use darling::{FromDeriveInput, FromField};
use darling_models::DeriveAttrInfo;
use proc_macro::TokenStream;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed,
};

use crate::{
    darling_models::{DeriveField, FieldInfo, IgnoreField, WantField},
    models::{DefaultEmpty, FieldMapper, SubModel, WantAll},
};

mod darling_models;
mod models;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[proc_macro_derive(SubModel, attributes(sub_model))]
pub fn derive_sub_model(input: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(input as DeriveInput);
    let derive_info =
        <DeriveAttrInfo as FromDeriveInput>::from_derive_input(&derive)
            .expect("Derive Info Cannot Load");

    let mut map = {
        let mut map = HashMap::new();
        for v in derive_info.all.inner {
            let all = WantAll {
                name: v.clone(),
                ignores: Vec::new(),
            };
            map.insert(v, SubModel::DefaultAll(all));
        }
        for v in derive_info.none.inner {
            let empty = DefaultEmpty {
                name: v.clone(),
                wants: Vec::new(),
            };
            map.insert(v, SubModel::DefaultEmpty(empty));
        }

        map
    };

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
        // println!("field extra {:?}, field Info {:?}", extra, field);

        let v = extra.field.inner;

        let field_ident = field.ident.unwrap();

        for field_info in v {
            let ident = field_info.get_ident();
            match (map.get_mut(ident).expect("Unknown SubModel"), field_info)
            {
                (
                    SubModel::DefaultAll(WantAll { ignores, .. }),
                    FieldInfo::Ignore(IgnoreField { .. }),
                ) => ignores.push(FieldMapper::Raw(field_ident.clone())),
                (
                    SubModel::DefaultEmpty(DefaultEmpty { wants, .. }),
                    FieldInfo::Want(WantField { to, .. }),
                ) => {
                    let mapper = match to {
                        Some(map_to) => {
                            FieldMapper::Mapping {
                                from: field_ident.clone(),
                                to: map_to,
                            }
                        }
                        None => FieldMapper::Raw(field_ident.clone()),
                    };
                    wants.push(mapper);
                }
                (SubModel::DefaultEmpty(_), FieldInfo::Ignore(_)) => {
                    panic!("Default Empty Can not In None Block")
                }
                (SubModel::DefaultAll(_), FieldInfo::Want(_)) => {
                    panic!("Default All SubModel Can not In Want block")
                }
            }
        }
    }

    let fields = fields
        .into_iter()
        .map(|f| (f.ident.unwrap(), f.ty))
        .collect::<Vec<_>>();

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
    }

    #[test]
    fn test_all_only() {
        let code = r#"
            #[derive(SubModel)]
            #[sub_model(all("Verified","Basic"))]
            struct Model{
                a:u32,
                b:u8
            }
        "#;

        let v = syn::parse_str(code).expect("Bad Code");
        let _v = <DeriveAttrInfo as FromDeriveInput>::from_derive_input(&v)
            .unwrap();
    }

    #[test]
    fn test_field() {
        let code = r#"
        #[derive(SubModel)]
        #[sub_model(all("Verified","Basic"),none("Empty"))]
        struct Model{
            #[sub_model(field(ignore(name="Verified",to="bbc")))]
            a:u32,
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

            println!("{:?}", info);
        }
    }
}
