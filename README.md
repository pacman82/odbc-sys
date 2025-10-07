# odbc-sys

[![Docs](https://docs.rs/odbc-sys/badge.svg)](https://docs.rs/odbc-sys/)
[![MIT licensed](https://img.shields.io/github/license/mashape/apistatus.svg)](https://github.com/pacman82/odbc-sys/blob/main/LICENSE)

FFI (Foreign Function Interface) bindings for ODBC (Open Database Connectivity).

ODBC is a standard C API for accessing databases. Applications call ODBC functions, which are handled by a **driver manager** (the intermediary layer), which then routes calls to database-specific **drivers**.
On Unix-like systems, you choose between driver managers: [unixODBC](http://www.unixodbc.org/) (most common on Linux) or [iODBC](http://www.iodbc.org/) (common on older macOS, BSD). Windows includes a built-in driver manager. This workspace provides separate crates for each driver manager to match their respective licenses while sharing common ODBC declarations.

This repository contains a workspace with multiple crates to support different driver managers.

## Crates

### [`odbc-sys`](./odbc-sys)

[![Crates.io version](https://img.shields.io/crates/v/odbc-sys)](https://crates.io/crates/odbc-sys)

**Recommended**: A unified crate that re-exports either `unixodbc-sys`, `iodbc-sys`, or `odbc-sys-core` based on feature flags.
This is the easiest way to use ODBC bindings in your project while choosing the driver manager implementation.

#### Features
- `unixodbc-sys`: Statically link to unixODBC
- `iodbc-sys` - Statically link to iODBC
- `unixodbc` - Dynamically link to unixODBC
- `iodbc` - Dynamically link to iODBC

On windows, dynamic linking to the system's ODBC driver is always activated.

#### Usage with static linking

Install a C compiler, then:

```toml
[dependencies]
odbc-sys = { version = "0.1", features = ["unixodbc-sys"] }
```

#### With dynamic linking


```bash
sudo apt install unixodbc-dev
```

```toml
[dependencies]
odbc-sys = { version = "0.1", features = ["unixodbc-sys"] }
```

### [`unixodbc-sys`](./unixodbc-sys)

[![Crates.io version](https://img.shields.io/crates/v/unixodbc-sys)](https://crates.io/crates/unixodbc-sys)

FFI bindings for ODBC. Licensed under LGPL-2.1-or-later to match unixODBC's license.
When using the `static` feature, the LGPL requires you to distribute your work
in a manner that allows the final user to re-compile the application to a different version of unixODBC:
https://www.gnu.org/licenses/gpl-faq.html#LGPLStaticVsDynamic

#### Links to
- UNIX: `libodbc` (typically [unixODBC](http://www.unixodbc.org/))
  - Needs installing when the `static` crate feature is not enabled:
    - Debian/Ubuntu: `sudo apt install unixodbc-dev`
    - Fedora: `sudo dnf install unixODBC-devel`
    - macOS: `brew install unixodbc`
- Windows: `odbc32.dll` (built-in, nothing to install)

#### Usage

```toml
[dependencies]
unixodbc-sys = "0.1"
```

### [`iodbc-sys`](./iodbc-sys)

[![Crates.io version](https://img.shields.io/crates/v/iodbc-sys)](https://crates.io/crates/iodbc-sys)

FFI bindings for ODBC. Licensed under MIT (to match iODBC's license).

#### Links to
- UNIX: `libiodbc` (iODBC - useful for older data sources)
  - Needs installing when the `static` crate feature is not enabled:
    - Debian/Ubuntu: `sudo apt install libiodbc2-dev`
    - macOS: `brew install libiodbc`
- Windows: `odbc32.dll` (built-in, nothing to install)

#### Usage

```toml
[dependencies]
iodbc-sys = "0.1"
```

## Design Goals

- Provide ODBC symbol declarations compatible with the C-Interface
- Support Unix and Windows in 32-bit and 64-bit
- Don't abstract away the underlying API
- Increase type safety where feasible
- Omit deprecated ODBC 2.0 symbols (e.g., `SQLAllocEnv`)

## Static Linking

Both crates support a `static` feature to compile from source, eliminating the need to install system packages.
Only the driver manager is statically linked; you still need to install individual database drivers on your system.

```toml
[dependencies]
unixodbc-sys = { version = "0.1", features = ["static"] }
```

#### Requirements
- C compiler at build time
- Git submodules initialized: `git submodule update --init --recursive`
- **No static linking on Windows**. Since windows ships with a built-in ODBC driver manager, it does not make sense to embed a separate driver manager in your binary.

#### Use pre-built static library

Set `ODBC_SYS_STATIC_PATH` to point to a directory containing the static library:

```bash
export ODBC_SYS_STATIC_PATH=/usr/local/lib
cargo build -p unixodbc-sys --features static
```

## Version Features

Both crates support ODBC version features:

- `odbc_version_3_50` - ODBC 3.5 symbols
- `odbc_version_3_80` - ODBC 3.8 symbols (default)
- `odbc_version_4` - ODBC 4.0 symbols (experimental)

## Platform Support

| Platform | unixodbc-sys | iodbc-sys | Static Linking |
|----------|--------------|-----------|----------------|
| Linux    | ✅ Primary   | ✅ Supported | ✅ |
| macOS    | ✅ Supported | ✅ Primary   | ✅ |
| BSD      | ✅ Supported | ✅ Supported | ✅ |
| Windows  | ✅ | ✅ | ❌ (uses built-in `odbc32.dll`) |

On Windows, both crates link to the same system library - choose based on license requirements or cross-platform consistency.

## Architecture

```
odbc-sys/
├── odbc-sys-core/      # Shared types/declarations (internal, not published)
├── unixodbc-sys/       # Re-exports core, links to libodbc/odbc32.dll - LGPL
├── iodbc-sys/          # Re-exports core, links to libiodbc/odbc32.dll - MIT
└── odbc-sys/           # Unified re-export with feature flags - MIT OR LGPL
```

This avoids code duplication while enabling separate licensing and publishing. The `odbc-sys` crate provides a convenient unified interface.

## Development

### Testing

You typically only have one ODBC driver manager installed at a time on UNIX. Test the installed one:

```bash
# With unixODBC (most Linux):
cargo test -p unixodbc-sys -p odbc-sys-core

# With iODBC (some macOS):
cargo test -p iodbc-sys -p odbc-sys-core

# Windows (both work):
cargo test --workspace
```

Running `cargo test --workspace` on UNIX will fail for the crate whose library isn't installed - this is expected.

## Contributing

Create an issue or pull request!

## Documentation

- [odbc-sys docs](https://docs.rs/odbc-sys/) - Recommended unified interface
- [unixodbc-sys docs](https://docs.rs/unixodbc-sys/)
- [iodbc-sys docs](https://docs.rs/iodbc-sys/)
