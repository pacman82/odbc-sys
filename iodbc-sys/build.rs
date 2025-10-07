fn main() {
    link_odbc();
}

#[cfg(not(feature = "static"))]
fn link_odbc() {}

#[cfg(feature = "static")]
use std::path::{Path, PathBuf};

#[cfg(feature = "static")]
fn link_odbc() {
    if let Ok(static_path) = std::env::var("ODBC_SYS_STATIC_PATH") {
        println!("cargo:rerun-if-env-changed=ODBC_SYS_STATIC_PATH");
        println!("cargo:rustc-link-search=native={static_path}");
        println!("cargo:rustc-link-lib=static=iodbc");
        return;
    }
    compile_odbc_from_source();
}

#[cfg(feature = "static")]
fn ensure_configured(vendor_dir: &Path) {
    assert!(
        vendor_dir.join("configure.ac").is_file(),
        "Submodule at {} not initialized. Run: git submodule update --init --recursive",
        vendor_dir.canonicalize().unwrap().display()
    );
}

#[cfg(feature = "static")]
fn iodbc_version(configure_ac_path: &Path) -> String {
    use std::{fs::File, io::BufRead, io::BufReader};

    let content = File::open(configure_ac_path).expect("Failed to read configure.ac");

    let defines = ["V_iodbc_major", "V_iodbc_minor", "V_iodbc_patch"];
    let mut version_components = [None, None, None];

    for line in BufReader::new(content).lines() {
        let line = line.expect("Failed to read line");
        for (i, &define) in defines.iter().enumerate() {
            if line.contains(define) && line.contains("m4_define") {
                version_components[i] = get_version(&line, 1);
            }
        }
    }

    let version: Vec<String> = version_components
        .into_iter()
        .map(|c| c.expect("version not found").trim().to_string())
        .collect();
    format!("\"{}\"", version.join("."))
}

#[cfg(feature = "static")]
fn get_version(line: &str, n: usize) -> Option<String> {
    line.split(['[', ']']).nth(n).map(str::to_lowercase)
}

#[cfg(feature = "static")]
fn compile_odbc_from_source() {
    let iodbc_src = Path::new("vendor/iODBC");

    ensure_configured(iodbc_src);

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_dir_path = Path::new(&out_dir);

    std::fs::copy("vendor/iodbc_config.h", out_dir_path.join("config.h"))
        .expect("Failed to copy iodbc_config.h to OUT_DIR/config.h");

    let mut build = cc::Build::new();

    build.includes([out_dir_path, iodbc_src]);
    let include_dirs = ["include", "iodbc", "iodbcinst", "iodbcadm"];
    build.includes(include_dirs.iter().map(|s| iodbc_src.join(s)));

    setup_compiler_flags(&mut build);

    build.define("HAVE_CONFIG_H", None);

    let configure_ac = iodbc_src.join("configure.ac");
    build.define("VERSION", Some(iodbc_version(&configure_ac).as_str()));

    let source_dirs = ["iodbc", "iodbc/trace", "iodbcinst"];
    build.files(c_files_in_dirs(iodbc_src, &source_dirs));
    build.compile("iodbc");

    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=iconv");
    }

    println!("cargo:rerun-if-changed=vendor/iODBC");
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
