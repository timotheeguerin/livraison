use clap::{Args, arg};
use color::cyan;

use crate::{
    LivraisonResult,
    scripts::shell::{ShellScriptOptions, create_shell_script},
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
    /// Example: "https://github.com/foo/bar/releases/{version}/{filename}"
    #[arg(long)]
    pub download_url: String,

    /// If set url that should return what is the latest version which can then be used to download the product
    #[arg(long)]
    pub resolve_latest_version_url: Option<String>,
}

pub fn create_script(args: ScriptArgs) -> LivraisonResult<()> {
    match args.target.as_str() {
        "sh" | "shell" => {
            let script = create_shell_script(ShellScriptOptions {
                name: args.name,
                bin_name: args.bin_name,
                download_url: args.download_url,
                filename: args.filename,
                resolve_latest_version_url: args.resolve_latest_version_url,
                ..Default::default()
            });
            std::fs::write(&args.out, script)?;

            println!("Shell script created at: {}", cyan(args.out));
        }
        _ => {
            panic!("Unsupported target: {}", args.target);
        }
    }
    Ok(())
}
