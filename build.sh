#!/bin/sh

set -xe

mkdir -p build/

rustc -o build/tula src/tula.rs
