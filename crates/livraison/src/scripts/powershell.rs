use indoc::{formatdoc, indoc};

use printer::{Doc, group, hardline, indent, join, text};

#[derive(Debug, Clone)]
pub struct PlatformMapping {
    /// Platform identifier as returned by Windows system info
    pub platform_id: String,
    /// Target name to use for this platform
    pub target: String,
}

#[derive(Debug)]
pub struct PlatformConfig {
    /// List of platform mappings to include
    pub mappings: Vec<PlatformMapping>,
    /// Default target to use if no platform matches (optional)
    pub default_target: Option<String>,
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self {
            mappings: vec![
                PlatformMapping {
                    platform_id: "x64-based".to_string(),
                    target: "windows-x64".to_string(),
                },
                PlatformMapping {
                    platform_id: "ARM64-based".to_string(),
                    target: "windows-arm64".to_string(),
                },
            ],
            default_target: Some("windows-x64".to_string()),
        }
    }
}

#[derive(Default, Debug)]
pub struct PowerShellScriptOptions {
    /// Product friendly name. Also the binary name if not provided
    pub name: String,

    /// Name of the binary
    pub bin_name: Option<String>,

    /// Filename template. Interpolate the following variables:
    /// - `{version}`: The version of the binary to download
    /// - `{target}`: The target platform for the binary
    /// - `{bin-name}`: The name of the binary
    ///
    /// Default to `{bin_name}-{target}.zip`
    pub filename: Option<String>,

    /// URL template for downloading the binary. Interpolate the following variables:
    /// - `{version}`: The version of the binary to download
    /// - `{filename}`: The filename. [filename]
    /// - `{target}`: The target platform for the binary
    /// - `{bin-name}`: The name of the binary
    ///
    /// Example: "https://github.com/foo/bar/releases/{version}/{filename}"
    pub download_url: String,

    /// URL template for downloading the latest version of the binary
    /// Default to the download_url with {version} set to 'latest'
    pub latest_download_url: Option<String>,

    pub resolve_latest_version_url: Option<String>,
    pub platform_config: PlatformConfig,
}

impl PowerShellScriptOptions {
    pub fn get_bin_name(&self) -> &str {
        self.bin_name.as_deref().unwrap_or(&self.name)
    }

    pub fn get_filename(&self) -> String {
        self.filename
            .clone()
            .unwrap_or_else(|| format!("{}-{}.zip", self.get_bin_name(), "$target"))
    }
}

pub fn create_powershell_script(options: &PowerShellScriptOptions) -> String {
    let header = text(formatdoc! {r#"
        #!/usr/bin/env pwsh
        param(
          [String]$Version = "latest",
          # Skips adding the {bin_name}.exe directory to the user's %PATH%
          [Switch]$NoPathUpdate = $false,
          # Skips adding the {bin_name}.exe to the list of installed programs
          [Switch]$NoRegisterInstallation = $false,

          # Debugging: Always download with 'Invoke-RestMethod' instead of 'curl.exe'
          [Switch]$DownloadWithoutCurl = $false
        );
    "#,
        bin_name = options.get_bin_name()
    });

    let mut script = vec![header, hardline, create_utility_functions(), hardline];

    if let Some(url) = &options.resolve_latest_version_url {
        script.push(find_latest_version_fn(url));
        script.push(hardline);
    }

    script.push(join(
        vec![
            get_download_url_fn(options),
            target_fn(options),
            get_filename_fn(options),
            install_function(options),
            main_execution(options),
        ],
        hardline,
    ));

    group(script).serialize()
}

