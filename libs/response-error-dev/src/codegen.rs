use http::StatusCode;
use quote::{format_ident, quote};
use crate::payloads::ErrorType;
pub struct ErrorGen<'s>{
    mark:char,
    mark_description:&'s str,
    status_code:StatusCode,
    ident:&'s str,
    description:&'s str,
    code:u16
}
impl<'s> ErrorGen<'s> {
    
    pub fn generate_code(&self)->String{
        let ErrorGen{ mark, status_code, ident, description, code,mark_description} = self;
        let doc_description =format!("## 响应异常  \n- `{mark}`: {mark_description}  \n- ErrorCode: {mark}{code:04x}: {description}  \n- HttpCode: {status_code}");
        let status_code = status_code.as_u16();
        let ident = format_ident!("{ident}");
        
        let code = quote! {
            #[doc=#doc_description]
            pub struct #ident;
            
            impl #ident{
                pub fn mark(&self)->char{ #mark}
                
                pub fn status_code(&self)->http::StatusCode {http::StatusCode::from_u16(#status_code).unwrap()}
                
                pub fn description(&self)->&'static str{#description};
                
                pub fn code(&self)->u16 {#code}
            }
        };
        
        code.to_string()
    }
    
}

impl<'s> ErrorGen<'s> {
    pub fn from_error_type(error:&'s ErrorType)->Vec<Self>{
        let mut out = Vec::new();
        
        for (code,err) in error.error.iter().enumerate(){
           let gen = ErrorGen{
               mark:error.mark,
               mark_description: &error.description,
               status_code: err.http_code.unwrap_or(error.default_status_code),
               ident: &err.ident,
               description: &err.description,
               code: (code as u16) +1,
           } ;
            out.push(gen)
        }
        
        out
    }
    
    pub fn from_error_type_list(errors:&'s[ErrorType])->Vec<Self>{
        let mut out = Vec::new();
        
        for error in errors{
            let v = Self::from_error_type(error);
            out.extend(v)
        }
        out
    }
}
#[cfg(test)]
mod test{
    use crate::codegen::ErrorGen;
    use crate::payloads::ErrorType;

    #[test]
    fn test(){
        let v = include_str!("../../.././example_error_config.toml");
        let payload:ErrorType = toml::from_str(v).expect("Error");

        let err = ErrorGen::from_error_type(&payload).remove(0);
        
        let gen = err.generate_code();
        
        println!("{gen}")
    }
}