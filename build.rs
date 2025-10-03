#[allow(unused_imports)]
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    link_odbc();

    if cfg!(target_os = "macos") {
        if let Some(homebrew_lib_path) = homebrew_library_path() {
            print_paths(&homebrew_lib_path);
        }

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

#[cfg(feature = "static")]
fn link_odbc() {
    assert!(
        !cfg!(target_os = "windows"),
        "odbc-sys does not currently support static linking on windows"
    );

    // Check if user wants to provide their own static library path
    if let Ok(static_path) = std::env::var("ODBC_SYS_STATIC_PATH") {
        // User-provided static library path (original behavior)
        println!("cargo:rerun-if-env-changed=ODBC_SYS_STATIC_PATH");
        println!("cargo:rustc-link-search=native={static_path}");
        println!("cargo:rustc-link-lib=static=odbc");
        println!("cargo:rustc-link-lib=static=ltdl");
        if cfg!(target_os = "macos") {
            // Homebrew's unixodbc uses the system iconv, so we can't do a fully static linking
            // but this way we at least have only dependencies on built-in libraries
            // See also https://github.com/Homebrew/homebrew-core/pull/46145
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

#[cfg(all(feature = "static", not(feature = "iodbc")))]
fn extract_version_from_configure_ac(configure_ac_path: &Path) -> Option<String> {
    let content = std::fs::read_to_string(configure_ac_path).ok()?;

    // Look for AC_INIT line with version
    // Format: AC_INIT([name], [version], ...)
    for line in content.lines() {
        if line.trim_start().starts_with("AC_INIT") {
            // Extract version from AC_INIT([name], [version], ...)
            if let Some(version_part) = line.split('[').nth(2) {
                if let Some(version) = version_part.split(']').next() {
                    let version = version.trim();
                    // Skip if it contains macro substitutions
                    if !version.contains("V_") && !version.is_empty() {
                        return Some(version.to_string());
                    }
                }
            }
        }
    }
    None
}

#[cfg(all(feature = "static", feature = "iodbc"))]
fn extract_iodbc_version(configure_ac_path: &Path) -> String {
    use std::{fs::File, io::BufRead, io::BufReader};

    let content = File::open(configure_ac_path).expect("Failed to read configure.ac");

    let mut major = None;
    let mut minor = None;
    let mut patch = None;

    fn get_version(line: &str) -> Option<String> {
        line.split(['[', ']']).nth(1).map(str::to_lowercase)
    }

    for line in BufReader::new(content).lines() {
        let line = line.expect("Failed to read line");
        if line.contains("m4_define(V_iodbc_major") {
            major = get_version(&line);
        } else if line.contains("m4_define(V_iodbc_minor") {
            minor = get_version(&line);
        } else if line.contains("m4_define(V_iodbc_patch") {
            patch = get_version(&line);
        }
    }

    format!(
        "{}.{}.{}",
        major.expect("major version not found").trim(),
        minor.expect("minor version not found").trim(),
        patch.expect("patch version not found").trim()
    )
}

#[cfg(all(feature = "static", not(feature = "iodbc")))]
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
    let version =
        extract_version_from_configure_ac(&configure_ac).unwrap_or_else(|| "unknown".to_string());
    let version_str = format!("\"{}\"", version);
    build.define("VERSION", version_str.as_str());

    let source_dirs = ["DriverManager", "odbcinst", "ini", "log", "lst"];
    build.files(c_files_in_dirs(&vendor_dir, &source_dirs));

    build.compile("odbc");

    // Link additional dependencies
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=iconv");
    }
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");

    println!("cargo:rerun-if-changed=vendor/unixODBC");
}

#[cfg(all(feature = "static", feature = "iodbc"))]
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
    let version = extract_iodbc_version(&configure_ac);
    let version_str = format!("\"{}\"", version);
    build.define("VERSION", version_str.as_str());

    // Collect all source files from all directories
    let source_dirs = ["iodbc", "iodbc/trace", "iodbcinst"];
    build.files(c_files_in_dirs(&iodbc_src, &source_dirs));
    build.compile("iodbc");

    // Link pthread for iODBC
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
    // brew returns also a linebreak (`\n`), we want to get rid of that.
    let prefix = prefix.trim();
    let lib_path = prefix.to_owned() + "/lib";
    Some(lib_path)
}
