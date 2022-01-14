mod cli;
mod config;

#[macro_use]
extern crate serde_derive;

fn main() {
    let rsw_config = config::RswConfig::new().unwrap();
    println!("toml => {:#?}", rsw_config);
    for rsw_crate in &rsw_config.crates {
        cli::new(rsw_crate);
    }
}
