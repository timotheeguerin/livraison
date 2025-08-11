#!/usr/bin/env pwsh

$ErrorActionPreference = "Stop"

Write-Host "=== Step 1: Running install.ps1 ===" -ForegroundColor Green
Set-Location C:\test
& .\install.ps1

Write-Host ""
Write-Host "=== Step 2: Verifying installation ===" -ForegroundColor Green
& .\verify-installed.ps1

Write-Host ""
Write-Host "=== Test completed successfully ===" -ForegroundColor Green
