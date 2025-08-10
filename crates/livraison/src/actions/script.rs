use clap::{Args, arg};
use color::{bold, cyan, green};

use crate::{
    LivraisonResult,
    scripts::{
        powershell::{PowerShellScriptOptions, create_powershell_script},
        shell::{ShellScriptOptions, create_shell_script},
    },
};

#[derive(Debug, Args)]
pub struct ScriptArgs {
    /// Target
    #[arg(short, long)]
    pub target: String,

    #[arg(short, long)]
    pub out: String,

    /// Name of the product. Bin name default to this
    #[arg(short, long)]
    pub name: String,

    /// Name of binary
    #[arg(long)]
    pub bin_name: Option<String>,

    /// Filename template. Interpolate the following variables:
    /// - `{version}`: The version of the binary to download
    /// - `{target}`: The target platform for the binary
    /// - `{bin-name}`: The name of the binary
    ///
    /// Default to
    /// - Unix Shell: `{bin_name}-{target}.tar.gz`
    /// - Windows: `{bin_name}-{target}.zip`
    #[arg(long)]
    pub filename: Option<String>,

    /// URL template for downloading the binary. Interpolate the following variables:
    /// - `{version}`: The version of the binary to download
    /// - `{filename}`: The filename. [filename]
    /// - `{target}`: The target platform for the binary
    /// - `{bin-name}`: The name of the binary
    ///
    /// Example: "https://github.com/foo/bar/releases/download/{version}/{filename}"
    #[arg(long)]
    pub download_url: String,
    /// URL template for downloading the latest binary. Default to the download_url with 'latest' as the {version}
    /// - `{filename}`: The filename. [filename]
    /// - `{target}`: The target platform for the binary
    /// - `{bin-name}`: The name of the binary
    ///
    /// Example: "https://github.com/foo/bar/releases/latest/download/{filename}"
    #[arg(long)]
    pub latest_download_url: Option<String>,

    /// If set url that should return what is the latest version which can then be used to download the product
    #[arg(long)]
    pub resolve_latest_version_url: Option<String>,
}

pub fn create_script(args: ScriptArgs) -> LivraisonResult<()> {
    match args.target.as_str() {
        "sh" | "shell" => {
            let options = ShellScriptOptions {
                name: args.name,
                bin_name: args.bin_name,
                download_url: args.download_url,
                filename: args.filename,
                resolve_latest_version_url: args.resolve_latest_version_url,
                latest_download_url: args.latest_download_url,
                ..Default::default()
            };
            let script = create_shell_script(&options);
            std::fs::write(&args.out, script)?;

            println!("Shell script created at: {}", cyan(args.out));
            print_kv("Name", &options.name);
            print_kv("Bin Name", options.get_bin_name());
            print_kv("Download URL", &options.download_url);
            if let Some(url) = &options.latest_download_url {
                print_kv("Latest Download URL", url);
            }
            print_kv("Filename", &options.get_filename());
        }
        "pwsh" | "powershell" => {
            let options = PowerShellScriptOptions {
                name: args.name,
                bin_name: args.bin_name,
                download_url: args.download_url,
                filename: args.filename,
                resolve_latest_version_url: args.resolve_latest_version_url,
                latest_download_url: args.latest_download_url,
                ..Default::default()
            };
            let script = create_powershell_script(&options);
            std::fs::write(&args.out, script)?;

            println!("PowerShell script created at: {}", cyan(args.out));
            print_kv("Name", &options.name);
            print_kv("Bin Name", options.get_bin_name());
            print_kv("Download URL", &options.download_url);
            if let Some(url) = &options.latest_download_url {
                print_kv("Latest Download URL", url);
            }
            print_kv("Filename", &options.get_filename());
        }
        _ => {
            panic!("Unsupported target: {}", args.target);
        }
    }
    Ok(())
}

fn print_kv(key: &str, value: &str) {
    println!("  {}: {}", bold(key), green(value));
}
