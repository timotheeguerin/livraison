#!/usr/bin/env pwsh

$ErrorActionPreference = "Stop"

Write-Host "=== Step 1: Running install.ps1 ===" -ForegroundColor Green
& ..\install.ps1
if ($LASTEXITCODE -ne 0) {
    Write-Error "install.ps1 failed with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
}

Write-Host ""
Write-Host "=== Step 2: Verifying installation ===" -ForegroundColor Green
& .\verify-installed.ps1
if ($LASTEXITCODE -ne 0) {
    Write-Error "verify-installed.ps1 failed with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
}

Write-Host ""
Write-Host "=== Test completed successfully ===" -ForegroundColor Green
