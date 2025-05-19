#!/bin/bash
set -e

repoDir="$( git rev-parse --show-toplevel )"

trap 'echo "Error occurred at line $LINENO"' ERR

mkdir --parents "${repoDir:?}/.bin"

# >>>> INPUT
cargo_edit_version="${1:-"0.13.2"}"
cargo_binstall_version="${2:-"1.12.3"}"
cargo_dioxus_cli_version="${3:-"0.6.3"}"
# <<<< INPUT

# >>>> SCRIPT
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --no-modify-path
cargo install cargo-edit --version "${cargo_edit_version}"
cargo install cargo-binstall --version "${cargo_binstall_version}"
cargo install dioxus-cli --version "${cargo_dioxus_cli_version}"
# <<<< SCRIPT
