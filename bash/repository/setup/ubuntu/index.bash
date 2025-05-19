#!/bin/bash
set -e

repoDir="$( git rev-parse --show-toplevel )"

trap 'echo "Error occurred at line $LINENO"' ERR

mkdir --parents "${repoDir:?}/.bin"

# >>>> INPUT
extraTools="${1:-"inkscape"}"
# <<<< INPUT

# >>>> SCRIPT
sudo apt get intall --yes ${extraTools}
# <<<< SCRIPT
