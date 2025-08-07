#![allow(unused_imports)]
use std::env::{self};

use livraison::cli::run_cli;

fn main() {
    match run_cli(env::args_os()) {
        Ok(_) => println!("Command executed successfully."),
        Err(e) => eprintln!("Error: {e}"),
    }
}
