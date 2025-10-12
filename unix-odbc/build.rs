use autotools::Config;

fn main() {
    if cfg!(target_os = "windows") {
        return;
    }

    let path = Config::new("unixODBC")
        .cflag("-std=gnu99")
        // Prevent recreating the configure script
        .env("ACLOCAL", "true")
        .env("AUTOMAKE", "true")
        .env("AUTOCONF", "true")
        .build();
    let path = path.join("lib");
    println!("cargo:rustc-link-search=native={}", path.display());
}
