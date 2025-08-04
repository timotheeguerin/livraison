#![allow(unused_imports)]
use std::env::{self};

use livraison::cli::run_cli;

fn main() {
    run_cli(env::args_os());
}
