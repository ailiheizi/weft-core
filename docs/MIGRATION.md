# Migrating Weft Core to a Standalone Repository

## 中文摘要

本文说明如何将 `weft-core` 从 **RELIK-plug / weft-app 单体仓库**拆分为独立 Git 仓库，便于单独发版、复用与后续发布到 crates.io。

**目标布局**：独立仓库根目录包含 `crates/weft-core`、`crates/weft-package-sdk`、`bins/`、`examples/`、`docs/`，以及根级 `config.example.toml` 与精简版 `packages/index.toml`。

**需要迁移的目录/文件**：`core/` → `crates/weft-core/`；`packages/sdk/` → `crates/weft-package-sdk/`；`config.example.toml`；仅含官方基础包条目的最小 `packages/index.toml`（完整包生态仍留在 weft-app 单体仓或通过 registry 拉取）。

**拆分方式**：推荐使用 `git subtree split` 保留 `core/` 相关提交历史；也可用 `git filter-repo` 做更激进的目录重写。

**weft-app 依赖方式**：短期用 **git tag + path/cargo git 依赖**；中期可选 **crates.io** 发布 `weft-core` 与 `weft-package-sdk`。

**embed-flutter 特性**：嵌入式 Flutter 客户端通过 FFI/cdylib 进程内调用，可不启动 HTTP 或仅作辅助；独立 `weft-core` 二进制则默认监听 `127.0.0.1:17830` 提供完整 HTTP API。拆仓后通过 Cargo feature `embed-flutter`（计划中）区分两种构建目标。

---

## Overview

This guide describes how to extract **weft-core** from the weft-app monorepo (`RELIK-plug`) into a standalone repository while keeping the Flutter desktop client and package ecosystem in the monorepo.

Goals:

- Independent release cadence and semver for `weft-core`
- Reusable HTTP API and Rust library for third-party integrators
- Clear dependency edge: monorepo → pinned `weft-core` revision

## Target repository layout

```
weft-core/                          # new standalone repo root
├── Cargo.toml                      # workspace root
├── Cargo.lock
├── README.md
├── LICENSE                         # Apache-2.0
├── config.example.toml
├── packages/
│   └── index.toml                  # minimal official package index (see below)
├── crates/
│   ├── weft-core/                  # from monorepo core/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   ├── tests/
│   │   ├── openapi.yaml
│   │   └── docs/
│   │       ├── SEMVER.md
│   │       └── MIGRATION.md
│   └── weft-package-sdk/           # from monorepo packages/sdk/
│       ├── Cargo.toml
│       └── src/
├── bins/                           # optional convenience wrappers
│   └── README.md                   # documents weft-core, weft, weft-rpc, weft-sign
├── examples/
│   ├── minimal-config/
│   └── hello-capability/
└── docs/
    ├── getting-started.md
    └── http-api.md                 # generated or linked from openapi.yaml
```

Workspace `Cargo.toml` (root):

```toml
[workspace]
members = ["crates/weft-core", "crates/weft-package-sdk"]
resolver = "2"
```

Update `crates/weft-core/Cargo.toml`:

```toml
weft-package-sdk = { path = "../weft-package-sdk" }
```

## Files and directories to move

| Monorepo path | Standalone path | Notes |
|---------------|-----------------|-------|
| `core/` | `crates/weft-core/` | Main library + bins (`weft-core`, `weft`, `weft-rpc`, `weft-sign`) |
| `packages/sdk/` | `crates/weft-package-sdk/` | Package authoring SDK; required dependency |
| `config.example.toml` | `config.example.toml` | Root example; paths adjusted for new layout |
| `packages/index.toml` | `packages/index.toml` | **Minimal** subset only (see below) |

### Minimal `packages/index.toml`

The standalone repo should ship a **trimmed** index containing only foundation packages needed to run core smoke tests and examples — not the full weft-app product catalog.

Suggested minimum entries:

- `agent-runtime` (wasm)
- `skills-runtime` (wasm)
- `session-events` (wasm)

Omit app-specific, installed-only, or product bundles (`weft-claw`, `tool-*`, etc.). The monorepo keeps the authoritative full index; production apps override `registry.package_source_url` or mount their own `packages/` tree.

### Files that stay in the monorepo

| Path | Reason |
|------|--------|
| `clients/` (Flutter) | Desktop shell; depends on weft-core via FFI or spawned process |
| `packages/official/*` (full set) | Product package sources and WASM builds |
| `packages/installed/*` | User-installed artifacts |
| `apps/` | Application manifests and generations |
| `docs/wiki/`, architecture docs | Product-level documentation |

## Git subtree split (recommended)

Run from the **monorepo root** (`RELIK-plug`).

