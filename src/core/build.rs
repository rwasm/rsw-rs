use std::process::Command;

use crate::config::CrateConfig;
use crate::core::RswInfo;

pub(crate) fn new(options: &CrateConfig) {
    let name = options.name.as_str();
    let status = Command::new("wasm-pack")
        .args(["build", name])
        .status()
        .expect("failed to execute process");

    println!("");

    match status.success() {
        true => {
            println!("{}", RswInfo::RswCrateOk(name));
        }
        false => {
            println!("{}", RswInfo::RswCrateFail(name));
        }
    }

    println!("\n{}\n", RswInfo::RswsSlitLine);
}
