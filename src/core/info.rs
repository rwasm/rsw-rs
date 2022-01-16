use colored::Colorize;
use core::fmt::Display;
use notify::DebouncedEvent;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub(crate) enum RswInfo<'a> {
    RswsSlitLine,
    RswCrateOk(&'a str, &'static str, Option<&'a str>),
    RswCrateFail(&'a str, &'static str),
    RswBuildCmd(&'a str),
    RswCrateChange(DebouncedEvent),
}

impl Display for RswInfo<'_> {
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
                    version.unwrap().blue(),
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
            RswInfo::RswCrateChange(event) => {
                write!(f, " {} {:?}", "└".yellow(), event)
            }
        }
    }
}
