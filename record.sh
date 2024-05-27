#!/bin/sh

set -xe

for row in $(cat tests.list); do
    file=$(echo $row | cut -d, -f1)
    kind=$(echo $row | cut -d, -f2)
    case $kind in
        "run")
            expect="$file.expect"
            cargo run -q run $file > $expect
            ;;
        "expand")
            expect="$file.expect.expand"
            cargo run -q expand $file > $expect
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
