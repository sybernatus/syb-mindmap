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
| `mindy-engine` | Parser + data model (YAML/JSON → tree) | serde, serde_yaml, serde_json, tracing |
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

## Common Commands

### Develop

```bash
moon run mindy-html:serve        # Dev server (hot reload)
moon run mindy-html:build.debug  # Debug WASM build
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
moon run :release                # Tag and release
moon run :deploy                 # Promote to main
```

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
