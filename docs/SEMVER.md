# Weft Core Versioning (Semver)

This document defines how **weft-core** (the Rust crate) and its **HTTP API** are versioned.

## Crate versioning (`weft-core`)

The `weft-core` crate follows [Semantic Versioning 2.0.0](https://semver.org/):

| Segment | Meaning |
|---------|---------|
| **MAJOR** | Breaking changes to the library surface, FFI contract, config schema, or default runtime behavior |
| **MINOR** | Backward-compatible features (new endpoints, optional config keys, new capability providers) |
| **PATCH** | Backward-compatible bug fixes and internal improvements |

The crate version is exposed at runtime via `GET /api/health` (`version` field) and `env!("CARGO_PKG_VERSION")`.

### Pre-1.0 policy (current: `0.1.x`)

While `weft-core` is **below 1.0.0**:

- **MINOR** bumps (`0.1 → 0.2`) may include breaking changes to `/api/*` management routes, config file layout, or internal FFI — but should be documented in release notes.
- **PATCH** bumps should not break existing clients that only use documented stable surfaces (see below).
- Breaking changes to `/v1/*` OpenAI-compatible routes require explicit deprecation notice and at least one minor release of warnings before removal.
- Treat `0.x` as **integration-phase**: downstream apps (weft-app monorepo, Flutter client) should pin to git tags or exact crate versions.

After **1.0.0**, standard semver applies: breaking changes require a MAJOR bump only.

## HTTP API versioning

Weft Core exposes two URL namespaces with different stability guarantees.

### `/v1/*` — OpenAI-compatible (stable)

| Route | Stability |
|-------|-----------|
| `POST /v1/chat/completions` | **Stable** — follows OpenAI Chat Completions semantics |
| `GET /v1/models` | **Stable** — follows OpenAI Models list semantics |

Rules:

- Request/response shapes align with the OpenAI API wherever possible.
- Weft-specific extensions use `x_`-prefixed fields (e.g. `x_provider`) and are optional.
- Breaking changes to `/v1/*` follow the same policy as a public SDK: deprecation period, changelog entry, and a MAJOR crate bump once at 1.0+.
- The `/v1` prefix is the **compatibility contract**. A future `/v2` would be introduced only for intentional OpenAI-divergent redesign, not for routine management features.

### `/api/*` — Management API (evolving)

Current routes (examples): `/api/health`, `/api/capabilities`, `/api/packages`, `/api/providers`, `/api/apps`, and nested app/scene/generation routes.

Rules:

- **Pre-1.0**: `/api/*` routes may change between minor crate versions. Clients should not assume cross-version stability without checking release notes.
- **Post-1.0 plan**: introduce explicit version prefix **`/api/v1/*`** for the stable management surface.
  - Unversioned `/api/*` becomes an alias to the latest stable management version during a transition window.
  - New breaking management features ship under `/api/v2/*` (or the next major API version) while `/api/v1/*` remains frozen.
- `GET /api/health` stays unversioned as a simple liveness probe (like `/health`).

### Version alignment matrix

| Surface | Current path | Target stable path | Tied to crate version |
|---------|--------------|--------------------|-----------------------|
| OpenAI compat | `/v1/*` | `/v1/*` (unchanged) | Documented in crate CHANGELOG |
| Management | `/api/*` | `/api/v1/*` (future) | Minor API version may trail crate minor |
| Health | `/api/health`, `/health` | Unversioned | Always available |

## Release checklist

When cutting a release:

1. Bump `version` in `core/Cargo.toml`.
2. Update `core/openapi.yaml` `info.version` to match.
3. Document breaking changes under **HTTP API** and **Config** sections in the release notes.
4. Tag the git commit: `weft-core-v0.1.0` (or `v0.1.0` if standalone repo).
5. If `/api/*` behavior changed, note migration steps for weft-app and other consumers.

## Consumer guidance

| Consumer | Recommendation |
|----------|----------------|
| **Flutter / weft-app** | Pin `weft-core` via git tag or path dependency; read `MIGRATION.md` when splitting repos |
| **External HTTP clients** | Depend only on `/v1/*` for long-term stability; treat `/api/*` as internal until `/api/v1` is declared stable |
| **Package authors** | Track capability names and provider contracts via `packages/sdk`; crate minor bumps may add capabilities without breaking existing ones |

## Related files

- `core/openapi.yaml` — machine-readable API skeleton
- `core/docs/MIGRATION.md` — extracting weft-core to a standalone repository
- `config.example.toml` — configuration schema (versioned alongside crate)
