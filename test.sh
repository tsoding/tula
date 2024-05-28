#!/bin/sh

set -xe

cargo build

for row in $(cat tests.list); do
    file=$(echo $row | cut -d, -f1)
    kind=$(echo $row | cut -d, -f2)
    actual=$(mktemp)
    case $kind in
        "run")
            ./target/debug/tula run $file > "$actual" 2>&1 || true
            diff -u "$file.expect" "$actual"
            ;;
        "expand")
            ./target/debug/tula expand $file > "$actual" 2>&1 || true
            diff -u "$file.expect.expand" "$actual"
            ;;
        "ignore")
            echo "$file is explicitly ignored"
            continue
            ;;
        *)
            echo "WARNING: unknown kind of test $kind. $file is ignored."
            continue
            ;;
    esac
    rm -v "$actual"
done
