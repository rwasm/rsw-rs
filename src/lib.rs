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
//!
//! #! clean - link & build
//! rsw clean
//! ```
//!
//! ----------
//!
//! rsw.toml
//!
//! <https://github.com/lencx/rsw-rs/blob/main/src/template/rsw.toml>
//!
//! ```toml
//! name = "rsw"
//! version = "0.1.0"
//!
//! #! time interval for file changes to trigger wasm-pack build, default `50` milliseconds
//! interval = 50
//!
//! #! link
//! #! npm link @see https://docs.npmjs.com/cli/v8/commands/npm-link
//! #! yarn link @see https://classic.yarnpkg.com/en/docs/cli/link
//! #! pnpm link @see https://pnpm.io/cli/link
//! #! The link command will only be executed if `[[crates]] link = true`
//! #! cli: `npm` | `yarn` | `pnpm`, default is `npm`
//! cli = "npm"
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
//! #! npm package name
//! name = "rsw-hello"
//! #! run `npm link`: `true` | `false`, default is `false`
//! link = false
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
//! #! run `npm link`: `true` | `false`, default is `false`
//! # link = false
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

pub mod config;
pub mod core;
pub mod template;
pub mod utils;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use crate::core::RswErr;
use crate::utils::print;

pub use crate::core::Cli;

pub fn rsw_cli() {
    utils::init_logger();

    if !utils::check_env_cmd("wasm-pack") {
        // TODO: ask if you want to install `wasm-pack` now
        print(RswErr::WasmPack);
        std::process::exit(1);
    }

    Cli::init();
}
