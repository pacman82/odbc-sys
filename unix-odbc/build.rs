// use std::env;

use std::{env, fs::File, path::PathBuf};

use autotools::Config;
use flate2::read::GzDecoder;
use tar::Archive;

fn main() {
    // Extract unix-odbc source to OUT_DIR so we do not pollute our source directory.
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = PathBuf::from(out_dir);
    let tar_gz = File::open("unixODBC-2.3.9.tar.gz").unwrap();
    let mut archive = Archive::new(GzDecoder::new(tar_gz));
    archive.unpack(&out_dir).unwrap();

    let unix_odbc_source_path = out_dir.join("unixODBC-2.3.9");
    let path = Config::new(unix_odbc_source_path)
        .cflag("-std=gnu99")
        .build();
    let path = path.join("lib");
    println!("cargo:rustc-link-search=native={}", path.display());
}
