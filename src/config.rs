// wasm-pack build
// https://rustwasm.github.io/wasm-pack/book/commands/build.html

use serde_derive::Deserialize;
use toml;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CreateConfig {
    name: Option<String>,
    out_dir: Option<String>,
    target: Option<String>,
    profile: Option<String>,
    mode: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct RswConfig {
    name: Option<String>,
    version: Option<String>,
    // npm | pnpm
    cli: Option<String>,
    // rust crates
    crates: Vec<Option<CreateConfig>>,
}

#[allow(dead_code)]
pub(crate) fn rsw_config(rsw_file: &'static str) -> RswConfig {
  toml::from_str(rsw_file).unwrap()
}
