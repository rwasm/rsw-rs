mod config;
mod core;
mod utils;

use crate::core::RswErr;

#[macro_use]
extern crate serde_derive;

fn main() {
    if !utils::check_env_cmd("wasm-pack") {
        // TODO: ask if you want to install `wasm-pack` now
        println!("{}", RswErr::EnvErr);
        std::process::exit(1);
    }

    let rsw_config = config::RswConfig::new().unwrap();
    // println!("toml => {:#?}", rsw_config);
    for rsw_crate in &rsw_config.crates {
        core::cli(rsw_crate);
    }
}
