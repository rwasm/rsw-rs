mod cli;
mod config;

fn main() {
    let rsw_config = config::RswConfig::new().unwrap();
    // println!("version => {}", rsw_config.version);
    for rsw_crate in &rsw_config.crates {
        cli::new(rsw_crate);
    }
}
