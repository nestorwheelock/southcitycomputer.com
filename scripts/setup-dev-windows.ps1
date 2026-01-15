# Development Environment Setup for Windows
# South City Computer - scc-server
#
# This script installs all dependencies needed to build and develop the server.
# Tested on: Windows 10, Windows 11
#
# Usage: Run PowerShell as Administrator, then:
#   Set-ExecutionPolicy Bypass -Scope Process -Force
#   .\scripts\setup-dev-windows.ps1
#
# Or run without admin (will prompt for elevation when needed):
#   powershell -ExecutionPolicy Bypass -File .\scripts\setup-dev-windows.ps1

param(
    [switch]$SkipRust,
    [switch]$SkipNode,
    [switch]$SkipTools,
    [switch]$Help
)

# =============================================================================
# Helper Functions
# =============================================================================

function Write-ColorOutput {
    param([string]$Message, [string]$Color = "White")
    Write-Host $Message -ForegroundColor $Color
}

function Write-Info { Write-ColorOutput "[INFO] $args" "Cyan" }
function Write-Success { Write-ColorOutput "[OK] $args" "Green" }
function Write-Warning { Write-ColorOutput "[WARN] $args" "Yellow" }
function Write-Error { Write-ColorOutput "[ERROR] $args" "Red" }

function Test-Administrator {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Install-WithWinget {
    param([string]$PackageId, [string]$Name)

    Write-Info "Installing $Name..."

    $result = winget list --id $PackageId 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Info "$Name already installed"
        return $true
    }

    winget install --id $PackageId --silent --accept-package-agreements --accept-source-agreements
    if ($LASTEXITCODE -eq 0) {
        Write-Success "$Name installed"
        return $true
    } else {
        Write-Warning "Failed to install $Name via winget"
        return $false
    }
}

# =============================================================================
# Header
# =============================================================================

Clear-Host
Write-Host ""
Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Yellow
Write-Host "║  South City Computer - Development Environment Setup      ║" -ForegroundColor Yellow
Write-Host "║  Windows                                                   ║" -ForegroundColor Yellow
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Yellow
Write-Host ""

if ($Help) {
    Write-Host "Usage: .\setup-dev-windows.ps1 [options]"
    Write-Host ""
    Write-Host "Options:"
    Write-Host "  -SkipRust    Skip Rust installation"
    Write-Host "  -SkipNode    Skip Node.js installation"
    Write-Host "  -SkipTools   Skip optional tools"
    Write-Host "  -Help        Show this help"
    Write-Host ""
    exit 0
}

# Check for winget
if (-not (Get-Command winget -ErrorAction SilentlyContinue)) {
    Write-Error "winget not found. Please install App Installer from Microsoft Store."
    Write-Host "https://www.microsoft.com/p/app-installer/9nblggh4nns1"
    exit 1
}

# =============================================================================
# Visual Studio Build Tools (Required for Rust on Windows)
# =============================================================================

Write-Info "Checking for Visual Studio Build Tools..."

$vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
$hasVS = $false

if (Test-Path $vsWhere) {
    $vsInstalls = & $vsWhere -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
    if ($vsInstalls) {
        Write-Info "Visual Studio Build Tools found"
        $hasVS = $true
    }
}

