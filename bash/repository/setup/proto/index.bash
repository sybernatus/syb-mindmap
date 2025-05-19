#!/bin/bash
set -e

repoDir="$( git rev-parse --show-toplevel )"

trap 'echo "Error occurred at line $LINENO"' ERR

mkdir --parents "${repoDir:?}/.bin"

# >>>> INPUT
proto_version="${1:-"0.49.1"}"
# <<<< INPUT

# >>>> SCRIPT
bash <(curl -fsSL https://moonrepo.dev/install/proto.sh) "${proto_version}" --yes
# <<<< SCRIPT
