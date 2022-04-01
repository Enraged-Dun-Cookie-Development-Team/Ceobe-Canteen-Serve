
crate::quick_trait! {
    pub AuthConfig{
        crate::trait_field!{*jwt_key:&[u8]}
        // crate::trait_field!{*token_head_name:Cow<'static,str>=Cow::Borrowed("Token")}

    }
}


