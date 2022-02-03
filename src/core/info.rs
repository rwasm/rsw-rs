use colored::Colorize;
use core::fmt::Display;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum RswInfo {
    RswsSlitLine,
    RswCrateOk(String, String, String),
    RswCrateFail(String, String),
    RswBuildCmd(String),
    RswCrateChange(String, String),
}

impl Display for RswInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RswInfo::RswCrateOk(name, mode, version) => {
                let rsw_tip = match *mode == "watch" {
                    true => "[👀 rsw::watch]",
                    false => "[✨ rsw::build]",
                };
                write!(
                    f,
                    "{} {} {}",
                    rsw_tip.green().on_black(),
                    name.purple(),
                    version.blue(),
                )
            }
            RswInfo::RswCrateFail(name, mode) => {
                let rsw_tip = format!("[💢 rsw::{}]", mode);
                write!(f, "{} {}", rsw_tip.red().on_black(), name)
            }
            RswInfo::RswsSlitLine => {
                write!(f, "{}", "*".repeat(36).yellow())
            }
            RswInfo::RswBuildCmd(args) => {
                write!(
                    f,
                    "{} {} {}",
                    "[🚧 rsw::build]".yellow().on_black(),
                    "wasm-pack".green(),
                    args
                )
            }
            RswInfo::RswCrateChange(event, path) => {
                write!(f, " {} {} {:?}", "└".yellow(), event, path)
            }
        }
    }
}
