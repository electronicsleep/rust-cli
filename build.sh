#!/bin/bash
set -ex
cargo fmt
cargo build
./target/debug/rust-cli --help
./target/debug/rust-cli -v echo test
./target/debug/rust-cli -e dev echo test
./target/debug/rust-cli tui test
echo "ok"
