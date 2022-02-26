use colored::Colorize;
use core::fmt::Display;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum RswInfo {
    SplitLine,
    RswTomlOk,
    RswTomExist,
    RunWatch(String),
    CrateLink(String, String),
    CrateFail(String, String),
    CrateOk(String, String, String),
    CrateChange(std::path::PathBuf),
    CrateNewOk(String),
    CrateNewExist(String),
    ConfigNewDir(String, std::path::PathBuf),
    Clean(String, String),
}

impl Display for RswInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RswInfo::CrateLink(cli, name) => {
                write!(
                    f,
                    "{} {} {}",
                    "[🔗 rsw::link]".green().on_black(),
                    cli,
                    name.yellow()
                )
            }
            RswInfo::Clean(a, b) => {
                write!(
                    f,
                    "{} {} {}",
                    "[🗑 rsw::clean]".green().on_black(),
                    a,
                    b.yellow()
                )
            }
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
                write!(f, "\n{}\n", "◼◻".repeat(24).yellow())
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
            RswInfo::RswTomlOk => {
                write!(
                    f,
                    "{} {} created successfully",
                    "[⚙️ rsw.toml]".green().on_black(),
                    "rsw.toml".yellow(),
                )
            }
            RswInfo::CrateNewOk(name) => {
                write!(
                    f,
                    "{} {} created successfully, please add the following code to `rsw.toml`\n\n{}",
                    "[🦀 rsw::crate]".green().on_black(),
                    name.yellow(),
                    format!("[[crates]]\nname = {:?}", name).yellow(),
                )
            }
            RswInfo::CrateNewExist(name) => {
                write!(
                    f,
                    "{} {} already exists",
                    "[🦀 rsw::crate]".red().on_black(),
                    name.yellow(),
                )
            }
            RswInfo::ConfigNewDir(template, path) => {
                write!(
                    f,
                    "{} [new] dir = \"{}\"\n{:?} No such file or director",
                    "[⚙️ rsw.toml]".red().on_black(),
                    template.yellow(),
                    path.display(),
                )
            }
        }
    }
}
