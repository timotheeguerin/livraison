#!/usr/bin/env pwsh

$ErrorActionPreference = "Stop"

Write-Host "Current user profile directory:"
Get-ChildItem $env:USERPROFILE | Format-Table Name, Mode, LastWriteTime
Write-Host "---"
Write-Host "Current PATH: $env:PATH"
Write-Host "---"

# Refresh environment variables from registry (this is what matters for Windows)
Write-Host "Refreshing PATH from registry..."
$userPath = [System.Environment]::GetEnvironmentVariable("PATH", "User")
$machinePath = [System.Environment]::GetEnvironmentVariable("PATH", "Machine")
$env:PATH = "$userPath;$machinePath"

Write-Host "Updated PATH: $env:PATH"
Write-Host "---"

# Test livraison command
Write-Host "Testing livraison command..."
livraison --version

Write-Host "✓ Livraison is available and working!"
