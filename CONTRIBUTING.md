# Contributing

## Prerequisites

- [moonrepo](https://moonrepo.dev) (`moon`)
- Rust stable toolchain + `wasm32-unknown-unknown` target
- `dx` (Dioxus CLI 0.7.x)
- Node.js + Yarn (VSCode extension)
- JDK 17 (IntelliJ plugin)

```bash
moon run :setup
```

## Branches

| Branch | Role |
|---|---|
| `develop` | Active development — default target for PRs |
| `main` | Production — only updated via `moon run :deploy` |

## Commit Messages

Conventional commit format: `<type>(<scope>): <description>`

**Types**: `feat`, `fix`, `refactor`, `docs`, `chore`, `test`, `style`

**Scopes**: `mindy-engine`, `mindy-html`, `vscode-extension`, `idea-plugin`, `ui`, `root`

## Development

```bash
moon run mindy-html:serve                            # Dev server (http://localhost:8080, hot reload)
moon run "vscode-extension/syb-mindmap:serve.debug" # Build + install extension + open VSCode
```

## Tests

```bash
moon run mindy-engine:test
moon run mindy-html:test
cargo test
```

## Release Workflow

### 1. Release (`moon run :release`)

Run from `develop` once all features are merged.

What it does:
1. `git pull`
2. Computes the next semver version from conventional commits (`git cliff --bump`)
3. Generates changelogs: `.github/CHANGELOG.md`, `vscode-extension/syb-mindmap/CHANGELOG.md`, `idea-plugin/CHANGELOG.md`
4. Bumps the version in all manifests:
   - `vscode-extension/syb-mindmap/package.json` (via `yarn version`)
   - `idea-plugin/build.gradle.kts` (via `sed`)
   - `Cargo.toml` workspace (via `cargo set-version`)
5. Commits everything as `chore: release tag 'x.y.z'`
6. Creates a git tag `x.y.z`
7. Pushes commits and tags

```bash
moon run :release
```

> Version bump strategy: `feat` → minor, `fix` → patch, `feat!` / `BREAKING CHANGE` → major.

### 2. Deploy (`moon run :deploy`)

Promotes the latest release tag to `main`.

What it does:
1. Reads the latest tag (`git describe --tags --abbrev=0`)
2. `git checkout main`
3. `git reset --hard <tag>`
4. `git push --force-with-lease`

```bash
moon run :deploy
```

> Always run `release` before `deploy`.

### 3. Publish

#### VSCode Marketplace

Depends on `build` (WASM release + `.vsix` package).

```bash
moon run "vscode-extension/syb-mindmap:publish"
```

Required: a VSCode Marketplace Personal Access Token, configured via `vsce`
(stored in the system keychain or exported as `VSCE_PAT`).

#### JetBrains Marketplace

Depends on `build` (`./gradlew buildPlugin`).

```bash
moon run "idea-plugin:publish"
```

Required environment variables:

| Variable | Description |
|---|---|
| `CERTIFICATE_CHAIN` | Plugin signing certificate chain |
| `PRIVATE_KEY` | Private key for signing |
| `PRIVATE_KEY_PASSWORD` | Password for the private key |
| `IDEA_PUBLISH_TOKEN` | JetBrains Marketplace token |

### Full Release Sequence

```bash
# 1. Merge all features into develop, then:
moon run :release

# 2. Promote to main
moon run :deploy

# 3. Publish extensions (env vars required, see above)
moon run "vscode-extension/syb-mindmap:publish"
moon run "idea-plugin:publish"
```
