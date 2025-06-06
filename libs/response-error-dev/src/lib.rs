mod payloads;
mod codegen;

use std::path::Path;

pub fn encode_response_error(config:impl AsRef<Path>){
    // rerun
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed={}",config.as_ref().display());
    
    
}