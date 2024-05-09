#!/bin/sh

set -xe

for file in $(cat tests.list); do
    actual=$(mktemp)
    cargo run -q run $file > "$actual"
    diff -u "$file.expect" "$actual"
    rm -v "$actual"
done
