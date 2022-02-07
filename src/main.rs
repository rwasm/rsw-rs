mod config;
mod core;
mod utils;
mod template;

#[macro_use]
extern crate serde_derive;

use crate::core::{Cli, RswErr};

fn main() {
    if !utils::check_env_cmd("wasm-pack") {
        // TODO: ask if you want to install `wasm-pack` now
        println!("{}", RswErr::WasmPack);
        std::process::exit(1);
    }

    Cli::new();
}
