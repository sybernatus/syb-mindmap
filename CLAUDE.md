# CLAUDE.md

## Repository Overview

Rust workspace generating mindmap diagrams from YAML/JSON. Two crates (`mindy-engine`, `mindy-html`) compiled to WebAssembly via Dioxus, distributed as a VSCode extension and an IntelliJ IDEA plugin.

## Architecture

### Structure

```
mindy-engine/              # Core Rust library (parser, data model)
mindy-html/                # Dioxus web app (WASM frontend)
  src/                     # Dioxus components
  assets/                  # Copied from root assets/ at build time
vscode-extension/          # VSCode plugin
idea-plugin/               # IntelliJ IDEA plugin
assets/                    # Shared assets (logo, schemas, ui, ext)
bash/repository/           # Repository scripts (setup, release)
tools/images/              # Tooling images
Cargo.toml                 # Workspace root (members: mindy-html, mindy-engine)
moon.yml                   # Moonrepo root (tasks: setup, release, deploy)
```

### Crates

| Crate | Description | Key deps |
|---|---|---|
| `mindy-engine` | Parser + data model (YAML/JSON → tree) | serde, serde_yml, serde_json, tracing |
| `mindy-html` | Dioxus WASM frontend | dioxus 0.7 (web), mindy-engine, web-sys |

### Moonrepo Tasks

| Project | Task | Command |
|---|---|---|
| root | `setup` | `bash/repository/setup/index.bash` |
| root | `release` | `bash/repository/release/index.bash` |
| root | `deploy` | checkout main, reset to latest tag, force-push |
| mindy-engine | `build` | `cargo build` |
| mindy-engine | `test` | `cargo test` |
| mindy-html | `build.assets` | copy assets from root `assets/` |
| mindy-html | `build.debug` | `dx build --platform web` |
| mindy-html | `build` | `dx bundle --release --platform web --target release` |
| mindy-html | `serve` | `dx serve --platform web` |
| mindy-html | `test` | `cargo test` |
| vscode-extension/syb-mindmap | `build.debug` | build WASM debug + package `.vsix` |
| vscode-extension/syb-mindmap | `build` | build WASM release + package `.vsix` |
| vscode-extension/syb-mindmap | `serve.debug` | build.debug + install extension + open VSCode |
| vscode-extension/syb-mindmap | `serve` | build + install extension + open VSCode |
| vscode-extension/syb-mindmap | `publish` | `vsce publish` (requires `build`) |
| idea-plugin | `build` | `./gradlew buildPlugin` |
| idea-plugin | `publish` | `./gradlew publishPlugin` (requires `build`) |

## Common Commands

### Develop

```bash
moon run mindy-html:serve        # Dev server navigateur (hot reload, http://localhost:8080)
moon run mindy-html:build.debug  # Debug WASM build
```

### Tester dans VSCode

```bash
# Build debug + installe l'extension + ouvre VSCode
moon run "vscode-extension/syb-mindmap:serve.debug"

# Puis dans VSCode : Ctrl+Shift+P → "Syb Mindmap"
```

### Build

```bash
moon run mindy-html:build        # Release WASM bundle
moon run mindy-engine:build      # Build engine only
```

### Test

```bash
moon run mindy-engine:test
moon run mindy-html:test
cargo test                       # All workspace tests
```

### Release

```bash
moon run :release                # Bump version, generate changelog, tag and push
moon run :deploy                 # Promote latest tag to main (force-push)
```

### Publish

```bash
moon run "vscode-extension/syb-mindmap:publish"   # Publish .vsix to VSCode Marketplace
moon run "idea-plugin:publish"                     # Publish to JetBrains Marketplace
```

## Release & Deploy Workflow

### Release (`moon run :release`)

Exécuté depuis `develop`. Effectue dans l'ordre :

1. `git pull` — synchronise la branche locale
2. `git cliff --bump` — calcule la prochaine version (semver) depuis les commits conventionnels et génère les changelogs :
   - `.github/CHANGELOG.md`
   - `vscode-extension/syb-mindmap/CHANGELOG.md`
   - `idea-plugin/CHANGELOG.md`
3. Bumpe la version dans tous les manifestes :
   - `vscode-extension/syb-mindmap/package.json` via `yarn version`
   - `idea-plugin/build.gradle.kts` via `sed`
   - `Cargo.toml` (workspace) via `cargo set-version`
4. Commit `chore: release tag 'x.y.z'` + tag git `x.y.z`
5. `git push && git push --tags`

```bash
moon run :release
```

### Deploy (`moon run :deploy`)

Promeut la dernière release sur `main`. Effectue dans l'ordre :

