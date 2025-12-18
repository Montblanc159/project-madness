#!/bin/sh

sh ./install-dependencies

cargo build
cargo doc
