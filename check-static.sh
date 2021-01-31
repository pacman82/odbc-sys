#!/bin/bash

if [ "${ODBC_SYS_STATIC_PATH:-}" != "" ]; then
    if [ "$TRAVIS_OS_NAME" == "osx" ]; then 
        otool -L target/debug/examples/static_test
        if otool -L target/debug/examples/static_test | grep -q odbc; then
            echo Found non-static odbc ref!
            exit 1
        fi
    else
        ldd target/debug/examples/static_test
        if ldd target/debug/examples/static_test | grep -q odbc; then
            echo Found non-static odbc ref!
            exit 1
        fi
    fi
else
    echo Non-static build
fi