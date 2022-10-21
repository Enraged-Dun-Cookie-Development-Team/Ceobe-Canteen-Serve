fn main() {
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-arg=/STACK:2097152")
}
