use std::process::Command;

use crate::config::CrateConfig;
use crate::core::RswInfo;
use crate::utils;

pub struct Build {
    config: CrateConfig,
    rsw_type: String,
}

impl Build {
    pub fn new(config: CrateConfig, rsw_type: &str) -> Build {
        Build {
            config,
            rsw_type: rsw_type.into(),
        }
    }

    pub fn init(&self) {
        let config = &self.config;
        let rsw_type = &self.rsw_type;
        let name = &config.name;
        let out_dir = config.out_dir.as_ref().unwrap();
        let mut args = vec!["build", name, "--out-dir", out_dir];

        // profile
        let mut profile = config.build.as_ref().unwrap().profile.as_ref().unwrap();
        if rsw_type == "watch" {
            profile = config.watch.as_ref().unwrap().profile.as_ref().unwrap();
        }
        let arg_profile = format!("--{}", profile);
        args.push(&arg_profile);

        // scope
        let (_, scope) = utils::get_pkg(&self.config.name);
        if scope != "" {
            args.push("--scope");
            args.push(scope.as_str());
        }

        let metadata = utils::get_crate_metadata(name.as_str());
        println!("{}", RswInfo::BuildCmd(args.join(" ").to_string()));

        let status = Command::new("wasm-pack")
            .args(args)
            .status()
            .expect("failed to execute process");

        println!("");

        match status.success() {
            true => {
                println!(
                    "{}",
                    RswInfo::CrateOk(
                        name.into(),
                        rsw_type.into(),
                        metadata["package"]["version"].to_string(),
                    )
                );
            }
            false => {
                println!("{}", RswInfo::CrateFail(name.into(), rsw_type.into()));
            }
        }

        println!("\n{}\n", RswInfo::SplitLine);
    }
}
