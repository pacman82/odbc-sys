#[allow(unused_imports)]
use std::path::Path;
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
fn extract_iodbc_version(configure_ac_path: &Path) -> Option<String> {
    let content = std::fs::read_to_string(configure_ac_path).ok()?;

    let mut major = None;
    let mut minor = None;
    let mut patch = None;

    for line in content.lines() {
        if line.contains("m4_define(V_iodbc_major") {
            major = line
                .split('[')
                .nth(1)?
                .split(']')
                .next()
                .map(|s| s.trim().to_string());
        } else if line.contains("m4_define(V_iodbc_minor") {
            minor = line
                .split('[')
                .nth(1)?
                .split(']')
                .next()
                .map(|s| s.trim().to_string());
        } else if line.contains("m4_define(V_iodbc_patch") {
            patch = line
                .split('[')
                .nth(1)?
                .split(']')
                .next()
                .map(|s| s.trim().to_string());
        }
    }

    if let (Some(maj), Some(min), Some(pat)) = (major, minor, patch) {
        Some(format!("{}.{}.{}", maj, min, pat))
    } else {
        None
    }
}

#[cfg(all(feature = "static", not(feature = "iodbc")))]
fn compile_odbc_from_source() {
    let vendor_dir = Path::new("vendor/unixODBC");

    ensure_configured(vendor_dir);

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_dir_path = Path::new(&out_dir);

    std::fs::copy("vendor/unixodbc_config.h", out_dir_path.join("config.h"))
        .expect("Failed to copy unixodbc_config.h to OUT_DIR/config.h");

    std::fs::copy("vendor/ltdl.h", out_dir_path.join("ltdl.h"))
        .expect("Failed to copy ltdl.h to OUT_DIR/ltdl.h");

    let mut build = cc::Build::new();

    build.include(out_dir_path);
    build.include(vendor_dir.join("include"));
    build.include(vendor_dir.join("DriverManager"));
    build.include(vendor_dir.join("odbcinst"));
    build.include(vendor_dir.join("ini"));
    build.include(vendor_dir.join("log"));
    build.include(vendor_dir.join("lst"));
    build.include(vendor_dir);

    build.flag_if_supported("-fPIC");
    build.flag_if_supported("-std=gnu99");
    build.flag_if_supported("-Wno-error");
    build.flag_if_supported("-Wno-implicit-function-declaration");
    build.flag_if_supported("-Wno-int-conversion");
    build.flag_if_supported("-w");

    let configure_ac = vendor_dir.join("configure.ac");
    let version =
        extract_version_from_configure_ac(&configure_ac).unwrap_or_else(|| "unknown".to_string());
    let version_str = format!("\"{}\"", version);
    build.define("VERSION", version_str.as_str());

    // Collect all source files from DriverManager
    let driver_manager_dir = vendor_dir.join("DriverManager");
    add_c_files(&mut build, &driver_manager_dir);

    // Collect all source files from odbcinst
    let odbcinst_dir = vendor_dir.join("odbcinst");
    add_c_files(&mut build, &odbcinst_dir);

    // Collect all source files from ini
    let ini_dir = vendor_dir.join("ini");
    add_c_files(&mut build, &ini_dir);

    // Collect all source files from log
    let log_dir = vendor_dir.join("log");
    add_c_files(&mut build, &log_dir);

    // Collect all source files from lst
    let lst_dir = vendor_dir.join("lst");
    add_c_files(&mut build, &lst_dir);

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
    let vendor_dir = Path::new("vendor/iODBC");

    ensure_configured(vendor_dir);

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_dir_path = Path::new(&out_dir);

    std::fs::copy("vendor/iodbc_config.h", out_dir_path.join("config.h"))
        .expect("Failed to copy iodbc_config.h to OUT_DIR/config.h");

    let mut build = cc::Build::new();

    build.include(out_dir_path);
    build.include(vendor_dir.join("include"));
    build.include(vendor_dir.join("iodbc"));
    build.include(vendor_dir.join("iodbcinst"));
    build.include(vendor_dir.join("iodbcadm"));
    build.include(vendor_dir);

    build.flag_if_supported("-fPIC");
    build.flag_if_supported("-std=gnu99");
    build.flag_if_supported("-Wno-error");
    build.flag_if_supported("-Wno-implicit-function-declaration");
    build.flag_if_supported("-Wno-int-conversion");
    build.flag_if_supported("-w");

    build.define("HAVE_CONFIG_H", None);

    let configure_ac = vendor_dir.join("configure.ac");
    let version = extract_iodbc_version(&configure_ac).unwrap_or_else(|| "unknown".to_string());
    let version_str = format!("\"{}\"", version);
    build.define("VERSION", version_str.as_str());

    // Collect all source files from iodbc
    let iodbc_dir = vendor_dir.join("iodbc");
    add_c_files(&mut build, &iodbc_dir);

    // Collect trace files from iodbc/trace
    let trace_dir = vendor_dir.join("iodbc/trace");
    add_c_files(&mut build, &trace_dir);

    // Collect all source files from iodbcinst
    let iodbcinst_dir = vendor_dir.join("iodbcinst");
    add_c_files(&mut build, &iodbcinst_dir);

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
fn add_c_files(build: &mut cc::Build, dir: &Path) {
    if !dir.exists() {
        return;
    }

    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "c" {
                    // Skip certain files that shouldn't be compiled
                    let filename = path.file_name().unwrap().to_str().unwrap();

                    // Skip test files, utilities, and GUI components
                    if filename.starts_with("test")
                        || filename == "dltest.c"
                        || filename == "isql.c"
                        || filename == "iusql.c"
                        || filename == "odbcinst.c"
                        || filename == "odbc-config.c"
                        || filename == "slencheck.c"
                    {
                        continue;
                    }

                    build.file(&path);
                }
            }
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
    // brew returns also a linebreak (`\n`), we want to get rid of that.
    let prefix = prefix.trim();
    let lib_path = prefix.to_owned() + "/lib";
    Some(lib_path)
}
