# Weft Core

**OpenAI-compatible API + WASM capability plugin OS**

Weft Core is a standalone Rust runtime that exposes an OpenAI-compatible HTTP API and loads **capability packages** (WASM via [Extism](https://extism.org/), native, or embedded) to extend behavior — agent loops, tools, memory, workflows, and more. Any client that speaks the OpenAI protocol (or plain HTTP) can drive it; no UI required.

> [!WARNING]
> **Early preview.** Weft Core is under active development. HTTP endpoints, capability contracts, and package manifests may change without notice. Treat this as an exploration preview, not a production dependency.

---

## Quick start

Requires a stable Rust toolchain.

```bash
# From the repository root
mkdir -p config
cp config.example.toml config/config.toml
# Edit config/config.toml and add at least one [[providers]] entry with API keys

cargo build --release -p weft-core
cargo run --release -p weft-core --bin weft-core
```

The server binds to **`127.0.0.1:17830`** by default (loopback only).

### Health check

```bash
curl -s http://127.0.0.1:17830/api/health
# {"status":"ok","version":"0.1.0"}
```

### Chat completion

Most endpoints require a loopback bearer token. On first start, the core writes one to `data/runtime-token`:

```bash
TOKEN=$(cat data/runtime-token)

curl -s http://127.0.0.1:17830/v1/chat/completions \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "deepseek-chat",
    "messages": [{"role": "user", "content": "Hello!"}]
  }'
```

Configure providers and default routing in `config/config.toml`. See [`config.example.toml`](config.example.toml) for the full template.

---

## What you get

- **OpenAI-compatible surface** — `/v1/chat/completions` (SSE streaming), `/v1/models`
- **Multi-provider router** — OpenAI, Anthropic, DeepSeek, OpenRouter, and any compatible endpoint, with API-key failover/rotation
- **Capability registry** — stable capability ids (`agent.runtime`, `tool.shell`, `memory.store`, …) resolved to packages at startup
- **Pluggable pipeline** — request → transform → provider → response; each stage overridable by WASM packages
- **Management API** — `/api/apps`, `/api/capabilities`, `/api/packages`, `/api/providers`, and more

---

## How it compares to Pi (pi-agent-core)

[Pi Agent Core](https://github.com/badlogic/pi-mono/tree/main/packages/agent) (`@mariozechner/pi-agent-core`) is a **TypeScript in-process library**: you embed an `Agent` class, wire tools in code, and subscribe to streaming events. It sits on top of `@mariozechner/pi-ai` for unified LLM calls.

| | **Pi Agent Core** | **Weft Core** |
|---|---|---|
| **Form factor** | npm library (in-process) | Standalone HTTP server |
| **Language** | TypeScript | Rust |
| **Extension model** | Tools & hooks in application code | WASM / native **capability packages** loaded at runtime |
| **API surface** | Programmatic `Agent` class | OpenAI-compatible REST + management API |
| **Agent loop** | Built into the library | Provided by packages (e.g. `agent-core`) on the capability bus |
| **Best for** | Embedding agents inside Node/Browser apps | Running a local agent **platform** any HTTP client can talk to |

Pi excels at embedding a minimal agent loop in your app. Weft Core excels at being a **local capability OS** — swap packages without recompiling, serve multiple clients (scripts, desktop UI, OpenAI SDK) over one loopback endpoint.

---

## Official desktop client (optional)

[Weft Desktop](https://github.com/ailiheizi/weft/tree/main/clients/weft_client) is the optional official UI — chat, multi-agent teams, package manager, and product surfaces. It connects to the same loopback API (`127.0.0.1:17830`). You can run Weft Core alone and bring your own client.

Prebuilt desktop bundles: [**Weft Releases**](https://github.com/ailiheizi/weft/releases).

---

## Documentation & references

| Resource | Description |
|---|---|
| [examples/](examples/) | Minimal integration examples (sidecar HTTP, custom WASM packages) |
| [openapi.yaml](openapi.yaml) | OpenAPI 3 spec for the HTTP surface |
| [docs/SEMVER.md](docs/SEMVER.md) | Versioning policy |
| [docs/CRATES_IO.md](docs/CRATES_IO.md) | Publishing to crates.io |
| [Weft monorepo architecture](https://github.com/ailiheizi/weft/blob/main/docs/ARCHITECTURE.md) | Full platform architecture (client + packages) |
| [Weft package index](https://github.com/ailiheizi/weft/blob/main/packages/index.toml) | Full capability → package source authority |

---

## Configuration

```toml
[core]
host = "127.0.0.1"
port = 17830

[[providers]]
name = "deepseek"
base_url = "https://api.deepseek.com"
format = "openai"
models = ["deepseek-chat"]

[[providers.keys]]
value = "sk-your-key"
```

`config/config.toml` is gitignored; only the example template is committed.

---

## License

Licensed under the [Apache License 2.0](LICENSE).

## Install from crates.io

```toml
[dependencies]
weft-core = "0.1"
weft-package-sdk = "0.1"
```

Binary: download from [**weft-core Releases**](https://github.com/ailiheizi/weft-core/releases) or build with `cargo install weft-core` (after first crates.io publish).
