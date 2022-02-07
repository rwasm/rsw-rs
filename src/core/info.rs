use colored::Colorize;
use core::fmt::Display;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum RswInfo {
    SplitLine,
    RswTomlOk,
    RswTomExist,
    BuildCmd(String),
    RunWatch(String),
    CrateFail(String, String),
    CrateOk(String, String, String),
    CrateChange(std::path::PathBuf),
    // NewHelp,
}

impl Display for RswInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RswInfo::CrateOk(name, mode, version) => {
                let rsw_tip = match *mode == "watch" {
                    true => "[👀 rsw::watch]",
                    false => "[✨ rsw::build]",
                };
                write!(
                    f,
                    "{} {} {}",
                    rsw_tip.green().on_black(),
                    name.purple(),
                    version.yellow(),
                )
            }
            RswInfo::CrateFail(name, mode) => {
                let rsw_tip = format!("[💢 rsw::{}]", mode);
                write!(f, "{} {}", rsw_tip.red().on_black(), name)
            }
            RswInfo::SplitLine => {
                write!(f, "{}", "◼◻".repeat(24).yellow())
            }
            RswInfo::BuildCmd(args) => {
                write!(
                    f,
                    "{} {} {}",
                    "[🚧 rsw::build]".yellow().on_black(),
                    "wasm-pack".green(),
                    args,
                )
            }
            RswInfo::CrateChange(path) => {
                write!(
                    f,
                    "{} {}",
                    "[📝 rsw::crate]".yellow().on_black(),
                    path.display(),
                )
            }
            RswInfo::RunWatch(name) => {
                write!(
                    f,
                    "{} {}",
                    "[🦀 rsw::watch]".yellow().on_black(),
                    name.purple(),
                )
            }
            RswInfo::RswTomExist => {
                write!(
                    f,
                    "{} {} already exists",
                    "[⚙️ rsw.toml]".red().on_black(),
                    "rsw.toml".yellow(),
                )
            }
            &RswInfo::RswTomlOk => {
                write!(
                    f,
                    "{} {} created successfully",
                    "[⚙️ rsw.toml]".green().on_black(),
                    "rsw.toml".yellow(),
                )
            }
            // RswInfo::NewHelp => {
            //     write!(
            //         f,
            //         "{} {} {}",
            //         "[💢 rsw::cmd]".red().on_black(),
            //         "For more information try",
            //         "rsw new -h".green(),
            //     )
            // }
        }
    }
}
