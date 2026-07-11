# Custom WASM Capability Package

Build your own **capability package** that weft-core loads at runtime (WASM via [Extism](https://extism.org/)).

## SDK

All WASM packages share the host-function wrappers in [`packages/sdk`](../../../packages/sdk):

| Crate | Path |
|---|---|
| `weft-package-sdk` | [`packages/sdk/Cargo.toml`](../../../packages/sdk/Cargo.toml) |

Add it to your package `Cargo.toml`:

```toml
[dependencies]
weft-package-sdk = { path = "../../sdk" }
extism-pdk = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

The SDK exposes helpers for KV, files, HTTP, LLM calls, package-to-package RPC, SQLite, and more — see [`packages/sdk/src/lib.rs`](../../../packages/sdk/src/lib.rs).

## Reference implementation

Copy the layout from an official package, e.g. [`packages/official/agent-core`](../../../packages/official/agent-core):

```
my-package/
├── Cargo.toml          # crate-type = ["cdylib"], depends on weft-package-sdk
├── package.toml        # capability manifest (identity, capabilities, entry)
└── src/lib.rs          # #[extism_pdk::plugin_fn] exports
```

## Build

Install the WASI target once:

```bash
rustup target add wasm32-wasip1
```

From your package directory:

```bash
cargo build --release --target wasm32-wasip1
```

Output: `target/wasm32-wasip1/release/<crate_name>.wasm`

weft-core discovers packages under `packages/` (see [`packages/index.toml`](../../../packages/index.toml)). After building, restart the core or place the `.wasm` where your `package.toml` entry points.

## Quick checklist

1. Create `package.toml` with capability id(s) and entry path.
2. Implement exports with `extism-pdk` + `weft-package-sdk`.
3. `cargo build --release --target wasm32-wasip1`
4. Register in `packages/index.toml` (or use `local://` registry in config).
5. Restart weft-core and verify via `GET /api/capabilities`.

## See also

- [`../../../docs/ARCHITECTURE.md`](../../../docs/ARCHITECTURE.md) — package runtime and capability resolution
- [`../../README-signing.md`](../../README-signing.md) — signing packages with `weft-sign`
- [`../sidecar-http/README.md`](../sidecar-http/README.md) — run weft-core and test HTTP endpoints
