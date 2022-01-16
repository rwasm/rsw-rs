use std::process::Command;

use crate::config::CrateConfig;
use crate::core::RswInfo;
use crate::utils;

pub(crate) fn new(options: &CrateConfig) {
    let name = options.name.as_str();
    let profile = options.build.as_ref().unwrap().profile.as_ref().unwrap();
    let mut args = vec!["build", name];
    let arg_profile = ["--", profile].join("");

    args.push(&arg_profile);

    let metadata = utils::get_crate_metadata(name);
    println!("{}", RswInfo::RswBuildCmd(&args.join(" ")));

    let status = Command::new("wasm-pack")
        .args(args)
        .status()
        .expect("failed to execute process");

    println!("");

    match status.success() {
        true => {
            println!("{}", RswInfo::RswCrateOk(name, metadata["package"]["version"].as_str()));
        }
        false => {
            println!("{}", RswInfo::RswCrateFail(name));
        }
    }

    println!("\n{}\n", RswInfo::RswsSlitLine);
}
