use clap::{Args, Parser, Subcommand, arg, command};

/// Simple program to greet a person
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

fn main() {
    let args = AppArgs::parse();

    match args.command {
        Command::Pack(pack_args) => {
            println!("Packing target: {}", pack_args.target);
        }
    }
}
