# Publishing to crates.io

Weft Core publishes two crates from this repository:

| Crate | Description |
|-------|-------------|
| [`weft-package-sdk`](https://crates.io/crates/weft-package-sdk) | SDK for authoring WASM capability packages |
| [`weft-core`](https://crates.io/crates/weft-core) | Runtime library and `weft-core` binary |

## Automated publish (recommended)

1. Create a [crates.io API token](https://crates.io/settings/tokens) with `publish-new` + `publish-update`.
2. Add it to this repo as **`CARGO_REGISTRY_TOKEN`**:

   ```bash
   gh secret set CARGO_REGISTRY_TOKEN --repo ailiheizi/weft-core
   ```

3. Tag a release:

   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

   The [`publish-crates.yml`](../.github/workflows/publish-crates.yml) workflow publishes **`weft-package-sdk` first**, then **`weft-core`**.

## Manual publish

From the repository root:

```bash
cargo publish -p weft-package-sdk
cargo publish -p weft-core
```

You must publish `weft-package-sdk` before `weft-core` because the latter depends on the former on crates.io.

## First-time crate claim

The first publish of each crate name claims it on crates.io under your account. Ensure you are logged in:

```bash
cargo login
```

## Versioning

Follow [`docs/SEMVER.md`](SEMVER.md). Pre-1.0 (`0.x.y`) allows breaking API changes with minor bumps.
