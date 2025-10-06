# unixodbc-sys

[![Docs](https://docs.rs/unixodbc-sys/badge.svg)](https://docs.rs/unixodbc-sys/)
[![LGPL licensed](https://img.shields.io/badge/license-LGPL--2.1--or--later-blue.svg)](./LICENSE)
[![Crates.io version](https://img.shields.io/crates/v/unixodbc-sys)](https://crates.io/crates/unixodbc-sys)

FFI (Foreign Function Interface) bindings for ODBC. Low-level bindings intended as a foundation for higher-level libraries.

#### Links to
- UNIX: `libodbc` (typically [unixODBC](http://www.unixodbc.org/))
  - Needs installing when the `static` feature is not enabled:
    - Debian/Ubuntu: `sudo apt install unixodbc-dev`
    - Fedora: `sudo dnf install unixODBC-devel`
    - macOS: `brew install unixodbc`
- Windows: `odbc32.dll` (built-in, nothing to install)

## License

Licensed under LGPL-2.1-or-later to match unixODBC's license.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
unixodbc-sys = "0.1"
```

## Static Linking

UNIX only - Enable the `static` feature to compile unixODBC from source:

```toml
[dependencies]
unixodbc-sys = { version = "0.1", features = ["static"] }
```

Requires a C compiler and Git submodules initialized. Not available on Windows.

## Documentation

See the main repository [README](../README.md) for more details.

