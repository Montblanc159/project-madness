#!/bin/sh

sudo apt update
sudo apt upgrade

# Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
