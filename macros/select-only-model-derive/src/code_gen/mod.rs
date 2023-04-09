use self::{field_conv::SelectCols, trait_impl_code_gen::TraitImpl};
use crate::derive_input::container::DeriveInput;

pub mod field_conv;
pub mod trait_impl_code_gen;

pub(crate) fn to_code_gen(input: &DeriveInput) -> TraitImpl<'_> {
    let fields = input
        .fields
        .iter()
        .map(|field| SelectCols::from(field, &input.origin))
        .collect();

    TraitImpl {
        ident: &input.ident,
        fields,
    }
}
