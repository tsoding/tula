#!/bin/sh

set -xe

cargo build

for row in $(cat tests.list); do
    file=$(echo $row | cut -d, -f1)
    kind=$(echo $row | cut -d, -f2)
    case $kind in
        "run")
            expect="$file.expect"
            ./target/debug/tula run $file > $expect 2>&1 || true
            ;;
        "expand")
            expect="$file.expect.expand"
            ./target/debug/tula expand $file > $expect
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
done
