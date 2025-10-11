fn main() {
    if cfg!(target_os = "windows") {
        return;
    }
    let path = autotools::Config::new("unixODBC")
        .cflag("-std=gnu99")
        .build();
    println!("cargo:rustc-link-search=native={}/lib", path.display());
    println!("cargo:rustc-link-lib=static=odbc");
}
