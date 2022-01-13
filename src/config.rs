// wasm-pack build
// https://rustwasm.github.io/wasm-pack/book/commands/build.html
use toml;
use std::{env, fs};
use colored::Colorize;
use anyhow::{Result, Error};
use serde_derive::Deserialize;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct CrateConfig {
    pub name: String,
    pub out_dir: Option<String>,
    pub target: Option<String>,
    pub profile: Option<String>,
    pub mode: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(crate) struct RswConfig {
    pub name: String,
    pub version: String,
    // npm | pnpm
    pub cli: Option<String>,
    // rust crates
    pub crates: Vec<CrateConfig>,
}

impl RswConfig {
	#[allow(dead_code)]
    pub fn new() -> Result<Rc<RswConfig>, Error> {
        let rsw_file = env::current_dir().unwrap().join("rsw.toml");
        let rsw_content = fs::read_to_string(rsw_file).unwrap_or_else(|e| {
			panic!(
				"\n{} {}, {} must exist in the project root path.\n",
				"[⚙️ rsw.toml]".red().on_black(),
				e,
				"rsw.toml".green(),
			);
		});
        let rsw_toml_parse = toml::from_str(&rsw_content).unwrap_or_else(|e| {
			panic!(
				"\n{} {}\n",
				"[⚙️ rsw.toml]".red().on_black(),
				e,
			);
		});

        Ok(Rc::new(rsw_toml_parse))
    }
}
