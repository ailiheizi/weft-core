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

## 一键配置 crates.io 发布

1. 打开 https://crates.io/settings/tokens 创建 API Token
2. 设置 GitHub Secret：

```powershell
gh secret set CARGO_REGISTRY_TOKEN --repo ailiheizi/weft-core
# 粘贴 token 后回车
```

3. 重新触发发布：

```powershell
gh workflow run publish-crates.yml --repo ailiheizi/weft-core
# 或打新 tag: git tag v0.1.1 && git push origin v0.1.1
```


## Versioning

Follow [`docs/SEMVER.md`](SEMVER.md). Pre-1.0 (`0.x.y`) allows breaking API changes with minor bumps.
