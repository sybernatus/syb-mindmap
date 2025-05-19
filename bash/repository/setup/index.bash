#!/bin/bash
set -e

repoDir="$( git rev-parse --show-toplevel )"
hereDir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

trap 'echo "Error occurred at line $LINENO"' ERR

mkdir --parents "${repoDir:?}/.bin"

cargo_edit_version="0.13.2"
cargo_binstall_version="1.12.3"
cargo_dioxus_cli_version="0.6.3"
pandoc_version="3.6.4"
proto_version="0.49.1"
# >>>> SCRIPT

# Install ubuntu dependencies
"${hereDir:?}/ubuntu/index.bash" "inkscape"

# Install rust
"${hereDir:?}/rust/index.bash" "${cargo_edit_version}" "${cargo_binstall_version}" "${cargo_dioxus_cli_version}"

# Install proto
"${hereDir:?}/proto/index.bash" "${proto_version}"
"proto" install

# Install pandoc
"${hereDir:?}/pandoc/index.bash" "${pandoc_version}"
# <<<< SCRIPT
