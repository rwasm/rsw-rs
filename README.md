<p align="center">
  <img src="./rsw.png" width="120">
</p>

「🌐 Language」[简体中文](./README.zh_CN.md)

**`rsw = rs(rust) → w(wasm)`** - A command-line tool for automatically rebuilding local changes, based on the `wasm-pack` implementation.

## rsw-rs

### Feature

- rsw init
- rsw watch
- rsw build
- rsw new
  - `wasm-pack`
  - `rsw`
  - `user`
- debug info

### TODO

- local sub-dependency file changes trigger hot updates
- integrate front-end scaffolding, such as `vite`, `webpack`, etc.

## Pre-installed

- [rust](https://www.rust-lang.org/learn/get-started)
- [nodejs](https://nodejs.org)
- [wasm-pack](https://github.com/rustwasm/wasm-pack)

## Usage

```bash
# Rust
cargo install rsw

# Or Node.js
npm install -g @rsw/cli
```

```bash
# help
rsw -h

# rsw.toml - initial configuration
rsw init

# generate a wasm project
rsw new <name>

# dev mode
rsw watch

# release mode
rsw build

# clean - link & build
rsw clean
```

## Awesome rsw

- [vite-plugin-rsw](https://github.com/lencx/vite-plugin-rsw) - 🦀 wasm-pack plugin for Vite
- [create-mpl](https://github.com/lencx/create-mpl) - ⚡️ Create a project in seconds!
- [learn-wasm](https://github.com/lencx/learn-wasm) - 🎲 Learning WebAssembly

## Logger

```bash
# @see: https://github.com/env-logger-rs/env_logger
# RUST_LOG=rsw=<info|trace|debug|error|warn> rsw <watch|build|new>
# 1. info
RUST_LOG=rsw=info rsw <SUBCOMMAND>

# 2. all: info, trace, debug, error, warn
RUST_LOG=rsw rsw <SUBCOMMAND>
```

## rsw.toml

> configuration file

- [TOML Doc](https://toml.io/en/)
- [`wasm-pack build` Doc](https://rustwasm.github.io/docs/wasm-pack/commands/build.html)

### Options

Create `rsw.toml` in the project root path, configure the `rust crate` parameter, and run the `rsw watch` or `rsw build` command.

- **`name`** - Profile name (optional)
- **`version`** - Profile version (optional)
- **`interval`** - Development mode `rsw watch`, time interval for file changes to trigger `wasm-pack build`, default `50` milliseconds
- **`cli`** - `npm` | `yarn` | `pnpm`, default is `npm`. Execute `link` using the specified `cli`, e.g. `npm link`
- **`[new]`** - Quickly generate a crate with `wasm-pack new`, or set a custom template in `rsw.toml -> [new] -> using`
  - **`using`** - `wasm-pack` | `rsw` | `user`, default is `wasm-pack`
    - `wasm-pack` - `rsw new <name> --template <template> --mode <normal|noinstall|force>` [wasm-pack new doc](https://rustwasm.github.io/docs/wasm-pack/commands/new.html)
    - `rsw` - `rsw new <name>`, built-in templates
    - `user` - `rsw new <name>`, if `dir` is not configured, use `wasm-pack new <name>` to initialize the project.
  - **`dir`** - Copy all files in this directory. This field needs to be configured when `using = "user"`. `using = "wasm-pack"` or `using = "rsw"`, this field will be ignored
- **`[[crates]]`** - Is an array that supports multiple `rust crate` configurations
  - **`name`** - npm package name, supporting organization, e.g. `@rsw/foo`
  - **`root`** - Relative to the project root path, default is `.`
  - **`link`** - `true` | `false`，default is `false`, Whether to execute the `link` command after this `rust crate` is built
  - **`target`** - `bundler` | `nodejs` | `web` | `no-modules`, default is `web`
  - **`out-dir`** - npm package output path, default `pkg`
  - **`[crates.watch]`** - Development mode
    - **`run`** - Whether this `crate` needs to be watching, default is `true`
    - **`profile`** - `dev` | `profiling`, default is `dev`
  - **`[crates.build]`** - Production mode
    - **`run`** - Whether this `crate` needs to be build, default is `true`
    - **`profile`** - `release` | `profiling`, default is `release`

**Note: `name` in `[[crates]]` is required, other fields are optional.**

## .rsw

> `rsw watch` - temp dir

- rsw.info - information about the `watch` mode
  - `[RSW::OK]`
  - `[RSW::NAME]`
  - `[RSW::FILE]`
  - `[RSW::ERR]`
  - `[RSW::ARGS]`
- rsw.err - `wasm-pack build` error

### Example

```toml
# rsw.toml

name = "rsw"
version = "0.1.0"

#! time interval for file changes to trigger wasm-pack build, default `50` milliseconds
interval = 50

#! link
#! npm link @see https://docs.npmjs.com/cli/v8/commands/npm-link
#! yarn link @see https://classic.yarnpkg.com/en/docs/cli/link
#! pnpm link @see https://pnpm.io/cli/link
#! The link command will only be executed if `[[crates]] link = true`
#! cli: `npm` | `yarn` | `pnpm`, default is `npm`
cli = "npm"

#! ---------------------------

#! rsw new <name>
[new]
#! @see https://rustwasm.github.io/docs/wasm-pack/commands/new.html
#! using: `wasm-pack` | `rsw` | `user`, default is `wasm-pack`
#! 1. wasm-pack: `rsw new <name> --template <template> --mode <normal|noinstall|force>`
#! 2. rsw: `rsw new <name>`, built-in templates
#! 3. user: `rsw new <name>`, if `dir` is not configured, use `wasm-pack new <name>` to initialize the project
using = "wasm-pack"
#! this field needs to be configured when `using = "user"`
#! `using = "wasm-pack"` or `using = "rsw"`, this field will be ignored
#! copy all files in this directory
dir = "my-template"

#! ################# NPM Package #################

#! When there is only `name`, other fields will use the default configuration
#! -------- package: rsw-hello --------
[[crates]]
#! npm package name
name = "rsw-hello"
#! run `npm link`: `true` | `false`, default is `false`
link = false

#! =======================================================

#! -------- package: @rsw/hello --------
# [[crates]]
# #! npm package name
# name = "@rsw/hello"
# #! default is `.`
# root = "."
# #! default is `pkg`
# out-dir = "pkg"
# #! target: bundler | nodejs | web | no-modules, default is `web`
# target = "web"
#! run `npm link`: `true` | `false`, default is `false`
# link = false
# #! rsw watch
# [crates.watch]
# #! default is `true`
# run = false
# #! profile: `dev` | `profiling`, default is `dev`
# profile = "dev"
# #! rsw build
# [crates.build]
# run = false
# #! profile: `release` | `profiling`, default is `release`
# profile = "release"
```

## License

MIT License © 2022 lencx
