mod config;
mod core;
mod utils;

#[macro_use]
extern crate serde_derive;

use crate::core::{Cli, RswErr};

fn main() {
    if !utils::check_env_cmd("wasm-pack") {
        // TODO: ask if you want to install `wasm-pack` now
        println!("{}", RswErr::EnvErr);
        std::process::exit(1);
    }

    let rsw_config = config::RswConfig::new().unwrap();
    Cli::new(&rsw_config);
    // println!("toml => {:#?}", rsw_config);
}
