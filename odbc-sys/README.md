# odbc-sys

[![Crates.io version](https://img.shields.io/crates/v/odbc-sys)](https://crates.io/crates/odbc-sys)
[![Docs](https://docs.rs/odbc-sys/badge.svg)](https://docs.rs/odbc-sys/)

FFI (Foreign Function Interface) bindings for ODBC (Open Database Connectivity).

This crate provides a unified interface for ODBC across platforms, allowing you to choose between different driver manager implementations via feature flags.

## Features

- `unixodbc-sys`: Statically link to the unixODBC driver manager
- `iodbc-sys`: Statically link to the iodbc driver manager

Only one of the two can be enabled at a time.

By default, this crate dynamically links to libodbc on unix and against the built-in ODBC driver manager on windows.

## Usage

```toml
[dependencies]
odbc-sys = "0.1"
```

To automatically compile iodbc and statically link it:

```toml
[dependencies]
odbc-sys = { version = "0.1", features = ["iodbc-sys"] }
```

For unixodbc

```toml
[dependencies]
odbc-sys = { version = "0.1", features = ["unixodbc-sys"] }
```
