# odbc-sys

[![Docs](https://docs.rs/odbc-sys/badge.svg)](https://docs.rs/odbc-sys/)
[![MIT licensed](https://img.shields.io/github/license/mashape/apistatus.svg)](https://github.com/pacman82/odbc-sys/blob/main/LICENSE)
[![Crates.io version](https://img.shields.io/crates/v/odbc-sys)](https://crates.io/crates/odbc-sys)

FFI (Foreign Function Interface) bindings for ODBC (Open Database Connectivity)
As ffi bindings to C-APIs are low level by nature this library is intended to be the foundation of
other libraries to build on top, rather than to be used directly.

## Design Goals

* Providing declarations of ODBC Symbols compatible to the C-Interface of an ODBC Driver Manager
* Provide correct definition of symbols for Unix and Windows in either 32Bit or 64Bit flavour
* Not to abstract away any power of the underlying API
* Increase type safety where feasible
* As it is as of now unlikely to happen that anyone is writing ODBC 2.0 applications in Rust
  therefore deprecated symbols like 'SQLAllocEnv' have been left out intentionally.

## Linking

This library will link against `odbc32.dll` (preinstalled) on Windows systems. On Linux and macOS it links against `libodbc.so` by default. This is typically provided by [unix-odbc](http://www.unixodbc.org/). Using the `--feature iodbc` you can also link against `libiodbc.so`. This may be interesting if you are trying to connect to some older data sources on macOS.

## Installing `unix-odbc`

### Linux

Use your systems packet manager to install `unixodbc-dev`. E.g. on Ubuntu / Debian

```shell
sudo apt install unixodbc-dev
```

### macOS

On Intel based architectures you can install `unix-odbc` using homebrew.

```shell
brew install unixodbc
```

Note for **ARM** based macOS Systems (M1 processors and later):

`cargo build` is not going to pick up `libodbc.so` installed via homebrew due to the fact that homebrew on ARM Mac installs into `/opt/homebrew/Cellar` as opposed to `/usr/local/opt/`.

You find documentation on what directories are searched during build here: <https://doc.rust-lang.org/cargo/reference/environment-variables.html#dynamic-library-paths>.

### Install unixODBC from source

You can also install unixODBC from source:

1. Download the unixODBC source code using [`vendor-unix-odbc-source.sh`](unix-odbc/vendor-unix-odbc-source.sh)
2. `./configure`
3. `make`
4. `make install`

Thanks to @TBPixel for testing this!

### Build unixODBC together with your code

Activate the `vendored-unix-odbc` feature to build unixODBC from source automatically when you compile your project.
This removes the need to install anything other than a C compiler on your system.

### Windows

As windows does ship with ODBC preinstalled, you are good to go out of the box.

## Current State

Symbols are added to this library as we go along implementing uses cases in higher level APIs. If you miss something please do not hesitate to contribute.

## Documentation

Thanks to the folks of [docs.rs] for building and hosting the [documentation]!

## Contributing

Want to help out? Just create an issue or pull request.

[docs.rs]: https://docs.rs
[documentation]: https://docs.rs/odbc-sys/
