Changelog
=========

0.8.2
-----

* Adds support for static linking

0.8.1
-----

* Fix: Move `SQL_ATTR_ASYNC_STMT_EVENT` is now part of the `SqlStatementAttribute` enumeration.

0.8.0
-----

* Adds `InfoType::SQL_ASYNC_MODE`
* Adds `InfoType::SQL_MAX_ASYNC_CONCURRENT_STATEMENTS`
* Adds `InfoType::SQL_ASYNC_DBC_FUNCTIONS`,
* Adds `InfoType::SQL_DRIVER_AWARE_POOLING_SUPPORTED`,
* Adds `InfoType::SQL_ASYNC_NOTIFICATION`
* Adds `SqlConnectionAttribute::SQL_ATTR_ASYNC_STMT_EVENT`
* Adds `SqlConnectionAttribute::SQL_ATTR_ASYNC_DBC_EVENT`

0.7.0
-----

* Adds `SqlDataType::SQL_EXT_TIME_OR_TIME_INTERVAL`
* Adds `SqlDataType::SQL_EXT_TIMESTAMP`
* Adds `SqlAttributesStringLength`
* Adds `SqlConnectionAttribute::SQL_ATTR_ASYNC_ENABLE`
* Adds `SqlGetTypeInfo`

0.6.3
-----

* Fix missing `SQLConnect`

0.6.2
-----

* yanked due to missing `SQLConnect`
* add `SqlRowCount`

0.6.1
-----

* Fix: `InputOutput` is now representend as `i16` instead of `u16` in C code.
