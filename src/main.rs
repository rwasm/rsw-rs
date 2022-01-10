mod config;

fn main() {
  let rsw_toml = config::rsw_toml_parse().unwrap();
  // println!("{:#?}", rsw_toml);
  // println!("name => {}", rsw_toml.name.as_ref().unwrap());

  for rsw_crate in rsw_toml.crates {
    let crate_config = rsw_crate.as_ref().unwrap();
    println!("crate name => {:?}", crate_config.name.as_ref().unwrap());
  }
}
