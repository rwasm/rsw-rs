use std::process::Command;

use crate::config::CrateConfig;
use crate::core::RswInfo;
use crate::utils;

pub struct Build;

impl Build {
    pub fn new(options: &CrateConfig, rsw_type: String) {
        let name = &options.name;
        let profile = options.build.as_ref().unwrap().profile.as_ref().unwrap();
        let mut args = vec!["build", name, "--out-dir", &options.out_dir.as_ref().unwrap()];
        let arg_profile = ["--", profile].join("");

        args.push(&arg_profile);

        let metadata = utils::get_crate_metadata(name);
        println!("{}", RswInfo::RswBuildCmd(args.join(" ").to_string()));

        let status = Command::new("wasm-pack")
            .args(args)
            .status()
            .expect("failed to execute process");

        println!("");

        match status.success() {
            true => {
                println!(
                    "{}",
                    RswInfo::RswCrateOk(
                        name.to_owned(),
                        rsw_type.to_owned(),
                        metadata["package"]["version"].to_string(),
                    )
                );
                // callback(options);
            }
            false => {
                println!(
                    "{}",
                    RswInfo::RswCrateFail(name.to_owned(), rsw_type.to_owned())
                );
            }
        }

        println!("\n{}\n", RswInfo::RswsSlitLine);
    }
}
