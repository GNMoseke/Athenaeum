#!/bin/sh
cargo build --profile release
cp target/release/ath ~/.local/bin/
