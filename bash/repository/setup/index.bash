#!/bin/bash
set -e

repoDir="$( git rev-parse --show-toplevel )"

trap 'echo "Error occurred at line $LINENO"' ERR

mkdir --parents "${repoDir:?}/.bin"

cargo install cargo-make --version 0.37.24 || true
cargo install cargo-edit --version 0.13.2 || true
sudo apt install inkscape || true
curl --location "https://github.com/jgm/pandoc/releases/download/3.6.4/pandoc-3.6.4-linux-amd64.tar.gz" | tar -xz -C "${repoDir:?}/.bin"
mv "${repoDir:?}/.bin/pandoc-3.6.4/bin/pandoc" "${repoDir:?}/.bin/pandoc"
rm --recursive --force "${repoDir:?}/.bin/pandoc-3.6.4"
