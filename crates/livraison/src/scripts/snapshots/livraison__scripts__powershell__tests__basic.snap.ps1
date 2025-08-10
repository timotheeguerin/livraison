#!/usr/bin/env pwsh
param(
  [String]$Version = "latest",
  # Skips adding the test.exe directory to the user's %PATH%
  [Switch]$NoPathUpdate = $false,
  # Skips adding the test.exe to the list of installed programs
  [Switch]$NoRegisterInstallation = $false,

  # Debugging: Always download with 'Invoke-RestMethod' instead of 'curl.exe'
  [Switch]$DownloadWithoutCurl = $false
);

function New-TemporaryDirectory {
    $tmp = [System.IO.Path]::GetTempPath() # Not $env:TEMP, see https://stackoverflow.com/a/946017
    $name = (New-Guid).ToString("N")
    New-Item -ItemType Directory -Path (Join-Path $tmp $name)
}

function Get-Download-Url {
    param(
      [String] $Version, 
      [String] $filename
    )
    
    return "https://example.com/$Version/$(Get-Filename -target $target)"

}

function Get-Target
{
  # Detect the target platform based on system info
  if ((Get-CimInstance Win32_ComputerSystem).SystemType -match "x64-based")
  {
    return "windows-x64"
  }
  elseif ((Get-CimInstance Win32_ComputerSystem).SystemType -match "ARM64-based")
  {
    return "windows-arm64"
  }
  else
  {
    return "windows-x64"
  }
}

function Get-Filename {
    param([String] $target)

    return "test-$target.zip"
}

function Install-test {
  param([string]$Version);

  $target = Get-Target

  $testRoot = if ($env:TEST_INSTALL) { $env:TEST_INSTALL } else { "$HOME\.test" }
  $testBin = mkdir -Force "$testRoot\bin"

  try {
    Remove-Item "$testBin\test.exe" -Force
  } catch [System.Management.Automation.ItemNotFoundException] {
    # ignore
  } catch [System.UnauthorizedAccessException] {
    $openProcesses = Get-Process -Name test | Where-Object { $_.Path -eq "$testBin\test.exe" }
    if ($openProcesses.Count -gt 0) {
      Write-Output "Install Failed - An older installation exists and is open. Please close open test processes and try again."
      return 1
    }
    Write-Output "Install Failed - An unknown error occurred while trying to remove the existing installation"
    Write-Output $_
    return 1
  } catch {
    Write-Output "Install Failed - An unknown error occurred while trying to remove the existing installation"
    Write-Output $_
    return 1
  }

  $filename = Get-Filename -target $target
  $URL = Get-Download-Url -Version $Version -Filename $filename
  $temp = (New-TemporaryDirectory)
  $ZipPath = "$temp\$filename"

  $null = mkdir -Force $testBin
  Remove-Item -Force $ZipPath -ErrorAction SilentlyContinue

  # curl.exe is faster than PowerShell 5's 'Invoke-WebRequest'
  # note: 'curl' is an alias to 'Invoke-WebRequest'. so the exe suffix is required
  if (-not $DownloadWithoutCurl) {
    curl.exe "-#SfLo" "$ZipPath" "$URL" 
  }
  if ($DownloadWithoutCurl -or ($LASTEXITCODE -ne 0)) {
    Write-Warning "The command 'curl.exe $URL -o $ZipPath' exited with code $LASTEXITCODE`nTrying an alternative download method..."
    try {
      # Use Invoke-RestMethod instead of Invoke-WebRequest because Invoke-WebRequest breaks on some platform(From bun script not sure why)
      Invoke-RestMethod -Uri $URL -OutFile $ZipPath
    } catch {
      Write-Output "Install Failed - could not download $URL"
      Write-Output "The command 'Invoke-RestMethod $URL -OutFile $ZipPath' exited with code $LASTEXITCODE`n"
      return 1
    }
  }

  if (!(Test-Path $ZipPath)) {
    Write-Output "Install Failed - could not download $URL"
    Write-Output "The file '$ZipPath' does not exist. Did an antivirus delete it?`n"
    return 1
  }

  try {
    $lastProgressPreference = $global:ProgressPreference
    $global:ProgressPreference = 'SilentlyContinue';
    Expand-Archive "$ZipPath" "$testBin" -Force
    $global:ProgressPreference = $lastProgressPreference
    if (!(Test-Path "$testBin\test.exe")) {
      throw "The file '$testBin\test.exe' does not exist. Download is corrupt or intercepted Antivirus?`n"
    }
  } catch {
    Write-Output "Install Failed - could not unzip $ZipPath"
    Write-Error $_
    return 1
  }

  Remove-Item $temp -Recurse -Force

  $testRevision = "$(& "$testBin\test.exe" --version)"
  if ($LASTEXITCODE -eq 1073741795) { # STATUS_ILLEGAL_INSTRUCTION
    Write-Output "Install Failed - test.exe is not compatible with your CPU. This should have been detected before downloading.`n"
    return 1
  }
  
  if ($LASTEXITCODE -ne 0) {
    Write-Output "Install Failed - could not verify test.exe"
    Write-Output "The command '$testBin\test.exe --version' exited with code $LASTEXITCODE`n"
    return 1
  }

  $DisplayVersion = "$(& "$testBin\test.exe" --version)"

  $C_RESET = [char]27 + "[0m"
  $C_GREEN = [char]27 + "[1;32m"

  Write-Output "$C_GREENTestCo $DisplayVersion was installed successfully!$C_RESET"
  Write-Output "The binary is located at $testBin\test.exe`n"

  $hasExistingOther = $false;
  try {
    $existing = Get-Command test -ErrorAction Stop
    if ($existing.Source -ne "$testBin\test.exe") {
      Write-Warning "Note: Another test.exe is already in %PATH% at $($existing.Source)`nTyping 'test' in your terminal will not use what was just installed.`n"
      $hasExistingOther = $true;
    }
  } catch {}

  if (-not $NoRegisterInstallation) {
    $rootKey = $null
    try {
      $RegistryKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\test"  
      $rootKey = New-Item -Path $RegistryKey -Force
      New-ItemProperty -Path $RegistryKey -Name "DisplayName" -Value "TestCo" -PropertyType String -Force | Out-Null
      New-ItemProperty -Path $RegistryKey -Name "InstallLocation" -Value "$testRoot" -PropertyType String -Force | Out-Null
      New-ItemProperty -Path $RegistryKey -Name "DisplayIcon" -Value $testBin\test.exe -PropertyType String -Force | Out-Null
      New-ItemProperty -Path $RegistryKey -Name "UninstallString" -Value "powershell -c `"& `'$testRoot\uninstall.ps1`' -PauseOnError`" -ExecutionPolicy Bypass" -PropertyType String -Force | Out-Null
    } catch {
      if ($rootKey -ne $null) {
        Remove-Item -Path $RegistryKey -Force
      }
    }
  }

  if(!$hasExistingOther) {
    # Only try adding to path if there isn't already a test.exe in the path
    $Path = (Get-Env -Key "Path") -split ';'
    if ($Path -notcontains $testBin) {
      if (-not $NoPathUpdate) {
        $Path += $testBin
        [Environment]::SetEnvironmentVariable('Path', ($Path -join ';'), , [System.EnvironmentVariableTarget]::User)
        $env:PATH = $Path;
      } else {
        Write-Output "Skipping adding '$testBin' to the user's %PATH%`n"
      }
    }

    Write-Output "To get started, restart your terminal/editor, then type `"test`"`n"
  }

  $LASTEXITCODE = 0;
}

Install-test -Version $Version
