use std::fs;

fn main() {
    if cfg!(target_os = "windows") {
        return;
    }
    
    // Touch all .in and .m4 files recursively before compilation
    //touch_autotools_files("unixODBC");
    
    let path = autotools::Config::new("unixODBC")
        .cflag("-std=gnu99")
        .build();
    println!("cargo:rustc-link-search=native={}/lib", path.display());
    println!("cargo:rustc-link-lib=static=odbc");
}

fn touch_autotools_files(dir: &str) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Recursively process subdirectories
                if let Some(path_str) = path.to_str() {
                    touch_autotools_files(path_str);
                }
            } else if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                // Touch .in and .m4 files
                if file_name.ends_with(".in") || file_name.ends_with(".m4") {
                    if let Err(e) = fs::File::create(&path) {
                        eprintln!("Warning: Failed to touch {}: {}", path.display(), e);
                    }
                }
            }
        }
    }
}
