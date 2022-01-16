use colored::Colorize;
use core::fmt::Display;
// use toml::Value;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum RswInfo<'a> {
    RswsSlitLine,
    RswCrateOk(&'a str, Option<&'a str>),
    RswCrateFail(&'a str),
    RswBuildCmd(&'a str),
    // RswCrateChange,
}

impl Display for RswInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RswInfo::RswCrateOk(name, version) => {
                write!(
                    f,
                    "{} {} {}",
                    "[✨ rsw::build]".green().on_black(),
                    name.purple(),
                    version.unwrap().blue(),
                )
            }
            RswInfo::RswCrateFail(name) => {
                write!(f, "{} {}", "[💢 rsw::build]".red().on_black(), name)
            }
            RswInfo::RswsSlitLine => {
                write!(f, "{}", "*".repeat(36).yellow())
            }
            RswInfo::RswBuildCmd(args) => {
                write!(f, "{} {} {}", "[🚧 rsw::build]".yellow().on_black(), "wasm-pack".green(), args)
            }
        }
    }
}
