#!/bin/bash
set -e

trap 'echo "Error occurred at line $LINENO"' ERR

git cliff --bump --config .cliff.toml > .github/CHANGELOG.md
git cliff --unreleased --bump --config .cliff.toml > vscode-extension/syb-mindmap/CHANGELOG.md
git cliff --unreleased --bump --config .cliff.toml > idea-plugin/CHANGELOG.md
git add .github/CHANGELOG.md

version="$( git cliff --config .cliff.toml --bumped-version 2> /dev/null )"

(
    cd vscode-extension/syb-mindmap
    yarn version --no-git-tag-version --no-commit-hooks --non-interactive --new-version "${version}"
    git add package.json
)

(
    cd idea-plugin
    sed --in-place --regexp-extended "s#^version = .*#version = \"${version}\"#" build.gradle.kts
    git add build.gradle.kts
)
cargo set-version --workspace "${version}" --offline

git add --all
git commit --message "chore: release tag '${version}'"

git tag "${version}"
git push --follow-tags