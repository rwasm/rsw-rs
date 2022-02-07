<p align="center">
  <img src="./rsw.png" width="120">
</p>

「🌐 Language」[简体中文](./README.zh_CN.md)

**`rsw = rs(rust) → w(wasm)`** - A command-line tool for automatically rebuilding local changes, based on the `wasm-pack` implementation.

## rsw-rs

### Feature

- rsw watch
- rsw build

### TODO

- rsw init - `rsw.toml`
- rsw new - rust crate
  - wasm-pack new
  - custom template
- deps watch - local sub-dependency file changes trigger hot updates
- debug info

## Usage

```bash
cargo install rsw
```

```bash
# help
rsw -h

# dev
rsw watch

# release
rsw build
```

## rsw.toml

> configuration file

- [TOML Doc](https://toml.io/en/)
- [`wasm-pack build` Doc](https://rustwasm.github.io/docs/wasm-pack/commands/build.html)

### Options

Create `rsw.toml` in the project root path, configure the `rust crate` parameter, and run the `rsw watch` or `rsw build` command.

- **`name`** - `optional`
- **`version`** - `optional`
- **`interval`** - development mode `rsw watch`, time interval for file changes to trigger `wasm-pack build`, default `50` milliseconds
- **`[new]`** - generate the `rust crate`
- **`[[crates]]`** - is an array that supports multiple `rust crate` configurations
  - **`name`** - npm package name, supporting organization, e.g. `@rsw/foo`
  - **`root`** - relative to the project root path, the default `.`
  - **`out-dir`** - npm package output path, default `pkg`
  - **`[crates.watch]`** - development mode
    - **`run`** - whether this `crate` needs to be watching, default is `true`
    - **`profile`** - `dev` | `profiling`, default is `dev`
  - **`[crates.build]`** - production mode
    - **`run`** - whether this `crate` needs to be build, default is `true`
    - **`profile`** - `release` | `profiling`, default is `release`

**Note: `name` in `[[crates]]` is required, other fields are optional.**

### Example

```toml
# rsw.toml

name = "rsw"
version = "0.1.0"
#! default value `50` ms
interval = 50

#! ---------------------------

#! rsw new <name>
[new]
#! @see https://rustwasm.github.io/docs/wasm-pack/commands/new.html
#! use: wasm-pack | rsw | user
#! 1. wasm-pack: `rsw new <name> --template <template> --mode <normal|noinstall|force>`
#! 2. rsw: `rsw new <name>`, built-in templates
#! 3. user: `rsw new <name>`, if `dir` is not configured, use `wasm-pack new <name>` to initialize the project
use = "wasm-pack"
#! this field needs to be configured when `use = "user"`
#! `use = "wasm-pack"` or `use = "rsw"`, this field will be ignored
dir = "my-template"

#! ################# NPM Package #################

#! When there is only `name`, other fields will use the default configuration
#! -------- package: rsw-hello --------
[[crates]]
name = "rsw-hello"

#! =======================================================

#! -------- package: @rsw/hello --------
# [[crates]]
# #! default value `.`
# root = "."
# #! npm package name
# name = "@rsw/hello"
# #! default value `pkg`
# out-dir = "pkg"
# #! rsw watch
# [crates.watch]
# #! default value `true`
# run = false
# #! profile: `dev` | `profiling`, default value `dev`
# profile = "dev"
# #! rsw build
# [crates.build]
# run = false
# #! profile: `release` | `profiling`, default value `release`
# profile = "release"
```

## License

MIT License © 2022 lencx
