fn main() {
    let path = autotools::build("unixODBC");
    println!("cargo:rustc-link-search=native={}/lib", path.display());
    println!("cargo:rustc-link-lib=static=odbc");
}
