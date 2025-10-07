#[allow(unused_imports)]
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    link_odbc();

    if cfg!(target_os = "macos") {
        if let Some(homebrew_lib_path) = homebrew_library_path() {
            print_paths(&homebrew_lib_path);
        }

        if let Some(dyld_paths) = option_env!("DYLD_LIBRARY_PATH") {
            print_paths(dyld_paths);
        }
        if let Some(dyld_fallback_paths) = option_env!("DYLD_FALLBACK_LIBRARY_PATH") {
            print_paths(dyld_fallback_paths);
        }
    }
}

#[cfg(feature = "static")]
fn link_odbc() {
    if let Ok(static_path) = std::env::var("ODBC_SYS_STATIC_PATH") {
        println!("cargo:rerun-if-env-changed=ODBC_SYS_STATIC_PATH");
        println!("cargo:rustc-link-search=native={static_path}");
        println!("cargo:rustc-link-lib=static=odbc");
        println!("cargo:rustc-link-lib=static=ltdl");
        if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-lib=dylib=iconv");
        }
        return;
    }
    compile_odbc_from_source();
}

#[cfg(not(feature = "static"))]
fn link_odbc() {}

#[cfg(feature = "static")]
fn ensure_configured(vendor_dir: &Path) {
    assert!(
        vendor_dir.join("configure.ac").is_file(),
        "Submodule at {} not initialized. Run: git submodule update --init --recursive",
        vendor_dir.canonicalize().unwrap().display()
    );
}

#[cfg(feature = "static")]
fn unixodbc_version(configure_ac_path: &Path) -> String {
    use std::{fs::File, io::BufRead, io::BufReader};
    let content = File::open(configure_ac_path).expect("Failed to read configure.ac");

    for line in BufReader::new(content).lines() {
        let line = line.expect("Failed to read line from configure.ac");
        if line.trim_start().starts_with("AC_INIT") {
            if let Some(version) = get_version(&line, 2) {
                return format!("\"{}\"", version);
            }
        }
    }
    panic!("Failed to find version in configure.ac");
}

#[cfg(feature = "static")]
fn get_version(line: &str, n: usize) -> Option<String> {
    line.split(['[', ']']).nth(n).map(str::to_lowercase)
}

#[cfg(feature = "static")]
fn compile_odbc_from_source() {
    let vendor_dir = Path::new("vendor/unixODBC");

    ensure_configured(vendor_dir);

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_dir_path = Path::new(&out_dir);

    std::fs::copy("vendor/unixodbc_config.h", out_dir_path.join("config.h"))
        .expect("copy unixodbc_config.h to OUT_DIR/config.h");

    std::fs::copy("vendor/ltdl.h", out_dir_path.join("ltdl.h"))
        .expect("copy ltdl.h to OUT_DIR/ltdl.h");

    let mut build = cc::Build::new();

    build.includes([out_dir_path, vendor_dir]);
    let header_dirs = ["include", "DriverManager", "odbcinst", "ini", "log", "lst"];
    build.includes(header_dirs.iter().map(|s| vendor_dir.join(s)));

    setup_compiler_flags(&mut build);

    let configure_ac = vendor_dir.join("configure.ac");
    build.define("VERSION", Some(unixodbc_version(&configure_ac).as_str()));

    let source_dirs = ["DriverManager", "odbcinst", "ini", "log", "lst"];
    build.files(c_files_in_dirs(vendor_dir, &source_dirs));

    build.compile("odbc");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=iconv");
    }
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");

    println!("cargo:rerun-if-changed=vendor/unixODBC");
}

#[cfg(feature = "static")]
fn c_files_in_dirs(base_dir: &Path, paths: &[&str]) -> Vec<PathBuf> {
    paths
        .iter()
        .map(|p| base_dir.join(p))
        .flat_map(|dir| std::fs::read_dir(dir).expect("read source directory"))
        .map(|p| p.expect("read source file").path())
        .filter(|p| p.extension().unwrap_or_default() == "c")
        .collect()
}

#[cfg(feature = "static")]
fn setup_compiler_flags(build: &mut cc::Build) {
    build.flag_if_supported("-fPIC");
    build.flag_if_supported("-std=gnu99");
    build.flag_if_supported("-Wno-error");
    build.flag_if_supported("-Wno-implicit-function-declaration");
    build.flag_if_supported("-Wno-int-conversion");
    build.flag_if_supported("-w");
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
