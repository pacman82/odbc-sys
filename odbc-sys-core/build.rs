use std::process::Command;

fn main() {
    if cfg!(target_os = "macos") {
        if let Some(homebrew_lib_path) = homebrew_library_path() {
            println!("cargo:rustc-link-search=native={}", homebrew_lib_path);
        }

        if let Ok(dyld_paths) = std::env::var("DYLD_LIBRARY_PATH") {
            print_paths(&dyld_paths);
        }
        if let Ok(dyld_fallback_paths) = std::env::var("DYLD_FALLBACK_LIBRARY_PATH") {
            print_paths(&dyld_fallback_paths);
        }
    }
}

fn print_paths(paths: &str) {
    for path in paths.split(':').filter(|x| !x.is_empty()) {
        println!("cargo:rustc-link-search=native={path}")
    }
}

fn homebrew_library_path() -> Option<String> {
    let output = Command::new("brew").arg("--prefix").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let prefix =
        String::from_utf8(output.stdout).expect("brew --prefix must yield utf8 encoded response");
    let prefix = prefix.trim();
    let lib_path = prefix.to_owned() + "/lib";
    Some(lib_path)
}
