# eventky-app-specs

Rust + WASM data model package for Eventky.

## Release automation

This repo now includes GitHub Actions workflows for CI and publishing:

- `.github/workflows/release.yml`
  - Runs CI on push/PR.
  - On published GitHub release (or manual dispatch with tag), builds WASM and publishes npm package.
- `.github/workflows/publish.yml`
  - Triggered by tag push (`v*`).
  - Publishes to crates.io and npm (idempotent), then creates GitHub release notes.

## Required secrets

Add these in GitHub repo settings (`Settings -> Secrets and variables -> Actions`):

- `CARGO_REGISTRY_TOKEN` - crates.io publish token.
- `NPM_TOKEN` - npm token with publish rights for `@eventky/pubky-app-specs`.

## Versioning rules

- `Cargo.toml` version must match the pushed tag without the leading `v`.
  - Example: tag `v0.1.1` requires `version = "0.1.1"`.
- Pre-release tags (`-alpha`, `-beta`, `-rc`) publish npm under non-latest dist-tag.

## Typical release flow

1. Update `Cargo.toml` version.
2. Run local checks:
   - `cargo test`
   - `wasm-pack build --target bundler --release`
3. Commit and push.
4. Create and push tag:
   - `git tag vX.Y.Z`
   - `git push origin vX.Y.Z`
5. Confirm `Publish` workflow succeeds.

## npm package name

WASM output package metadata is normalized in CI to publish as:

- `@eventky/pubky-app-specs`

This keeps Eventky app dependency coordinates stable.
