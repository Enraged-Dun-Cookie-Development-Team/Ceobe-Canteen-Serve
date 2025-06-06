mod payloads;
mod codegen;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::codegen::ErrorGen;
use crate::payloads::ErrorCfg;

pub fn encode_response_error(config:impl AsRef<Path>){
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let outfile = out_dir.join("error_cfg.rs");
    // rerun
    println!("cargo::rustc-env=ERR_CFG_PATH={}",outfile.display());
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed={}",config.as_ref().display());


    let cfg = std::fs::read_to_string(config).expect("Error Config File Not Exist");
    let err_cfg = toml::from_str::<ErrorCfg>(&cfg).expect("Invalid Config File Format");
    
    let err_group = ErrorGen::from_error_cfg(&err_cfg)
        .into_iter()
        .map(|(ident,errors)|ErrorGen::generate_kind_code(ident,&errors))
        ;
    
    let mut out = File::options().write(true).create(true).open(outfile).expect("Cannot Create File");
    for error_kind_group in err_group{
        out.write_all(error_kind_group.to_string().as_bytes()).expect("Cannot write to file")
    }
}