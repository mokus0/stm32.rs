#!/bin/sh

set -e
cd "$(dirname "$0")"/..

cd examples
for dir in */; do
    echo "==== building example: $dir ===="
    (cd $dir && ./build.sh)
done

