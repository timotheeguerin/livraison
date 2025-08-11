use std::{
    env::{self},
    ffi::OsString,
    fmt::Debug,
    path::PathBuf,
};

use clap::{Args, Parser, Subcommand, arg, command};

use crate::{
    LivraisonResult,
    actions::{
        pack::{CommonOptions, pack_for_target},
        script::{ScriptArgs, create_script},
    },
    common::DataFile,
};
// use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct AppArgs {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Args)]
struct GlobalOpts {
    /// Enable debug mode
    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Create a new package
    Pack(PackArgs),
    /// Create an installer script
    Script(ScriptArgs),
}

#[derive(Debug, Args)]
struct PackArgs {
    /// Target
    #[arg(short, long)]
    target: String,

    /// Name of the bundle
    #[arg(short, long)]
    name: String,

    /// Product version
    #[arg(long)]
    version: Option<String>,

    /// Product description
    #[arg(long)]
    description: Option<String>,

    /// Output file path
    #[arg(short, long)]
    out: Option<String>,

    /// Binary files
    #[arg(long)]
    bin_file: Vec<String>,
}

pub fn run_cli<I, T>(args: I) -> LivraisonResult<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
    I: std::fmt::Debug,
{
    let cwd = env::current_dir()?;
    let args = AppArgs::parse_from(args);
    match args.command {
        Command::Pack(pack_args) => pack_for_target(
            pack_args.target,
            CommonOptions {
                name: pack_args.name,
                version: pack_args.version,
                bin_files: pack_args
                    .bin_file
                    .iter()
                    .map(DataFile::from_local)
                    .collect(),
                out: match pack_args.out {
                    Some(out) => PathBuf::from(out),
                    None => cwd.join("dist"),
                },
                ..Default::default()
            },
        )?,
        Command::Script(args) => create_script(args)?,
    }

    Ok(())
}
