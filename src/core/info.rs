use colored::Colorize;
use core::fmt::Display;

#[derive(Debug)]
pub(crate) enum RswInfo<'a> {
    RswCrateOk(&'a str),
    RswCrateFail(&'a str),
    RswsSlitLine,
    // RswCrateChange,
}

impl Display for RswInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RswInfo::RswCrateOk(name) => {
                write!(
                    f,
                    "{} {}, {} must exist in the project root path.",
                    "[✨ rsw build]".green().on_black(),
                    name,
                    "rsw.toml".green(),
                )
            }
            RswInfo::RswCrateFail(name) => {
                write!(f, "{} {}", "[💢 rsw build]".red().on_black(), name)
            }
            RswInfo::RswsSlitLine => {
                write!(f, "{}", "*".repeat(36).yellow())
            }
        }
    }
}
