//! rsw link

use std::path::PathBuf;
use std::process::Command;

use crate::core::RswInfo;
use crate::utils::print;

pub struct Link {
    cli: String,
    name: String,
    cwd: PathBuf,
}

impl Link {
    pub fn new(cli: String, cwd: PathBuf, name: String) -> Link {
        Link { cli, cwd, name }
    }
    pub fn init(self) {
        if self.cli == "yarn" {
            self.yarn_link();
        }
        if self.cli == "pnpm" {
            self.pnpm_link();
        }
    }

    pub fn npm_link(cli: String, crates: &Vec<&str>) {
        Command::new(cli).arg("link").args(crates).status().unwrap();
        print(RswInfo::CrateLink("npm link".into(), crates.join(" ")));
    }

    pub fn yarn_link(&self) {
        // register package
        // 1. cd <name>
        // 2. yarn link
        Command::new(&self.cli)
            .current_dir(&self.cwd)
            .arg("link")
            .status()
            .unwrap();

        // yarn link <name>
        Command::new(&self.cli)
            .arg("link")
            .arg(&self.name)
            .status()
            .unwrap();

        print(RswInfo::CrateLink(
            "yarn link".into(),
            self.name.to_string(),
        ));
    }
    pub fn pnpm_link(&self) {
        // pnpm link --dir <root_path>
        Command::new(&self.cli)
            .current_dir(&self.cwd)
            .args(["link", "--dir"])
            .arg(std::env::current_dir().unwrap())
            .status()
            .unwrap();

        print(RswInfo::CrateLink(
            "pnpm link".into(),
            self.name.to_string(),
        ));
    }
    pub fn unlink(cli: &String, crates: &Vec<String>) {
        // <npm|yarn|pnpm> unlink -g foo bar
        Command::new(cli)
            .arg("unlink")
            .args(crates)
            .status()
            .unwrap();

        if cli == "npm" {
            Command::new("npm")
                .args(["unlink", "-g"])
                .args(crates)
                .status()
                .unwrap();
        }

        print(RswInfo::Clean(format!("{} unlink", cli), crates.join(" ")));
    }
}
