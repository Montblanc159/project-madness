#!/bin/sh

sh ./install-dependencies

cargo build
cargo doc --document-private-items
