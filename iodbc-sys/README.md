# iodbc-sys

[![Docs](https://docs.rs/iodbc-sys/badge.svg)](https://docs.rs/iodbc-sys/)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io version](https://img.shields.io/crates/v/iodbc-sys)](https://crates.io/crates/iodbc-sys)

FFI (Foreign Function Interface) bindings for ODBC. Low-level bindings intended as a foundation for higher-level libraries.

#### Links to
- UNIX: `libiodbc` ([iODBC](https://www.iodbc.org/) - useful for older data sources)
  - Needs installing when the `static` feature is not enabled:
    - Debian/Ubuntu: `sudo apt install libiodbc2-dev`
    - macOS: `brew install libiodbc`
- Windows: `odbc32.dll` (built-in, nothing to install)

## License

Licensed under MIT to match iODBC's license.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
iodbc-sys = "0.1"
```

## Static Linking

UNIX only - Enable the `static` feature to compile iODBC from source:

```toml
[dependencies]
iodbc-sys = { version = "0.1", features = ["static"] }
```

Requires a C compiler and Git submodules initialized. Not available on Windows.

## Documentation

See the main repository [README](../README.md) for more details.

