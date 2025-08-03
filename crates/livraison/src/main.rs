#![allow(unused_imports)]
use std::env::{self};

use livraison::cli::run_cli;

fn main() {
    handle();
}

#[cfg(not(target_arch = "wasm32"))]
fn handle() {
    run_cli(env::args_os());
}

#[cfg(target_arch = "wasm32")]
fn handle() {
    // noop
}
