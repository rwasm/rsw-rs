//! Usage
//!
//! ```bash
//! #! help
//! rsw -h
//!
//! #! new help
//! rsw new -h
//!
//! #! dev
//! rsw watch
//!
//! #! release
//! rsw build
//!
//! #! generate a project quickly
//! rsw new <name>
//! ```

//! ----------

//! rsw.toml
//!
//! ```toml
//! name = "rsw"
//! version = "0.1.0"
//! #! default is `50` ms
//! interval = 50
//!
//! #! ---------------------------
//!
//! #! rsw new <name>
//! [new]
//! #! @see https://rustwasm.github.io/docs/wasm-pack/commands/new.html
//! #! using: `wasm-pack` | `rsw` | `user`, default is `wasm-pack`
//! #! 1. wasm-pack: `rsw new <name> --template <template> --mode <normal|noinstall|force>`
//! #! 2. rsw: `rsw new <name>`, built-in templates
//! #! 3. user: `rsw new <name>`, if `dir` is not configured, use `wasm-pack new <name>` to initialize the project
//! using = "wasm-pack"
//! #! this field needs to be configured when `using = "user"`
//! #! `using = "wasm-pack"` or `using = "rsw"`, this field will be ignored
//! #! copy all files in this directory
//! dir = "my-template"
//!
//! #! ################# NPM Package #################
//!
//! #! When there is only `name`, other fields will use the default configuration
//! #! -------- package: rsw-hello --------
//! [[crates]]
//! name = "rsw-hello"
//!
//! #! =======================================================
//!
//! #! -------- package: @rsw/hello --------
//! # [[crates]]
//! # #! npm package name
//! # name = "@rsw/hello"
//! # #! default is `.`
//! # root = "."
//! # #! default is `pkg`
//! # out-dir = "pkg"
//! # #! target: bundler | nodejs | web | no-modules, default is `web`
//! # target = "web"
//! # #! rsw watch
//! # [crates.watch]
//! # #! default is `true`
//! # run = false
//! # #! profile: `dev` | `profiling`, default is `dev`
//! # profile = "dev"
//! # #! rsw build
//! # [crates.build]
//! # run = false
//! # #! profile: `release` | `profiling`, default is `release`
//! # profile = "release"
//! ```

mod config;
mod core;
mod template;
mod utils;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use crate::core::{Cli, RswErr};

fn main() {
    utils::init_logger();

    if !utils::check_env_cmd("wasm-pack") {
        // TODO: ask if you want to install `wasm-pack` now
        println!("{}", RswErr::WasmPack);
        std::process::exit(1);
    }

    Cli::new();
}
