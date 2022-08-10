macro_rules! syn_error {
    ($exp:expr) => {
        match $exp {
            Ok(var) => var,
            Err(err) => {
                return proc_macro::TokenStream::from(err.into_compile_error())
            }
        }
    };
}
macro_rules! darling_error {
    ($exp:expr) => {
        match $exp {
            Ok(var) => var,
            Err(err) => {
                return proc_macro::TokenStream::from(err.write_errors())
            }
        }
    };
}
