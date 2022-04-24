//! rsw link

use std::path::PathBuf;

use crate::core::RswInfo;
use crate::utils::{get_root, os_cli, print};

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
    pub fn npm_link(cli: String, crates: Vec<String>) {
        os_cli(
            cli,
            [&["link".into()], &crates[..]].concat(),
            get_root(),
        );
        print(RswInfo::CrateLink("npm link".into(), crates.join(" ")));
    }

    pub fn yarn_link(&self) {
        // register package
        // 1. cd <root>/<name>
        // 2. yarn link
        os_cli(self.cli.clone(), ["link".into()].to_vec(), &self.cwd);

        // yarn link <name>
        os_cli(
            self.cli.clone(),
            ["link".into(), self.name.clone()].to_vec(),
            get_root(),
        );

        print(RswInfo::CrateLink(
            "yarn link".into(),
            self.name.to_string(),
        ));
    }

    pub fn pnpm_link(&self) {
        // pnpm link --dir <root_path>
        let dir = get_root().to_string_lossy().to_string();
        os_cli(
            self.cli.clone(),
            ["link".into(), "--dir".into(), dir].to_vec(),
            &self.cwd,
        );

        print(RswInfo::CrateLink(
            "pnpm link".into(),
            self.name.to_string(),
        ));
    }

    pub fn unlink(cli: &String, crates: Vec<String>) {
        let root = get_root();

        // <yarn|pnpm> unlink foo bar
        os_cli(
            cli.clone(),
            [&["unlink".into()], &crates[..]].concat(),
            &root,
        );

        if cli == "npm" {
            // npm unlink -g foo bar
            os_cli(
                cli.clone(),
                [&["unlink".into(), "-g".into()], &crates[..]].concat(),
                root,
            );
        }

        print(RswInfo::Clean(format!("{} unlink", cli), crates.join(" ")));
    }
}
