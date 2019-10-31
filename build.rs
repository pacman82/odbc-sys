fn main() {
    if cfg!(r#static) {
        let static_path = std::env::var("ODBC_SYS_STATIC_PATH").unwrap_or("/usr/lib".to_string());
        println!("cargo:rerun-if-env-changed=ODBC_SYS_STATIC_PATH");
        println!("cargo:rustc-link-search=native={}", static_path);
        println!("cargo:rustc-link-lib=static=odbc");
    }

    if cfg!(target_os = "macos") {
        // if we're on Mac OS X we'll kindly add DYLD_LIBRARY_PATH to rustc's
        // linker search path
        if let Some(dyld_paths) = option_env!("DYLD_LIBRARY_PATH") {
            print_paths(dyld_paths);
        }
        // if we're on Mac OS X we'll kindly add DYLD_FALLBACK_LIBRARY_PATH to rustc's
        // linker search path
        if let Some(dyld_fallback_paths) = option_env!("DYLD_FALLBACK_LIBRARY_PATH") {
            print_paths(dyld_fallback_paths);
        }
    }
}

fn print_paths(paths: &str) {
    for path in paths.split(':').filter(|x| !x.is_empty()) {
        println!("cargo:rustc-link-search=native={}", path)
    }
}
