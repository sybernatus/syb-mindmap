#!/bin/bash
set -e

repoDir="$( git rev-parse --show-toplevel )"

trap 'echo "Error occurred at line $LINENO"' ERR

mkdir --parents "${repoDir:?}/.bin"

# >>>> INPUT
pandoc_version="${1:-"3.6.4"}"
# <<<< INPUT

# >>>> SCRIPT
curl --location "https://github.com/jgm/pandoc/releases/download/${pandoc_version}/pandoc-${pandoc_version}-linux-amd64.tar.gz" | tar -xz -C "${repoDir:?}/.bin"
mv "${repoDir:?}/.bin/pandoc-${pandoc_version}/bin/pandoc" "${repoDir:?}/.bin/pandoc"
rm --recursive --force "${repoDir:?}/.bin/pandoc-${pandoc_version}"
# <<<< SCRIPT
