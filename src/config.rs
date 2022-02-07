//! rsw.toml parse
//!
//! [wasm-pack build](https://rustwasm.github.io/docs/wasm-pack/commands/build.html)

use anyhow::{Error, Result};
use std::{env, fs, process};

use crate::core::RswErr;

pub static RSW_FILE: &'static str = "rsw.toml";

#[derive(Debug, Serialize, Deserialize)]
// @see https://serde.rs/container-attrs.html#rename_all
#[serde(rename_all = "kebab-case")]
pub struct CrateConfig {
    /// <https://docs.npmjs.com/cli/v8/configuring-npm/package-json#name>
    ///
    /// Your package's name, and must be lowercase and one word,
    /// and may contain hyphens and underscores, support `scope`.
    /// For example: `rsw-foo`, `@rsw/foo`
    pub name: String,
    #[serde(default = "default_root")]
    /// crate root path, default is `.`
    pub root: Option<String>,
    /// <https://rustwasm.github.io/wasm-pack/book/commands/build.html#output-directory>
    ///
    /// By default, wasm-pack will generate a directory for it's build output called `pkg`.
    /// You can use `out-dir` to customize the directory where files are generated.
    #[serde(default = "default_out_dir")]
    pub out_dir: Option<String>,
    #[serde(default = "default_watch")]
    pub watch: Option<WatchOptions>,
    #[serde(default = "default_build")]
    pub build: Option<BuildOptions>,
    /// target: bundler | nodejs | web | no-modules
    ///
    /// <https://rustwasm.github.io/wasm-pack/book/commands/build.html#target>
    /// TODO
    pub target: Option<String>,
    /// TODO
    pub mode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WatchOptions {
    /// When executing the command `rsw watch`, whether to include this `crate`.
    #[serde(default = "default_true")]
    pub run: Option<bool>,
    /// profile: dev | profiling
    ///
    /// <https://rustwasm.github.io/wasm-pack/book/commands/build.html#profile>
    ///
    /// When in `watch` mode, the value of `profile` is `dev`,
    /// building this way is faster but applies few optimizations to the output,
    /// and enables debug assertions and other runtime correctness checks.
    /// The `--profiling` and `--release` profiles use cargo's release profile,
    /// but the former enables debug info as well,
    /// which helps when investigating performance issues in a profiler.
    #[serde(default = "default_dev")]
    pub profile: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BuildOptions {
    /// When executing the command `rsw build`, whether to include this `crate`.
    #[serde(default = "default_true")]
    pub run: Option<bool>,
    /// profile: release | profiling
    ///
    /// <https://rustwasm.github.io/wasm-pack/book/commands/build.html#profile>
    ///
    /// The `--profiling` and `--release` profiles use cargo's release profile,
    /// but the former enables debug info as well,
    /// which helps when investigating performance issues in a profiler.
    #[serde(default = "default_release")]
    pub profile: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RswConfig {
    /// rsw name
    pub name: Option<String>,
    /// rsw version
    pub version: Option<String>,
    /// npm | yarn | pnpm
    pub cli: Option<String>,
    /// In `watch` mode, the time interval for `wasm-pack build`, in milliseconds.
    pub interval: Option<u64>,
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
            interval: Some(50),
            crates: vec![],
        }
    }
}

impl RswConfig {
    pub fn new() -> Result<RswConfig, Error> {
        let rsw_file = env::current_dir().unwrap().join(RSW_FILE);
        let rsw_content = fs::read_to_string(rsw_file).unwrap_or_else(|e| {
            println!("{}", RswErr::Config(e));
            process::exit(1);
        });
        let rsw_toml_parse = toml::from_str(&rsw_content).unwrap_or_else(|e| {
            println!("{}", RswErr::ParseToml(e));
            process::exit(1);
        });

        Ok(rsw_toml_parse)
    }
}

fn default_root() -> Option<String> {
    Some(".".to_string())
}

fn default_out_dir() -> Option<String> {
    Some("pkg".to_string())
}

fn default_release() -> Option<String> {
    Some("release".to_string())
}

fn default_dev() -> Option<String> {
    Some("dev".to_string())
}

fn default_true() -> Option<bool> {
    Some(true)
}

fn default_watch() -> Option<WatchOptions> {
    Some(WatchOptions {
        run: default_true(),
        profile: default_dev(),
    })
}

fn default_build() -> Option<BuildOptions> {
    Some(BuildOptions {
        run: default_true(),
        profile: default_release(),
    })
}
