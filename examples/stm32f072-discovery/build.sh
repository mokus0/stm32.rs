#!/bin/sh

set -e
cd "$(dirname "$0")"

cargo build --target=thumbv6-none-eabi -j4 --release \
    && arm-none-eabi-size -Ax target/*/release/*discovery
