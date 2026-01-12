$ErrorActionPreference = "Stop"

$Repo = "miyaniakshar1234/zenith-cli"
$Binary = "zenith-cli.exe"
$InstallDir = "$env:USERPROFILE\.zenith\bin"

# Detect Architecture
# Currently we only build windows-amd64 in the workflow
$AssetName = "zenith-cli-windows-amd64.exe"

Write-Host "🚀 Installing Zenith CLI..." -ForegroundColor Cyan

# Create Directory
if (!(Test-Path -Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

# Download
$DownloadUrl = "https://github.com/$Repo/releases/latest/download/$AssetName"
$OutputPath = "$InstallDir\$Binary"

Write-Host "⬇️  Downloading from GitHub..."
Invoke-WebRequest -Uri $DownloadUrl -OutFile $OutputPath

# Add to PATH (User Environment)
$UserPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::User)
if ($UserPath -notlike "*$InstallDir*") {
    Write-Host "🔧 Adding to PATH..."
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", [EnvironmentVariableTarget]::User)
    $env:Path += ";$InstallDir"
    Write-Host "✅ PATH updated. You may need to restart your terminal." -ForegroundColor Green
} else {
    Write-Host "✅ Already in PATH." -ForegroundColor Green
}

Write-Host "🎉 Installation Complete! Type 'zenith-cli' to launch." -ForegroundColor Green
