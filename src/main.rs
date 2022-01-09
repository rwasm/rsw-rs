use std::include_str;
mod config;

fn main() {
    let rsw = config::rsw_config(include_str!("../rsw.toml"));

    println!("{:#?}", rsw);
}