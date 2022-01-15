mod cli;
mod watch;
mod build;
mod config;
mod utils;
mod error;

use error::RswErr;

#[macro_use]
extern crate serde_derive;

fn main() {
    if !utils::check_env_cmd("wasm-pack") {
        println!("{}", RswErr::EnvErr);
        std::process::exit(1);
    }

    let rsw_config = config::RswConfig::new().unwrap();
    println!("toml => {:#?}", rsw_config);
    for rsw_crate in &rsw_config.crates {
        cli::new(rsw_crate);
    }
}
