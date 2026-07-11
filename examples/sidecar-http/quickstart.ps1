# Quick smoke test for the sidecar-http example (Windows).
param(
    [switch]$Build
)

$ErrorActionPreference = 'Stop'

$ExampleDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = (Resolve-Path (Join-Path $ExampleDir '..\..')).Path
$Binary = Join-Path $RepoRoot 'target\release\weft-core.exe'
$ConfigDir = $ExampleDir
$DataDir = Join-Path $ExampleDir 'data'
$ConfigFile = Join-Path $ExampleDir 'config.toml'
$ConfigExample = Join-Path $ExampleDir 'config.example.toml'
$Port = 17830
$BaseUrl = "http://127.0.0.1:$Port"
$TokenPath = Join-Path $DataDir 'runtime-token'
$StartedProcess = $null

function Stop-StartedCore {
    if ($null -ne $StartedProcess -and -not $StartedProcess.HasExited) {
        Stop-Process -Id $StartedProcess.Id -Force -ErrorAction SilentlyContinue
    }
}

try {
    if (-not (Test-Path $ConfigFile)) {
        Copy-Item $ConfigExample $ConfigFile
        Write-Host "Created config.toml — edit [[providers.keys]] before chat completions work."
    }

    if ($Build) {
        Write-Host "Building weft-core..."
        Push-Location $RepoRoot
        try {
            cargo build --release -p weft-core --bin weft-core
        } finally {
            Pop-Location
        }
    }

    if (-not (Test-Path $Binary)) {
        throw "weft-core binary not found at: $Binary`nRun: .\quickstart.ps1 -Build   or: cd core; cargo build --release --bin weft-core"
    }

    function Test-Health {
        try {
            Invoke-RestMethod -Uri "$BaseUrl/api/health" -TimeoutSec 2 | Out-Null
            return $true
        } catch {
            return $false
        }
    }

    if (-not (Test-Health)) {
        Write-Host "Starting weft-core (background)..."
        New-Item -ItemType Directory -Force -Path $DataDir | Out-Null
        $StartedProcess = Start-Process `
            -FilePath $Binary `
            -ArgumentList @('--config-dir', $ConfigDir, '--data-dir', $DataDir) `
            -WorkingDirectory $RepoRoot `
            -PassThru `
            -WindowStyle Hidden

        $ready = $false
        for ($i = 0; $i -lt 30; $i++) {
            if (Test-Health) {
                $ready = $true
                break
            }
            Start-Sleep -Seconds 1
        }
        if (-not $ready) {
            throw "weft-core did not become healthy within 30 seconds."
        }
    }

    Write-Host ""
    Write-Host "=== Health ==="
    $health = Invoke-RestMethod -Uri "$BaseUrl/api/health"
    $health | ConvertTo-Json -Compress
    Write-Host ""

    Write-Host "=== Runtime token ==="
    Write-Host "Path: $TokenPath"
    if (-not (Test-Path $TokenPath)) {
        throw "Token file not found yet. Wait for weft-core to finish starting."
    }
    $token = (Get-Content $TokenPath -Raw).Trim()
    Write-Host "Token: $($token.Substring(0, [Math]::Min(8, $token.Length)))... (truncated)"
    Write-Host ""

    Write-Host "=== Chat completion (requires a valid provider key in config.toml) ==="
    Write-Host @"
curl.exe -s "$BaseUrl/v1/chat/completions" `
  -H "Authorization: Bearer $token" `
  -H "Content-Type: application/json" `
  -d '{\"model\":\"deepseek-chat\",\"messages\":[{\"role\":\"user\",\"content\":\"Say hello in one sentence.\"}]}'
"@
    Write-Host ""
} finally {
    Stop-StartedCore
}