if (-not $hasVS) {
    Write-Info "Installing Visual Studio Build Tools..."
    Write-Warning "This may take 10-20 minutes and require a restart."

    # Download and run VS Build Tools installer
    $vsbtUrl = "https://aka.ms/vs/17/release/vs_BuildTools.exe"
    $vsbtInstaller = "$env:TEMP\vs_BuildTools.exe"

    Write-Info "Downloading Visual Studio Build Tools..."
    Invoke-WebRequest -Uri $vsbtUrl -OutFile $vsbtInstaller

    Write-Info "Installing (this will take a while)..."
    Start-Process -FilePath $vsbtInstaller -ArgumentList `
        "--quiet", "--wait", "--norestart", `
        "--add", "Microsoft.VisualStudio.Workload.VCTools", `
        "--add", "Microsoft.VisualStudio.Component.Windows11SDK.22621", `
        "--includeRecommended" `
        -Wait

    Write-Success "Visual Studio Build Tools installed"
    Write-Warning "You may need to restart your terminal or computer."
}

# =============================================================================
# Rust Installation
# =============================================================================

if (-not $SkipRust) {
    Write-Info "Checking for Rust..."

    if (Get-Command rustc -ErrorAction SilentlyContinue) {
        $rustVersion = rustc --version
        Write-Info "Rust already installed: $rustVersion"
        Write-Info "Updating Rust..."
        rustup update stable
    } else {
        Write-Info "Installing Rust via rustup..."

        $rustupInit = "$env:TEMP\rustup-init.exe"
        Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile $rustupInit

        # Install with default options
        Start-Process -FilePath $rustupInit -ArgumentList "-y" -Wait

        # Add to PATH for current session
        $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"

        Write-Success "Rust installed"
    }

    # Install Rust components
    Write-Info "Installing Rust components..."
    rustup component add clippy
    rustup component add rustfmt
    rustup component add rust-analyzer 2>$null

    Write-Success "Rust components installed"

    # Install cargo tools
    Write-Info "Installing cargo tools..."

    if (-not (Get-Command cargo-watch -ErrorAction SilentlyContinue)) {
        cargo install cargo-watch
    }
    if (-not (Get-Command cargo-add -ErrorAction SilentlyContinue)) {
        cargo install cargo-edit
    }

    Write-Success "Cargo tools installed"
}

# =============================================================================
# Node.js Installation
# =============================================================================

if (-not $SkipNode) {
    Write-Info "Checking for Node.js..."

    if (Get-Command node -ErrorAction SilentlyContinue) {
        $nodeVersion = node --version
        Write-Info "Node.js already installed: $nodeVersion"
    } else {
        Install-WithWinget "OpenJS.NodeJS.LTS" "Node.js LTS"
        # Refresh PATH
        $env:PATH = [System.Environment]::GetEnvironmentVariable("PATH", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("PATH", "User")
    }

    # Install minification tools
    Write-Info "Installing CSS/JS minification tools..."
    npm install -g clean-css-cli terser 2>$null
    Write-Success "Minification tools installed"
}

# =============================================================================
# Optional Tools
# =============================================================================

if (-not $SkipTools) {
    Write-Info "Installing optional development tools..."

    # Git
    if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
        Install-WithWinget "Git.Git" "Git"
    } else {
        Write-Info "Git already installed"
    }

    # jq (JSON processor)
    Install-WithWinget "jqlang.jq" "jq"

    # curl (usually built into Windows 10/11)
    if (-not (Get-Command curl.exe -ErrorAction SilentlyContinue)) {
        Install-WithWinget "cURL.cURL" "cURL"
    }
}

# =============================================================================
# Image Tools (Optional)
# =============================================================================

Write-Info "Checking for image tools..."

# WebP tools
$cwebpPath = "$env:ProgramFiles\libwebp\bin\cwebp.exe"
if (-not (Test-Path $cwebpPath)) {
    Write-Warning "WebP tools not installed. To install manually:"
    Write-Host "  1. Download from: https://developers.google.com/speed/webp/download"
    Write-Host "  2. Extract to: C:\Program Files\libwebp"
    Write-Host "  3. Add to PATH: C:\Program Files\libwebp\bin"
} else {
    Write-Info "WebP tools found"
}

# =============================================================================
# Verify Installation
# =============================================================================

Write-Host ""
Write-Info "Verifying installation..."
Write-Host ""

Write-Host "System:" -ForegroundColor Cyan
Write-Host "  OS:          $([System.Environment]::OSVersion.VersionString)"
Write-Host "  PowerShell:  $($PSVersionTable.PSVersion)"
Write-Host ""

Write-Host "Development Tools:" -ForegroundColor Cyan
try { Write-Host "  Rust:        $(rustc --version)" } catch { Write-Host "  Rust:        Not found" }
try { Write-Host "  Cargo:       $(cargo --version)" } catch { Write-Host "  Cargo:       Not found" }
try { Write-Host "  Git:         $(git --version)" } catch { Write-Host "  Git:         Not found" }
Write-Host ""

Write-Host "Node.js Tools:" -ForegroundColor Cyan
try { Write-Host "  Node:        $(node --version)" } catch { Write-Host "  Node:        Not found" }
try { Write-Host "  npm:         $(npm --version)" } catch { Write-Host "  npm:         Not found" }
try { Write-Host "  cleancss:    $(cleancss --version)" } catch { Write-Host "  cleancss:    Not found" }
try { Write-Host "  terser:      $(terser --version)" } catch { Write-Host "  terser:      Not found" }
Write-Host ""

# =============================================================================
# Build Test
# =============================================================================

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectDir = Split-Path -Parent $scriptDir
$cargoToml = Join-Path $projectDir "contact-handler\Cargo.toml"

if (Test-Path $cargoToml) {
    Write-Info "Testing build..."
    Push-Location (Join-Path $projectDir "contact-handler")

    try {
        cargo build --release 2>&1 | Out-Null
        if ($LASTEXITCODE -eq 0) {
            $binaryPath = "target\release\scc-server.exe"
            if (Test-Path $binaryPath) {
                $size = (Get-Item $binaryPath).Length / 1MB
                Write-Success "Build successful! Binary size: $([math]::Round($size, 1)) MB"
            }
        } else {
            Write-Warning "Build test failed. Check error messages above."
        }
    } catch {
        Write-Warning "Build test failed: $_"
    }

    Pop-Location
} else {
    Write-Info "Project not found at $projectDir - skipping build test"
}

# =============================================================================
# Summary
# =============================================================================

Write-Host ""
Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║  Development environment setup complete!                   ║" -ForegroundColor Green
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:"
Write-Host "  1. cd contact-handler"
Write-Host "  2. cargo build --release     # Build production binary"
Write-Host "  3. cargo run --bin scc-dev   # Run development server"
Write-Host "  4. cargo test                # Run tests"
Write-Host ""
Write-Host "Useful commands:"
Write-Host "  cargo watch -x run           # Auto-rebuild on changes"
Write-Host "  cargo clippy                 # Run linter"
Write-Host "  cargo fmt                    # Format code"
Write-Host ""

if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
    Write-Warning "Rust not in PATH. Please restart your terminal or run:"
    Write-Host '  $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"'
}
