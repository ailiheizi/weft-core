# Weft Core

**OpenAI 兼容 API + WASM 能力插件操作系统**

Weft Core 是一个独立的 Rust 运行时：对外暴露 OpenAI 兼容的 HTTP API，并通过 **能力包**（[Extism](https://extism.org/) WASM、原生或内嵌）扩展行为——智能体循环、工具、记忆、工作流等。任何支持 OpenAI 协议（或普通 HTTP）的客户端都可以驱动它，**不依赖 UI**。

> [!WARNING]
> **早期预览。** Weft Core 仍在积极开发中。HTTP 端点、能力契约和包清单可能随时变更。请将其视为探索性预览，而非生产依赖。

---

## 快速开始

需要 stable Rust 工具链。

```bash
# 在仓库根目录执行
cp config.example.toml config/config.toml
# 编辑 config/config.toml，至少添加一个带 API Key 的 [[providers]] 条目

cd core
cargo build --release
cargo run --release --bin weft-core
```

服务默认监听 **`127.0.0.1:17830`**（仅回环地址）。

### 健康检查

```bash
curl -s http://127.0.0.1:17830/api/health
# {"status":"ok","version":"0.1.0"}
```

### 聊天补全

大多数端点需要回环 Bearer Token。首次启动时，核心会在 `data/runtime-token` 写入 Token：

```bash
TOKEN=$(cat ../data/runtime-token)

curl -s http://127.0.0.1:17830/v1/chat/completions \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "deepseek-chat",
    "messages": [{"role": "user", "content": "你好！"}]
  }'
```

在 `config/config.toml` 中配置 Provider 与默认路由。完整模板见 [`../config.example.toml`](../config.example.toml)。

---

## 核心能力

- **OpenAI 兼容接口** — `/v1/chat/completions`（SSE 流式）、`/v1/models`
- **多 Provider 路由** — OpenAI、Anthropic、DeepSeek、OpenRouter 及任意兼容端点，支持 API Key 故障转移/轮询
- **能力注册表** — 稳定的能力 id（`agent.runtime`、`tool.shell`、`memory.store` 等），启动时解析到具体包
- **可插拔管道** — 请求 → 转换 → Provider → 响应；每一层均可被 WASM 包覆盖
- **管理 API** — `/api/apps`、`/api/capabilities`、`/api/packages`、`/api/providers` 等

---

## 与 Pi（pi-agent-core）的对比

[Pi Agent Core](https://github.com/badlogic/pi-mono/tree/main/packages/agent)（`@mariozechner/pi-agent-core`）是一个 **TypeScript 进程内库**：嵌入 `Agent` 类、在代码中注册工具、订阅流式事件。它基于 `@mariozechner/pi-ai` 统一 LLM 调用。

| | **Pi Agent Core** | **Weft Core** |
|---|---|---|
| **形态** | npm 库（进程内） | 独立 HTTP 服务 |
| **语言** | TypeScript | Rust |
| **扩展方式** | 在应用代码中注册工具与钩子 | 运行时加载 WASM / 原生 **能力包** |
| **API 形态** | 编程式 `Agent` 类 | OpenAI 兼容 REST + 管理 API |
| **Agent 循环** | 内置于库 | 由能力包（如 `agent-core`）在能力总线上提供 |
| **适用场景** | 在 Node/浏览器应用中嵌入 Agent | 作为本地 Agent **平台**，任意 HTTP 客户端均可接入 |

Pi 擅长在应用内嵌入精简的 Agent 循环。Weft Core 擅长作为 **本地能力操作系统** —— 无需重编译即可更换包，通过单一回环端点服务多种客户端（脚本、桌面 UI、OpenAI SDK）。

---

## 官方桌面客户端（可选）

[Weft Desktop](../clients/weft_client/) 是可选的官方 UI —— 聊天、多智能体团队、包管理器及产品界面。它连接同一回环 API（`127.0.0.1:17830`）。你可以只运行 Weft Core，自带客户端。

预构建包：[**Releases**](https://github.com/ailiheizi/weft/releases)。

---

## 文档与参考

| 资源 | 说明 |
|---|---|
| [examples/](examples/) | 最小集成示例（curl、SDK、自定义客户端） |
| [openapi.yaml](openapi.yaml) | HTTP 接口 OpenAPI 3 规范 |
| [../docs/ARCHITECTURE.md](../docs/ARCHITECTURE.md) | 客户端、核心与包架构详解 |
| [../docs/FEATURES.md](../docs/FEATURES.md) | 功能导览与能力参考 |
| [../packages/index.toml](../packages/index.toml) | 能力 → 包映射的权威来源 |
| [README-signing.md](README-signing.md) | 包签名与验证 |

---

## 配置

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

`config/config.toml` 已被 git 忽略；仓库中仅提交示例模板。

---

## 许可证

基于 [Apache License 2.0](../LICENSE) 许可。
