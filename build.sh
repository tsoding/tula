#!/bin/sh

set -xe

mkdir -p build/

rustc -o build/tula src/tula.rs
rustc -o build/test_lexer src/test_lexer.rs
rustc -o build/test_sexpr src/test_sexpr.rs
