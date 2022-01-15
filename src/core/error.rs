use colored::Colorize;
use core::fmt::Display;

pub(crate) enum RswErr {
    FileErr(std::io::Error),
    ParseErr(toml::de::Error),
    EnvErr,
    CmdErr,
}

impl Display for RswErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RswErr::FileErr(err) => {
                write!(
                    f,
                    "{} {}, {} must exist in the project root path.",
                    "[⚙️ rsw.toml]".red().on_black(),
                    err,
                    "rsw.toml".green(),
                )
            }
            RswErr::ParseErr(err) => {
                write!(f, "{} {}", "[⚙️ rsw.toml]".red().on_black(), err)
            }
            RswErr::EnvErr => {
                write!(f,
                    "{} wasm-pack {}\nCannot find wasm-pack in your PATH. Please make sure wasm-pack is installed.",
                    "[⚙️ rsw::env]".red().on_black(),
                    "https://github.com/rustwasm/wasm-pack".green(),
                )
            }
            RswErr::CmdErr => {
                write!(f, "{} rsw -h", "[⚠️ rsw::cmd]".red().on_black(),)
            }
        }
    }
}
