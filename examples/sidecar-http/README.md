# Sidecar HTTP Example

Run **weft-core** as a loopback HTTP sidecar and call it with plain `curl` — no desktop client required.

---

## English

### Prerequisites

- [Rust](https://rustup.rs/) stable toolchain
- `curl` (included on macOS/Linux; use `curl.exe` on Windows)

### 1. Copy config

From the repository root:

```bash
cp core/examples/sidecar-http/config.example.toml core/examples/sidecar-http/config.toml
```

Edit `config.toml` and replace `sk-your-deepseek-key` with a real API key (or swap the provider block for OpenAI, Anthropic, etc.).

### 2. Build weft-core

```bash
cd core
cargo build --release --bin weft-core
cd ..
```

**Windows (PowerShell):**

```powershell
cd core
cargo build --release --bin weft-core
cd ..
```

Binary output:

| OS | Path |
|---|---|
| Linux / macOS | `core/target/release/weft-core` |
| Windows | `core\target\release\weft-core.exe` |

### 3. Start the server

Run from the **repository root** so packages and config resolve correctly:

```bash
./core/target/release/weft-core \
  --config-dir core/examples/sidecar-http \
  --data-dir core/examples/sidecar-http/data
```

**Windows (PowerShell):**

```powershell
.\core\target\release\weft-core.exe `
  --config-dir core\examples\sidecar-http `
  --data-dir core\examples\sidecar-http\data
```

The server listens on **`127.0.0.1:17830`** by default.

### 4. Read the runtime token

On first start, weft-core writes a loopback bearer token:

| OS | Path |
|---|---|
| Linux / macOS | `core/examples/sidecar-http/data/runtime-token` |
| Windows | `core\examples\sidecar-http\data\runtime-token` |

```bash
# Linux / macOS
export TOKEN=$(cat core/examples/sidecar-http/data/runtime-token)
```

```powershell
# Windows
$token = (Get-Content core\examples\sidecar-http\data\runtime-token -Raw).Trim()
```

Most endpoints require `Authorization: Bearer <token>`. `/api/health` and `/v1/models` are public.

### 5. Health check (no token)

```bash
curl -s http://127.0.0.1:17830/api/health
# {"status":"ok","version":"0.1.0"}
```

```powershell
curl.exe -s http://127.0.0.1:17830/api/health
```

### 6. Chat completion

```bash
curl -s http://127.0.0.1:17830/v1/chat/completions \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "deepseek-chat",
    "messages": [{"role": "user", "content": "Hello!"}]
  }'
```

```powershell
curl.exe -s http://127.0.0.1:17830/v1/chat/completions `
  -H "Authorization: Bearer $token" `
  -H "Content-Type: application/json" `
  -d '{"model":"deepseek-chat","messages":[{"role":"user","content":"Hello!"}]}'
```

### One-liner scripts

```bash
# Linux / macOS — optional --build to compile first
bash core/examples/sidecar-http/quickstart.sh
bash core/examples/sidecar-http/quickstart.sh --build
```

```powershell
# Windows
.\core\examples\sidecar-http\quickstart.ps1
.\core\examples\sidecar-http\quickstart.ps1 -Build
```

Scripts copy config if missing, optionally build, start weft-core in the background, hit `/api/health`, print the token path, and show a ready-to-paste chat `curl` command.

---

## 中文

### 前置条件

- 已安装 [Rust](https://rustup.rs/) stable 工具链
- `curl`（macOS / Linux 自带；Windows 使用 `curl.exe`）

### 1. 复制配置

在仓库根目录执行：

```bash
cp core/examples/sidecar-http/config.example.toml core/examples/sidecar-http/config.toml
```

编辑 `config.toml`，将 `sk-your-deepseek-key` 替换为真实 API Key（也可改成 OpenAI、Anthropic 等其他 provider）。

### 2. 编译 weft-core

```bash
cd core
cargo build --release --bin weft-core
cd ..
```

**Windows (PowerShell)：**

```powershell
cd core
cargo build --release --bin weft-core
cd ..
```

产物路径：

| 系统 | 路径 |
|---|---|
| Linux / macOS | `core/target/release/weft-core` |
| Windows | `core\target\release\weft-core.exe` |

### 3. 启动服务

请在**仓库根目录**运行（以便正确加载 packages 与配置）：

```bash
./core/target/release/weft-core \
  --config-dir core/examples/sidecar-http \
  --data-dir core/examples/sidecar-http/data
```

**Windows (PowerShell)：**

```powershell
.\core\target\release\weft-core.exe `
  --config-dir core\examples\sidecar-http `
  --data-dir core\examples\sidecar-http\data
```

默认监听 **`127.0.0.1:17830`**（仅本机回环）。

### 4. 读取 runtime-token

首次启动时，weft-core 会生成本机鉴权 token：

| 系统 | 路径 |
|---|---|
| Linux / macOS | `core/examples/sidecar-http/data/runtime-token` |
| Windows | `core\examples\sidecar-http\data\runtime-token` |

```bash
# Linux / macOS
export TOKEN=$(cat core/examples/sidecar-http/data/runtime-token)
```

```powershell
# Windows
$token = (Get-Content core\examples\sidecar-http\data\runtime-token -Raw).Trim()
```

除 `/api/health`、`/v1/models` 等放行端点外，大多数 API 需要 `Authorization: Bearer <token>`。

### 5. 健康检查（无需 token）

```bash
curl -s http://127.0.0.1:17830/api/health
# {"status":"ok","version":"0.1.0"}
```

```powershell
curl.exe -s http://127.0.0.1:17830/api/health
```

### 6. 聊天补全

```bash
curl -s http://127.0.0.1:17830/v1/chat/completions \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "deepseek-chat",
    "messages": [{"role": "user", "content": "你好！"}]
  }'
```

```powershell
curl.exe -s http://127.0.0.1:17830/v1/chat/completions `
  -H "Authorization: Bearer $token" `
  -H "Content-Type: application/json" `
  -d '{"model":"deepseek-chat","messages":[{"role":"user","content":"你好！"}]}'
```

### 一键脚本

```bash
# Linux / macOS — 加 --build 会先编译
bash core/examples/sidecar-http/quickstart.sh
bash core/examples/sidecar-http/quickstart.sh --build
```

```powershell
# Windows
.\core\examples\sidecar-http\quickstart.ps1
.\core\examples\sidecar-http\quickstart.ps1 -Build
```

脚本会自动复制配置、可选编译、后台启动 weft-core、请求 `/api/health`、打印 token 路径，并输出可直接粘贴的 chat `curl` 命令。

---

## See also

- [`../../README.md`](../../README.md) — weft-core overview
- [`../../../config.example.toml`](../../../config.example.toml) — full config template
- [`../custom-package/README.md`](../custom-package/README.md) — build WASM capability packages
