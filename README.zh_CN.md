<p align="center">
  <img src="./rsw.png" width="120">
</p>

「🌐 语言」[English](./README.md)

**`rsw = rs(rust) → w(wasm)`** - 基于 `wasm-pack` 实现的一个命令行工具，当本地文件变更时自动构建。

## rsw-rs

### 功能

- `rsw build` - 基于 `rsw.toml` 配置同时构建多个 `rust crate`
- `rsw watch` - 基于 `rsw.toml` 配置同时监听多个 `rust crate` 中的文件变更，自动触发构建。

## 用法

```bash
# 安装
cargo install rsw
```

```bash
# 查看帮助
rsw -h

# 开发模式
rsw watch

# 生产构建
rsw build
```

## rsw.toml

> 配置文件

- [TOML 文档](https://toml.io/cn/)
- [`wasm-pack build` 文档](https://rustwasm.github.io/docs/wasm-pack/commands/build.html)

## 配置信息

在项目根路径下创建 `rsw.toml`，配置 `rust crate` 参数，然后执行 `rsw watch` 或者 `rsw build`。

- **`name`** - 配置文件名称（无意义，可选）
- **`version`** - 配置文件版本（无意义，可选）
- **`interval`** - 开发模式 `rsw watch` 下，文件变更触发 `wasm-pack build` 的时间间隔，默认 `50` 毫秒
- **`[new]`** - 生成一个 `rust crate`
- **`[[crates]]`** - 是一个数组，支持多个 `rust crate` 配置
  - **`name`** - npm 包名，支持组织，例如 `@rsw/foo`
  - **`root`** - 此 `rust crate` 在项目根路径下的相对路径，默认 `.`
  - **`out-dir`** - npm 包输出路径，默认 `pkg`
  - **`[crates.watch]`** - 开发模式下的配置
    - **`run`** - 是否执行，默认为 `true`
    - **`profile`** - `dev` | `profiling`，默认 `dev`
  - **`[crates.build]`** - 生产构建下的配置
    - **`run`** - 是否执行，默认为 `true`
    - **`profile`** - `release` | `profiling`，默认 `release`

**注意：`[[crates]]` 中 `name` 是必须的，其他字段均为可选。**

### 示例

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
