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
    pub fn new<S: Into<String>, P: Into<PathBuf>>(cli: S, cwd: P, name: S) -> Link {
        Link {
            cli: cli.into(),
            cwd: cwd.into(),
            name: name.into(),
        }
    }

    pub fn init(self) {
        if self.cli == "yarn" {
            self.yarn_link();
        }
        if self.cli == "pnpm" {
            self.pnpm_link();
        }
    }

    pub fn npm_link(cli: String, crates: &[&str]) {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/c", &cli, "link"])
                .args(crates)
                .status()
                .unwrap();
        } else {
            Command::new(cli).arg("link").args(crates).status().unwrap();
        }

        print(RswInfo::CrateLink("npm link".into(), crates.join(" ")));
    }

    pub fn yarn_link(&self) {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/c", &self.cli, "link"])
                .current_dir(&self.cwd)
                .status()
                .unwrap();

            // yarn link <name>
            Command::new("cmd")
                .args(["/c", &self.cli, "link"])
                .arg(&self.name)
                .status()
                .unwrap();
        } else {
            // register package
            // 1. cd <root>/<name>
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
        }

        print(RswInfo::CrateLink(
            "yarn link".into(),
            self.name.to_string(),
        ));
    }

    pub fn pnpm_link(&self) {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/c", &self.cli, "link", "--dir"])
                .current_dir(&self.cwd)
                .arg(std::env::current_dir().unwrap())
                .status()
                .unwrap();
        } else {
            // pnpm link --dir <root_path>
            Command::new(&self.cli)
                .current_dir(&self.cwd)
                .args(["link", "--dir"])
                .arg(std::env::current_dir().unwrap())
                .status()
                .unwrap();
        }

        print(RswInfo::CrateLink(
            "pnpm link".into(),
            self.name.to_string(),
        ));
    }

    pub fn unlink(cli: &str, crates: &[String]) {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/c", cli, "unlink"])
                .args(crates)
                .status()
                .unwrap();
        } else {
            // <npm|yarn|pnpm> unlink -g foo bar
            Command::new(cli)
                .arg("unlink")
                .args(crates)
                .status()
                .unwrap();
        }

        if cli == "npm" {
            if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .args(["/c", cli, "unlink", "-g"])
                    .args(crates)
                    .status()
                    .unwrap();
            } else {
                Command::new("npm")
                    .args(["unlink", "-g"])
                    .args(crates)
                    .status()
                    .unwrap();
            }
        }

        print(RswInfo::Clean(format!("{} unlink", cli), crates.join(" ")));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_link() {
        let link = Link::new("pnpm", ".", "test");
        link.init();
    }
}
