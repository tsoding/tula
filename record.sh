#!/bin/sh

set -xe

for file in $(cat tests.list); do
    cargo run -q run $file > "$file.expect";
done
