#!/bin/bash

if [ "${ODBC_SYS_STATIC_PATH:-}" != "" ]; then
    if [ "$TRAVIS_OS_NAME" == "osx" ]; then 
        if otool -L target/debug/examples/static_test | grep -q odbc; then
            echo Found non-static odbc ref!
            otool -L target/debug/examples/static_test
            exit 1
        fi
    else
        if ldd target/debug/examples/static_test | grep -q odbc; then
            echo Found non-static odbc ref!
            ldd target/debug/examples/static_test
            exit 1
        fi
    fi
fi