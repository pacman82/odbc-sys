odbc-sys
========

[![Build Status](https://travis-ci.org/pacman82/odbc-sys.svg?branch=master)](https://travis-ci.org/pacman82/odbc-sys)
[![Docs](https://docs.rs/odbc-sys/badge.svg)](https://docs.rs/odbc-sys/)
[![MIT licensed](https://img.shields.io/github/license/mashape/apistatus.svg)](https://github.com/pacman82/odbc-sys/blob/master/LICENSE)
[![Published](http://meritbadge.herokuapp.com/odbc-sys)](https://crates.io/crates/odbc-sys)

FFI (Foreign Function Interface) bindings for ODBC (Open Database Connectivity)
As ffi bindings to C-APIs are low level by nature this library is intended to be the foundation of
other libraries to build on top, rather than to be used directly.

Design Goals
------------

* Providing declarations of ODBC Symbols compatible to the C-Interface of an ODBC Driver Manager
* Provide correct definition of symbols for Unix and Windows in either 32Bit or 64Bit flavour
* Not to abstract away any power of the underlying API
* Increase type safety where feasable
* As it is as of now unlikely to happen that anyone is writing ODBC 2.0 applications in Rust
  therefore deprecated symbols like 'SQLAllocEnv' have been left out intentionally.

Current State
-------------

Symbols are added to this library as we go along implementing uses cases in higher level APIs. If
you miss something please do not hesitate to contribute.

Documentation
-------------

Thanks to the folks of [docs.rs] for building and hosting the [documentation]!

Contributing
------------

Want to help out? Just create an issue, pull request or contact markus.klein@live.de.

[docs.rs]: https://docs.rs
[documentation]: https://docs.rs/odbc-sys/
