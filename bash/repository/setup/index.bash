#!/bin/bash
set -e

repoDir="$( git rev-parse --show-toplevel )"

trap 'echo "Error occurred at line $LINENO"' ERR

mkdir --parents "${repoDir:?}/.bin"

cargo_edit_version="0.13.2"
pandoc_version="3.6.4"
proto_version="0.49.1"
# >>>> SCRIPT

# Install cargo-edit
cargo install cargo-edit --version "${cargo_edit_version}" || true

# Install inkscape
sudo apt install inkscape || true

## Install moon
#curl --location "https://github.com/moonrepo/moon/releases/download/v${moon_version}/moon-x86_64-unknown-linux-gnu" --output "${repoDir:?}/.bin/moon"
#chmod +x "${repoDir:?}/.bin/moon"

# Install proto
mkdir --parents "${repoDir:?}/.bin/proto-tmp"
curl --location "https://github.com/moonrepo/proto/releases/download/v${proto_version}/proto_cli-x86_64-unknown-linux-gnu.tar.xz" | tar -xJf - --directory "${repoDir:?}/.bin/proto-tmp" --strip-components=1
mv "${repoDir:?}/.bin/proto-tmp/proto" "${repoDir:?}/.bin/proto-tmp/proto-shim" "${repoDir:?}/.bin/"
rm --recursive --force "${repoDir:?}/.bin/proto-tmp"
chmod +x "${repoDir:?}/.bin/proto" "${repoDir:?}/.bin/proto-shim"

"${repoDir:?}/.bin/proto" install

# Install pandoc
curl --location "https://github.com/jgm/pandoc/releases/download/${pandoc_version}/pandoc-${pandoc_version}-linux-amd64.tar.gz" | tar -xz -C "${repoDir:?}/.bin"
mv "${repoDir:?}/.bin/pandoc-${pandoc_version}/bin/pandoc" "${repoDir:?}/.bin/pandoc"
rm --recursive --force "${repoDir:?}/.bin/pandoc-${pandoc_version}"
# <<<< SCRIPT1