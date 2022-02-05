use regex::Regex;
use std::process::Command;

use crate::config::CrateConfig;
use crate::core::RswInfo;
use crate::utils;

pub struct Build;

impl Build {
    pub fn new(options: &CrateConfig, rsw_type: &String) {
        let name = &options.name;
        let out_dir = &options.out_dir.as_ref().unwrap();
        let mut args = vec!["build", name.as_str(), "--out-dir", out_dir];

        // profile
        let mut profile = options.build.as_ref().unwrap().profile.as_ref().unwrap();
        if rsw_type == "watch" {
            profile = options.watch.as_ref().unwrap().profile.as_ref().unwrap();
        }
        let arg_profile = format!("--{}", profile);
        args.push(&arg_profile);

        // scope
        let (_, scope) = Build::get_pkg(&options.name);
        if scope != "" {
            args.push("--scope");
            args.push(scope.as_str());
        }

        let metadata = utils::get_crate_metadata(name.as_str());
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

    // https://docs.npmjs.com/creating-a-package-json-file#required-name-and-version-fields
    fn get_pkg(name: &str) -> (String, String) {
        // example: @rsw/test | rsw-test | wasm123
        let re = Regex::new(r"(?x)(@([\w\d_-]+)/)?((?P<pkg_name>[\w\d_-]+))").unwrap();
        let caps = re.captures(name).unwrap();
        let mut scope = "".to_string();
        if caps.get(2) != None {
            scope = caps.get(2).unwrap().as_str().to_string();
        }

        (caps["pkg_name"].to_owned(), scope)
    }
}

#[cfg(test)]
mod pkg_name_tests {
    use super::*;

    #[test]
    fn pkg_word() {
        assert_eq!(
            Build::get_pkg("@rsw/test").to_owned(),
            ("test".to_string(), "rsw".to_string())
        );
    }

    #[test]
    fn pkg_word_num() {
        assert_eq!(
            Build::get_pkg("wasm123").to_owned(),
            ("wasm123".to_string(), "".to_string())
        );
    }

    #[test]
    fn pkg_word_num_line() {
        assert_eq!(
            Build::get_pkg("@rsw-org/my_wasm").to_owned(),
            ("my_wasm".to_string(), "rsw-org".to_string())
        );
    }
}
