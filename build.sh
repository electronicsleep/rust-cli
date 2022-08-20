#!/bin/bash
set -ex
cargo fmt
cargo build
./target/debug/rust-cli --help
