/// CLI 入口：在 CI 中调用以验证配置或生成 Wiki。
///
/// 用法：
///   cargo run --example error-config-cli -- validate <config-path>
///   cargo run --example error-config-cli -- wiki <config-path>
/// <output-dir>
use std::{env, process};

use response_error_dev::{ErrorCfg, generate_wiki_markdown};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  error-config-cli validate <config-path>");
        eprintln!("  error-config-cli wiki <config-path> <output-dir>");
        process::exit(1);
    }

    match args[1].as_str() {
        "validate" => {
            if args.len() < 3 {
                eprintln!("Usage: error-config-cli validate <config-path>");
                process::exit(1);
            }
            let config_path = &args[2];
            println!("Validating config: {config_path}");

            let cfg = std::fs::read_to_string(config_path)
                .expect("Failed to read config file");
            let err_cfg: ErrorCfg =
                toml::from_str(&cfg).expect("Invalid TOML format");

            match err_cfg.validate() {
                Ok(()) => {
                    println!("✅ Config validation passed");
                }
                Err(errors) => {
                    eprintln!("❌ Config validation failed:");
                    for err in &errors {
                        eprintln!("   - {err}");
                    }
                    process::exit(1);
                }
            }
        }
        "wiki" => {
            if args.len() < 4 {
                eprintln!(
                    "Usage: error-config-cli wiki <config-path> <output-dir>"
                );
                process::exit(1);
            }
            let config_path = &args[2];
            let output_dir = &args[3];

            println!("Generating Wiki from: {config_path}");
            println!("Output to: {output_dir}");

            generate_wiki_markdown(config_path, output_dir)
                .expect("Failed to generate Wiki markdown");
            println!("✅ Wiki generated successfully");
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Available commands: validate, wiki");
            process::exit(1);
        }
    }
}
