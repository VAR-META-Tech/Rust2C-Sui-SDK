#!/bin/bash
set -e

export RUST_BACKTRACE=1
cargo build --release
cbindgen --config cbindgen.toml --crate sui_rust_sdk --output ./header/sui_lib.h
gcc test.c -L target/release/ -lsui_rust_sdk -o Test
./test
