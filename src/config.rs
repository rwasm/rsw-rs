//! rsw.toml parse
//!
//! [wasm-pack build](https://rustwasm.github.io/wasm-pack/book/commands/build.html)

use std::{env, fs, process};
use anyhow::{Result, Error};
use crate::error::RswErr;

#[derive(Debug, Serialize, Deserialize)]
// @see https://serde.rs/container-attrs.html#rename_all
#[serde(rename_all = "kebab-case")]
pub(crate) struct CrateConfig {
    /// npm package
    pub name: String,
    /// TODO
    #[serde(default = "default_out_dir")]
    pub out_dir: Option<String>,
    /// TODO
    #[serde(default = "default_true")]
    watch: Option<bool>,
    /// TODO
    #[serde(default = "default_true")]
    build: Option<bool>,
    /// TODO
    ///
    /// <https://rustwasm.github.io/wasm-pack/book/commands/build.html#target>
    ///
    /// target: bundler | nodejs | web | no-modules
    ///
    pub target: Option<String>,
    /// TODO
    pub mode: Option<String>,
    /// <https://rustwasm.github.io/wasm-pack/book/commands/build.html#profile>
    ///
    /// profile: profiling | release
    ///
    /// When in `watch` mode, the value of `profile` is `dev`,
    /// building this way is faster but applies few optimizations to the output,
    /// and enables debug assertions and other runtime correctness checks.
    #[serde(default = "default_profile")]
    pub profile: Option<String>,
}

fn default_out_dir() -> Option<String> {
    Some("./pkg".to_string())
}
fn default_profile() -> Option<String> {
    Some("release".to_string())
}

fn default_true() -> Option<bool> {
    Some(true)
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RswConfig {
    /// rsw name
    pub name: Option<String>,
    /// rsw version
    pub version: Option<String>,
    /// npm | yarn | pnpm
    pub cli: Option<String>,
    /// rust crates
    #[serde(default)]
    pub crates: Vec<CrateConfig>,
}

impl Default for RswConfig {
    fn default() -> Self {
        Self {
            name: Some("rsw".to_string()),
            version: Some("0.0.0".to_string()),
            cli: Some("npm".to_string()),
            crates: vec![],
        }
    }
}

impl RswConfig {
    pub fn new() -> Result<RswConfig, Error> {
        let rsw_file = env::current_dir().unwrap().join("rsw.toml");
        let rsw_content = fs::read_to_string(rsw_file).unwrap_or_else(|e| {
            println!("{}", RswErr::FileErr(e));
            process::exit(1);
        });
        let rsw_toml_parse = toml::from_str(&rsw_content).unwrap_or_else(|e| {
            println!("{}", RswErr::ParseErr(e));
            process::exit(1);
        });

        Ok(rsw_toml_parse)
    }
}
