fn main() {
    #[cfg(all(target_os = "windows", debug_assertions))]
    println!("cargo:rustc-link-arg=/STACK:2097152")
}
