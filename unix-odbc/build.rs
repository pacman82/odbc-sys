use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let vendor_dir = Path::new("./unixODBC");

    let mut build = cc::Build::new();

    build.includes([vendor_dir, vendor_dir.parent().unwrap()]);
    let header_dirs = ["include", "DriverManager", "odbcinst", "ini", "log", "lst"];
    build.includes(header_dirs.iter().map(|s| vendor_dir.join(s)));

    setup_compiler_flags(&mut build);

    let configure_ac = vendor_dir.join("configure.ac");
    build.define("VERSION", Some(unixodbc_version(&configure_ac).as_str()));

    let source_dirs = ["DriverManager", "odbcinst", "ini", "log", "lst"];
    build.files(c_files_in_dirs(vendor_dir, &source_dirs));

    build.compile("odbc");
    println!("cargo:rerun-if-changed={}", vendor_dir.display());
}

fn unixodbc_version(configure_ac_path: &Path) -> String {
    use std::{fs::File, io::BufRead, io::BufReader};
    let content = File::open(configure_ac_path).expect("Failed to read configure.ac");

    for line in BufReader::new(content).lines() {
        let line = line.expect("Failed to read line from configure.ac");
        if line.trim_start().starts_with("AC_INIT") {
            let version = line.split(['[', ']']).nth(2);
            if let Some(version) = version {
                return format!("\"{}\"", version);
            }
        }
    }
    panic!("Failed to find version in configure.ac");
}

fn c_files_in_dirs(base_dir: &Path, paths: &[&str]) -> Vec<PathBuf> {
    paths
        .iter()
        .map(|p| base_dir.join(p))
        .flat_map(|dir| fs::read_dir(dir).expect("read source directory"))
        .map(|p| p.expect("read source file").path())
        .filter(|p| p.extension().unwrap_or_default() == "c")
        .collect()
}

fn setup_compiler_flags(build: &mut cc::Build) {
    build.flag_if_supported("-std=gnu99");
    build.flag_if_supported("-Wno-pointer-to-int-cast");
    build.warnings(false);
}
