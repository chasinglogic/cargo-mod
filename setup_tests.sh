#!/bin/bash

if [ -d tests/generator_test/ ]; then
    rm -rf tests/generator_test/
fi

cd tests
cargo new generator_test
