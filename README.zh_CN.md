<p align="center">
  <img src="./rsw.png" width="120">
  <h2 align="center">rsw-rs</h2>
</p>

**`rsw = rs(rust) → w(wasm)`** - 基于 `wasm-pack` 实现的一个命令行工具，当本地文件变更时自动构建。

**[English](./README.md) | 简体中文**

## 预安装

- [rust](https://www.rust-lang.org/learn/get-started)
- [nodejs](https://nodejs.org)
- [wasm-pack](https://github.com/rustwasm/wasm-pack)

## 用法

```bash
# 在 Rust 环境下安装到全局
cargo install rsw

# 或者在 Node.js 环境下安装
npm install -D @rsw/cli
```

```bash
# 查看帮助
rsw -h

# rsw.toml - 初始化配置
rsw init

# 生成一个 wasm 项目
rsw new <name>

# 开发模式
rsw watch

# 生产构建
rsw build

# 清除 link 及 build 产物
rsw clean
```

## Awesome rsw

- [[rsw demo] learn-wasm](https://github.com/lencx/learn-wasm) - 🎲 Learning WebAssembly
- [vite-plugin-rsw](https://github.com/lencx/vite-plugin-rsw) - 🦀 wasm-pack plugin for Vite
- [create-mpl](https://github.com/lencx/create-mpl) - ⚡️ Create a project in seconds!

## 日志

```bash
# @see: https://github.com/env-logger-rs/env_logger
# RUST_LOG=rsw=<info|trace|debug|error|warn> rsw <watch|build|new>
# 1. info
RUST_LOG=rsw=info rsw <SUBCOMMAND>

# 2. all: info, trace, debug, error, warn
RUST_LOG=rsw rsw <SUBCOMMAND>
```

### .watchignore

定义要忽略的文件/路径，类似于 `.gitignore`。

例如:

```bash
# .watchignore
*.js
a/b/**/*.txt
!a/b/**/main.txt
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
- **`cli`** - `npm` | `yarn` | `pnpm`，默认是 `npm`。使用指定的 `cli` 执行 `link`，例如 `npm link`
- **`[new]`** - 使用 `wasm-pack new` 快速生成一个 `rust crate`, 或者使用自定义模板 `rsw.toml -> [new] -> using`
  - **`using`** - `wasm-pack` | `rsw` | `user`, 默认是 `wasm-pack`
    - `wasm-pack` - `rsw new <name> --template <template> --mode <normal|noinstall|force>`，了解更多 [wasm-pack new 文档](https://rustwasm.github.io/docs/wasm-pack/commands/new.html)
    - `rsw` - `rsw new <name>`, 使用内置模板
    - `user` - `rsw new <name>`, 如果未设置 `dir`，则使用 `wasm-pack new <name>` 初始化项目
  - **`dir`** - 如果 `using = "user"` 则复制此目录下的所有文件初始化项目，`using = "wasm-pack"` 或 `using = "rsw"` 时，则忽略这个字段
- **`[[crates]]`** - 是一个数组，支持多个 `rust crate` 配置
  - **`name`** - npm 包名，支持组织，例如 `@rsw/foo`
  - **`root`** - 此 `rust crate` 在项目根路径下的相对路径，默认 `.`
  - **`link`** - `true` | `false`，默认为 `false`，此 `rust crate` 构建后是否执行 `link` 命令，与 `cli` 配合使用
  - **`target`** - `bundler` | `nodejs` | `web` | `no-modules`, 默认 `web`
  - **`out-dir`** - npm 包输出路径，默认 `pkg`
  - **`[crates.watch]`** - 开发模式下的配置
    - **`run`** - 是否执行，默认为 `true`
    - **`profile`** - `dev` | `profiling`，默认 `dev`
  - **`[crates.build]`** - 生产构建下的配置
    - **`run`** - 是否执行，默认为 `true`
    - **`profile`** - `release` | `profiling`，默认 `release`

**注意：`[[crates]]` 中 `name` 是必须的，其他字段均为可选。**

## .rsw

> `rsw watch` - 临时目录

- rsw.info - `watch` 模式下相关信息
  - `[RSW::OK]`
  - `[RSW::ERR]`
  - `[RSW::NAME]`
  - `[RSW::PATH]`
  - `[RSW::BUILD]`
- rsw.err - `wasm-pack build` 失败信息
- rsw.crates - `rsw.toml` 中的所有包信息

### 示例

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
# run = true
# #! profile: `dev` | `profiling`, default is `dev`
# profile = "dev"
# #! rsw build
# [crates.build]
# #! default is `true`
# run = true
# #! profile: `release` | `profiling`, default is `release`
# profile = "release"
```

## License

MIT License © 2022 lencx
