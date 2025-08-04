use std::{ffi::OsString, fmt::Debug};

use clap::{Args, Parser, Subcommand, arg, command};
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
}

#[derive(Debug, Args)]
struct PackArgs {
    /// Target
    #[arg(short, long)]
    target: String,
}

pub fn run_cli<I, T>(args: I)
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
    I: std::fmt::Debug,
{
    let args = AppArgs::parse_from(args);

    match args.command {
        Command::Pack(pack_args) => {
            println!("Packing target: {}", pack_args.target);
        }
    }
}