fn create_utility_functions() -> Doc {
    text(indoc! {r#"
        function New-TemporaryDirectory {
            $tmp = [System.IO.Path]::GetTempPath() # Not $env:TEMP, see https://stackoverflow.com/a/946017
            $name = (New-Guid).ToString("N")
            New-Item -ItemType Directory -Path (Join-Path $tmp $name)
        }
    "#})
}

fn find_latest_version_fn(latest_version_url: &str) -> Doc {
    text(formatdoc! {r#"
        function Find-Latest-Version {{
            return (Invoke-webrequest -UseBasicParsing -URI "{latest_version_url}").Content.Trim()
        }}
    "#})
}

fn get_download_url_fn(options: &PowerShellScriptOptions) -> Doc {
    let interpolated_url = interpolate_url(&options.download_url, options);

    let body = match &options.latest_download_url {
        Some(url) => {
            let latest_url = interpolate_url(url, options);
            formatdoc! {r#"
            if($Version -eq "latest") {{
                return "{latest_url}"
            }} else {{
                return "{interpolated_url}"
            }}
        "#}
        }
        None => match options.resolve_latest_version_url {
            Some(_) => formatdoc! {r#"
            if($Version -eq "latest") {{
                $Version = (Find-Latest-Version)
            }}

            return "{interpolated_url}"
        "#},
            None => formatdoc! {r#"
            return "{interpolated_url}"
        "#},
        },
    };

    text(formatdoc! {r#"
        function Get-Download-Url {{
            param(
              [String] $Version, 
              [String] $filename
            )
            
            {body}
        }}
    "#})
}

fn get_filename_fn(options: &PowerShellScriptOptions) -> Doc {
    let filename = options
        .filename
        .clone()
        .unwrap_or("{bin_name}-$target.zip".to_string())
        .replace("{version}", "$Version")
        .replace("{bin_name}", options.get_bin_name())
        .replace("{target}", "$target");

    text(formatdoc! {r#"
        function Get-Filename {{
            param([String] $target)

            return "{filename}"
        }}
    "#})
}

fn interpolate_url(template: &str, options: &PowerShellScriptOptions) -> String {
    template
        .replace("{version}", "$Version")
        .replace("{bin_name}", options.get_bin_name())
        .replace("{filename}", "$(Get-Filename -target $target)")
        .replace("{target}", "$target")
}

fn target_fn(options: &PowerShellScriptOptions) -> Doc {
    let mut platform_cases: Vec<Doc> = Vec::new();
    let mut first = true;
    for mapping in &options.platform_config.mappings {
        if !first {
            platform_cases.push(text("else"));
        }
        first = false;
        platform_cases.push(text(format!(
            "if ((Get-CimInstance Win32_ComputerSystem).SystemType -match \"{}\")",
            mapping.platform_id
        )));
        platform_cases.push(group(vec![
            hardline,
            block(text(format!("return \"{}\"", mapping.target))),
        ]))
    }

    let default = options
        .platform_config
        .default_target
        .clone()
        .unwrap_or("windows-x64".to_string());
    if options.platform_config.mappings.is_empty() {
        platform_cases.push(text(format!(r#"return "{default}""#)));
    } else {
        platform_cases.push(group(vec![
            text("else"),
            hardline,
            block(text(format!(r#"return "{default}""#))),
        ]));
    }

    make_fn(
        "Get-Target",
        group(vec![
            text("# Detect the target platform based on system info"),
            hardline,
            group(platform_cases),
        ]),
    )
}

fn make_fn(name: &str, body: impl Into<Doc>) -> Doc {
    group(vec![text("function "), text(name), hardline, block(body)])
}

fn block(body: impl Into<Doc>) -> Doc {
    group(vec![
        text("{"),
        hardline,
        indent(body),
        hardline,
        text("}"),
        hardline,
    ])
}

fn install_function(options: &PowerShellScriptOptions) -> Doc {
    let bin_name = options.get_bin_name();
    let name = &options.name;

    let bin_name_upper = bin_name.to_uppercase();

    // Build the PowerShell script content with proper escaping
    let script_content = formatdoc!(
        r#"
        function Install-{bin_name} {{
          param([string]$Version);

          $target = Get-Target

          ${bin_name}Root = if ($env:{bin_name_upper}_INSTALL) {{ $env:{bin_name_upper}_INSTALL }} else {{ "$HOME\.{bin_name}" }}
          ${bin_name}Bin = mkdir -Force "${bin_name}Root\bin"

          try {{
            Remove-Item "${bin_name}Bin\{bin_name}.exe" -Force
          }} catch [System.Management.Automation.ItemNotFoundException] {{
            # ignore
          }} catch [System.UnauthorizedAccessException] {{
            $openProcesses = Get-Process -Name {bin_name} | Where-Object {{ $_.Path -eq "${bin_name}Bin\{bin_name}.exe" }}
            if ($openProcesses.Count -gt 0) {{
              Write-Output "Install Failed - An older installation exists and is open. Please close open {bin_name} processes and try again."
              return 1
            }}
            Write-Output "Install Failed - An unknown error occurred while trying to remove the existing installation"
            Write-Output $_
            return 1
          }} catch {{
            Write-Output "Install Failed - An unknown error occurred while trying to remove the existing installation"
            Write-Output $_
            return 1
          }}

          $filename = Get-Filename -target $target
          $URL = Get-Download-Url -Version $Version -Filename $filename
          $temp = (New-TemporaryDirectory)
          $ZipPath = "$temp\$filename"

          $null = mkdir -Force ${bin_name}Bin
          Remove-Item -Force $ZipPath -ErrorAction SilentlyContinue

          # curl.exe is faster than PowerShell 5's 'Invoke-WebRequest'
          # note: 'curl' is an alias to 'Invoke-WebRequest'. so the exe suffix is required
          if (-not $DownloadWithoutCurl) {{
            curl.exe "-#SfLo" "$ZipPath" "$URL" 
          }}
          if ($DownloadWithoutCurl -or ($LASTEXITCODE -ne 0)) {{
            Write-Warning "The command 'curl.exe $URL -o $ZipPath' exited with code $LASTEXITCODE`nTrying an alternative download method..."
            try {{
              # Use Invoke-RestMethod instead of Invoke-WebRequest because Invoke-WebRequest breaks on some platform(From bun script not sure why)
              Invoke-RestMethod -Uri $URL -OutFile $ZipPath
            }} catch {{
              Write-Output "Install Failed - could not download $URL"
              Write-Output "The command 'Invoke-RestMethod $URL -OutFile $ZipPath' exited with code $LASTEXITCODE`n"
              return 1
            }}
          }}

          if (!(Test-Path $ZipPath)) {{
            Write-Output "Install Failed - could not download $URL"
            Write-Output "The file '$ZipPath' does not exist. Did an antivirus delete it?`n"
            return 1
          }}

          try {{
            $lastProgressPreference = $global:ProgressPreference
            $global:ProgressPreference = 'SilentlyContinue';
            Expand-Archive "$ZipPath" "${bin_name}Bin" -Force
            $global:ProgressPreference = $lastProgressPreference
            if (!(Test-Path "${bin_name}Bin\{bin_name}.exe")) {{
              throw "The file '${bin_name}Bin\{bin_name}.exe' does not exist. Download is corrupt or intercepted Antivirus?`n"
            }}
          }} catch {{
            Write-Output "Install Failed - could not unzip $ZipPath"
            Write-Error $_
            return 1
          }}

          Remove-Item $temp -Recurse -Force

          ${bin_name}Revision = "$(& "${bin_name}Bin\{bin_name}.exe" --version)"
          if ($LASTEXITCODE -eq 1073741795) {{ # STATUS_ILLEGAL_INSTRUCTION
            Write-Output "Install Failed - {bin_name}.exe is not compatible with your CPU. This should have been detected before downloading.`n"
            return 1
          }}
          
          if ($LASTEXITCODE -ne 0) {{
            Write-Output "Install Failed - could not verify {bin_name}.exe"
            Write-Output "The command '${bin_name}Bin\{bin_name}.exe --version' exited with code $LASTEXITCODE`n"
            return 1
          }}

          $DisplayVersion = "$(& "${bin_name}Bin\{bin_name}.exe" --version)"

          $C_RESET = [char]27 + "[0m"
          $C_GREEN = [char]27 + "[1;32m"

          Write-Output "$C_GREEN{name} $DisplayVersion was installed successfully!$C_RESET"
          Write-Output "The binary is located at ${bin_name}Bin\{bin_name}.exe`n"

          $hasExistingOther = $false;
          try {{
            $existing = Get-Command {bin_name} -ErrorAction Stop
            if ($existing.Source -ne "${bin_name}Bin\{bin_name}.exe") {{
              Write-Warning "Note: Another {bin_name}.exe is already in %PATH% at $($existing.Source)`nTyping '{bin_name}' in your terminal will not use what was just installed.`n"
              $hasExistingOther = $true;
            }}
          }} catch {{}}

          if (-not $NoRegisterInstallation) {{
            $rootKey = $null
            try {{
              $RegistryKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\{bin_name}"  
              $rootKey = New-Item -Path $RegistryKey -Force
              New-ItemProperty -Path $RegistryKey -Name "DisplayName" -Value "{name}" -PropertyType String -Force | Out-Null
              New-ItemProperty -Path $RegistryKey -Name "InstallLocation" -Value "${bin_name}Root" -PropertyType String -Force | Out-Null
              New-ItemProperty -Path $RegistryKey -Name "DisplayIcon" -Value ${bin_name}Bin\{bin_name}.exe -PropertyType String -Force | Out-Null
              New-ItemProperty -Path $RegistryKey -Name "UninstallString" -Value "powershell -c `"& `'${bin_name}Root\uninstall.ps1`' -PauseOnError`" -ExecutionPolicy Bypass" -PropertyType String -Force | Out-Null
            }} catch {{
              if ($rootKey -ne $null) {{
                Remove-Item -Path $RegistryKey -Force
              }}
            }}
          }}

          if(!$hasExistingOther) {{
            # Only try adding to path if there isn't already a {bin_name}.exe in the path
            $Path = (Get-Env -Key "Path") -split ';'
            if ($Path -notcontains ${bin_name}Bin) {{
              if (-not $NoPathUpdate) {{
                $Path += ${bin_name}Bin
                [Environment]::SetEnvironmentVariable('Path', ($Path -join ';'), [System.EnvironmentVariableTarget]::User)
                $env:PATH = $Path;
              }} else {{
                Write-Output "Skipping adding '${bin_name}Bin' to the user's %PATH%`n"
              }}
            }}

            Write-Output "To get started, restart your terminal/editor, then type `"{bin_name}`"`n"
          }}

          $LASTEXITCODE = 0;
        }}
    "#,
        bin_name = bin_name,
        bin_name_upper = bin_name_upper,
        name = name,
    );

    text(script_content)
}

fn main_execution(options: &PowerShellScriptOptions) -> Doc {
    text(formatdoc! {r#"
        Install-{bin_name} -Version $Version
    "#,
        bin_name = options.get_bin_name()
    })
}

#[cfg(test)]
mod tests {
    use crate::scripts::powershell::{PowerShellScriptOptions, create_powershell_script};

    #[test]
    fn test_create_basic() {
        let script = create_powershell_script(&PowerShellScriptOptions {
            name: "TestCo".to_string(),
            bin_name: Some("test".to_string()),
            download_url: "https://example.com/{version}/{filename}".to_string(),
            ..Default::default()
        });

        insta::assert_binary_snapshot!("basic.ps1", script.into());
    }
}
