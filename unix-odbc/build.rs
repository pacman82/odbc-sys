use std::fs::{self, read_to_string};
use std::path::{Path, PathBuf};

fn main() {
    let vendor_dir = Path::new("unixODBC");
    let src_dirs: Vec<PathBuf> = ["DriverManager", "odbcinst", "ini", "log", "lst"]
        .into_iter()
        .map(|s| vendor_dir.join(s))
        .collect();

    let configure_ac_path = vendor_dir.join("configure.ac");
    let configure_ac = read_to_string(configure_ac_path).expect("Failed to read configure.ac");
    let (version, _) = configure_ac
        .split_once("AC_INIT([unixODBC], [")
        .and_then(|(_, s)| s.split_once(']'))
        .expect("Failed to parse version from configure.ac");
    let version_str = format!("\"{}\"", version);

    let mut build = cc::Build::new();
    build
        .includes([vendor_dir, Path::new("."), &vendor_dir.join("include")])
        .includes(&src_dirs)
        .files(
            src_dirs
                .iter()
                .flat_map(|dir| fs::read_dir(dir).expect("read source directory"))
                .map(|p| p.expect("read source file").path())
                .filter(|p| p.extension().unwrap_or_default() == "c"),
        )
        .define("VERSION", version_str.as_str())
        .flag_if_supported("-std=gnu99")
        .flag_if_supported("-Wno-pointer-to-int-cast")
        .warnings(false)
        .compile("odbc");
}
