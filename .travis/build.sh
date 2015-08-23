#!/bin/sh

set -e
cd "$(dirname "$0")"/..

cd examples
for dir in */; do
    (cd $dir && ./build.sh)
done