### 1. Split `core/` history

```bash
# Create a branch containing only core/ history
git subtree split --prefix=core -b split/weft-core

# Clone a fresh standalone repo (or add a new remote)
git clone . ../weft-core-standalone
cd ../weft-core-standalone
git checkout split/weft-core

# Restructure: move core/ contents to crates/weft-core/
mkdir -p crates
git mv . crates/weft-core-tmp 2>/dev/null || true
# If subtree split leaves files at repo root of core/:
# mv src openapi.yaml docs Cargo.toml → crates/weft-core/
```

### 2. Split `packages/sdk/` and merge

```bash
cd /path/to/RELIK-plug
git subtree split --prefix=packages/sdk -b split/weft-package-sdk

cd /path/to/weft-core-standalone
git remote add monorepo /path/to/RELIK-plug
git fetch monorepo split/weft-package-sdk
git merge monorepo/split/weft-package-sdk --allow-unrelated-histories
# Move packages/sdk → crates/weft-package-sdk
```

### 3. Add root workspace files

Copy `config.example.toml`, write minimal `packages/index.toml`, add root `Cargo.toml`, `README.md`, `LICENSE`.

### Alternative: `git filter-repo`

For a clean single-pass extraction:

```bash
git filter-repo --path core/ --path packages/sdk/ --path config.example.toml
```

Then manually reshape directories to the target layout. This rewrites history more aggressively than subtree split.

## How weft-app should depend on weft-core

### Phase 1 — Git tag (current recommendation)

In the monorepo `Cargo.toml` or `clients/weft_client` build scripts:

```toml
# Option A: path dependency during transition
weft-core = { path = "../vendor/weft-core/crates/weft-core" }

# Option B: git dependency pinned to tag
weft-core = { git = "https://github.com/<org>/weft-core.git", tag = "v0.1.0" }
```

Flutter desktop build:

1. Vendor or submodule `weft-core` at a pinned tag.
2. Build `weft_core` cdylib for FFI, or spawn `weft-core` binary from `bins/`.
3. Pass `data_dir` and `runtime-token` path consistently (see `core/src/ffi.rs`).

### Phase 2 — Path dependency in monorepo vendor tree

```
RELIK-plug/
├── vendor/weft-core/     # git submodule @ tag v0.1.0
└── clients/...
```

CI checks out submodule at the tag recorded in monorepo `Cargo.lock`.

### Phase 3 — crates.io (future)

Publish:

- `weft-package-sdk` — lightweight, few deps, publish first
- `weft-core` — optional `embed-flutter` feature; default features for standalone server

Monorepo `Cargo.toml`:

```toml
weft-core = { version = "0.1", features = ["embed-flutter"] }
```

Pin minor versions pre-1.0; follow `SEMVER.md` for upgrade notes.

## `embed-flutter` feature: client vs standalone

Today, `weft-core` builds as both `lib` + `cdylib` with `flutter_rust_bridge` for in-process FFI. The planned Cargo feature flag separates deployment modes:

| Feature set | Binary | HTTP server | FFI / cdylib | Use case |
|-------------|--------|-------------|--------------|----------|
| **default** | `weft-core` | `127.0.0.1:17830` | optional | CLI, headless server, dev |
| **`embed-flutter`** | linked into Flutter | optional / internal | **required** | Desktop client in-process dispatch |
| **`standalone`** (alias of default) | `weft-core` exe | **required** | not linked | External tools, OpenAPI clients |

Design intent after split:

```toml
# crates/weft-core/Cargo.toml (planned)
[features]
default = ["standalone"]
standalone = []
embed-flutter = ["flutter_rust_bridge"]
```

- **Flutter client**: enable `embed-flutter`; call Rust via FFI; HTTP may still run on loopback for webview / devtools but is not the primary dispatch path.
- **Standalone server**: disable `embed-flutter`; ship `weft-core` binary; consumers use `openapi.yaml` and `/v1/*` + `/api/*`.

Document feature flags in the standalone repo `README.md` and keep FFI entry points (`core/src/ffi.rs`) stable across patch releases.

## Post-migration validation

1. `cargo test -p weft-core` passes in standalone repo.
2. `weft-core` starts and `GET http://127.0.0.1:17830/api/health` returns `{"status":"ok","version":"0.1.0"}`.
3. Monorepo Flutter build links against the same tag and passes smoke tests.
4. `openapi.yaml` and `SEMVER.md` versions match `Cargo.toml`.

## Related documents

- [`SEMVER.md`](./SEMVER.md) — crate and HTTP API versioning policy
- [`../openapi.yaml`](../openapi.yaml) — HTTP API skeleton
- Monorepo `docs/architecture/` — product runtime architecture (stays in weft-app)
