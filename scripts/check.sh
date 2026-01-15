#!/bin/sh

cargo fmt --check
cargo check
cargo clippy
sh ./scripts/check-dialogs.sh
