# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3](https://github.com/pacman82/odbc-sys/compare/unix-odbc-v0.1.2...unix-odbc-v0.1.3) - 2025-10-25

### Fixed

- remove SQL_WCHART_CONVERT to fix ABI mismatch with system drivers

### Other

- simplify version extraction in build script
- remove some unused config varuiables
- simplify build script
- remove dependency to autotools

## [0.1.2](https://github.com/pacman82/odbc-sys/compare/unix-odbc-v0.1.1...unix-odbc-v0.1.2) - 2025-10-12

### Added

- unix-odbc crate, just does nothing than used with windows
- introduce compile time feature static_ltdl to allow for explicit

### Other

- Prevent recreating configure script for unixODBC during build
- formattig
- Remove superfluous unixODBC tar.gz
- build from unixODBC source included verbatum
- Vendor source for unixODBC verbatum
- update vendor script to include source verbatum
- formattig

## [0.1.1](https://github.com/pacman82/odbc-sys/compare/unix-odbc-v0.1.0...unix-odbc-v0.1.1) - 2025-10-09

### Fixed

- An issue has been fixed which prevent the build on mac-os

### Other

- release v0.27.1

## [0.1.0](https://github.com/pacman82/odbc-sys/releases/tag/unix-odbc-v0.1.0) - 2025-10-08

### Added

- Introduce vendored-unix-odbc flag. This enables building and linking against unixODBC as part of cargo build
