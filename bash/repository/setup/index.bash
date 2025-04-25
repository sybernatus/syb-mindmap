#!/bin/bash
set -e

trap 'echo "Error occurred at line $LINENO"' ERR


cargo install cargo-make --version 0.37.24 || true
cargo install cargo-edit --version 0.13.2 || true
sudo apt install inkscape || true