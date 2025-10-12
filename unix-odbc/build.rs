use std::{env, path::PathBuf};

use autotools::Config;
use fs_extra::dir::{copy, CopyOptions};

fn main() {
    if cfg!(target_os = "windows") {
        return;
    }

    // OUT_DIR is an environment provided by cargo. It points to a temporary build directory. This
    // allows us to keep our source tree clean.
    let out_dir: PathBuf = env::var("OUT_DIR").unwrap().parse().unwrap();
    let unix_odbc_src = out_dir.join("unixODBC");

    copy("unixODBC", &unix_odbc_src, &CopyOptions::new()).unwrap();

    let path = Config::new(&unix_odbc_src)
        .cflag("-std=gnu99")
        .build();
    let path = path.join("lib");
    println!("cargo:rustc-link-search=native={}", path.display());
}