1. Récupère le tag le plus récent (`git describe --tags --abbrev=0`)
2. `git checkout main`
3. `git reset --hard <tag>` — aligne `main` sur le tag
4. `git push --force-with-lease` — force-push sécurisé

```bash
moon run :deploy
```

> **Ordre obligatoire** : toujours faire `release` avant `deploy`.

### Publish VSCode (`moon run "vscode-extension/syb-mindmap:publish"`)

Dépend de `build` (build WASM release + package `.vsix`). Lance :

```bash
yarn run vsce publish
```

**Variable d'environnement requise** : le token PAT VSCode Marketplace doit être
configuré via `vsce` (stocké dans le keychain ou via `VSCE_PAT`).

### Publish IntelliJ (`moon run "idea-plugin:publish"`)

Dépend de `build` (`./gradlew buildPlugin`). Lance :

```bash
./gradlew publishPlugin
```

**Variables d'environnement requises** :

| Variable | Usage |
|---|---|
| `CERTIFICATE_CHAIN` | Certificat de signature du plugin |
| `PRIVATE_KEY` | Clé privée pour la signature |
| `PRIVATE_KEY_PASSWORD` | Mot de passe de la clé privée |
| `IDEA_PUBLISH_TOKEN` | Token JetBrains Marketplace |

### Séquence complète d'une release

```bash
# 1. Depuis develop, après merge des features
moon run :release

# 2. Promouvoir sur main
moon run :deploy

# 3. Publier les extensions (nécessite les variables d'env ci-dessus)
moon run "vscode-extension/syb-mindmap:publish"
moon run "idea-plugin:publish"
```

## Release & Deploy Workflow

### Release (`moon run :release`)

Exécuté depuis `develop`. Effectue dans l'ordre :

1. `git pull` — synchronise la branche locale
2. `git cliff --bump` — calcule la prochaine version (semver) depuis les commits conventionnels et génère les changelogs :
   - `.github/CHANGELOG.md`
   - `vscode-extension/syb-mindmap/CHANGELOG.md`
   - `idea-plugin/CHANGELOG.md`
3. Bumpe la version dans tous les manifestes :
   - `vscode-extension/syb-mindmap/package.json` via `yarn version`
   - `idea-plugin/build.gradle.kts` via `sed`
   - `Cargo.toml` (workspace) via `cargo set-version`
4. Commit `chore: release tag 'x.y.z'` + tag git `x.y.z`
5. `git push && git push --tags`

### Deploy (`moon run :deploy`)

Promeut la dernière release sur `main` :

1. Récupère le tag le plus récent (`git describe --tags --abbrev=0`)
2. `git checkout main`
3. `git reset --hard <tag>`
4. `git push --force-with-lease`

> Toujours faire `release` avant `deploy`.

### Publish

```bash
moon run "vscode-extension/syb-mindmap:publish"   # vsce publish → VSCode Marketplace
moon run "idea-plugin:publish"                     # gradlew publishPlugin → JetBrains Marketplace
```

Variables d'env requises pour IntelliJ : `CERTIFICATE_CHAIN`, `PRIVATE_KEY`, `PRIVATE_KEY_PASSWORD`, `IDEA_PUBLISH_TOKEN`.

Séquence complète : `release` → `deploy` → `publish` (VSCode + IntelliJ).

Voir `CONTRIBUTING.md` pour la documentation complète.

## Development Workflow

### Git Branches

- `develop` — active development branch (default)
- `main` — production branch, updated via `deploy` task (reset to latest tag)

### Git Commit Messages

Conventional commit format: `<type>(<scope>): <description>`

Types: `feat`, `fix`, `refactor`, `docs`, `chore`, `test`, `style`

Scopes: `mindy-engine`, `mindy-html`, `vscode-extension`, `idea-plugin`, `ui`, `root`

No AI signatures or co-authorship lines.

## Important Patterns

### Bash Script Headers

All bash scripts must use this exact header:

```bash
#!/usr/bin/env bash

# >>>> HEADER
set -e

repo_dir="$(git rev-parse --show-toplevel)"
here_dir="$(dirname "${0}" | while read -r dir; do cd "${dir}" && pwd && break; done | sed "s|${repo_dir}/||")"
trap "An error occurred in ${here_dir}/index.bash on line \${LINENO}." ERR
# <<<< HEADER
```

### Bash Command Parameters

Always use long-form parameters when they exist:

- `cp --recursive --force` not `cp -rf`
- `rm --recursive --force` not `rm -rf`
- `mkdir --parents` not `mkdir -p`
- `curl --location` not `curl -L`

### Bash Variable Expansion

Always use braced form `${variable}`, never `$variable`.

## Dependencies

moonrepo, Rust toolchain, `dx` (Dioxus CLI), cargo, wasm32 target
