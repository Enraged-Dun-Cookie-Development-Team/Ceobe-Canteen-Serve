use darling::FromField;



#[derive(Debug,FromField)]
#[darling(attributes(select_only),and_then="Self::verify_valid")]
pub struct ModelFieldDefine{
    pub(crate) ident:Option<syn::Ident>,
    #[darling(default)]
    pub(crate) from_col:Option<syn::Ident>,
}

impl ModelFieldDefine {
    fn verify_valid(self)->darling::Result<Self>{
        if self.ident.is_none(){
            Err(darling::Error::unexpected_type("Tuple Struct"))?;
        }
        Ok(self)
    }
}