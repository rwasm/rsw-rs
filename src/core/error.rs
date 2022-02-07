use colored::Colorize;
use core::fmt::Display;

pub enum RswErr {
    WasmPack,
    Command,
    Config(std::io::Error),
    ParseToml(toml::de::Error),
    WatchFile(notify::Error),
    Crate(String, std::io::Error),
}

impl Display for RswErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RswErr::WasmPack => {
                write!(f,
                    "{} wasm-pack {}\nCannot find wasm-pack in your PATH. Please make sure wasm-pack is installed.",
                    "[⚙️ rsw::env]".red().on_black(),
                    "https://github.com/rustwasm/wasm-pack".green(),
                )
            }
            RswErr::Config(_err) => {
                write!(
                    f,
                    "{} {} must exist in the project root path.",
                    "[⚙️ rsw.toml]".red().on_black(),
                    "rsw.toml".yellow(),
                    // _err,
                )
            }
            RswErr::ParseToml(err) => {
                write!(f, "{} {}", "[⚙️ rsw.toml]".red().on_black(), err)
            }
            RswErr::Command => {
                write!(f, "{} rsw -h", "[⚠️ rsw::cmd]".red().on_black())
            }
            RswErr::WatchFile(e) => {
                write!(
                    f,
                    "{} Error while trying to watch the files:\n\t{}",
                    "[🦀 rsw::crate]".red().on_black(),
                    e
                )
            }
            RswErr::Crate(name, err) => {
                write!(
                    f,
                    "{} {} {}",
                    "[🦀 rsw::crate]".red().on_black(),
                    name.yellow(),
                    err
                )
            }
        }
    }
}
