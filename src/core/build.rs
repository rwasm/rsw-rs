use regex::Regex;
use std::process::Command;

use crate::config::CrateConfig;
use crate::core::RswInfo;
use crate::utils;

#[derive(Debug, Serialize, Deserialize)]
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
        let (_, scope) = self.get_pkg();
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

    // https://docs.npmjs.com/creating-a-package-json-file#required-name-and-version-fields
    fn get_pkg(&self) -> (String, String) {
        // example: @rsw/test | rsw-test | wasm123
        let re = Regex::new(r"(?x)(@([\w\d_-]+)/)?((?P<pkg_name>[\w\d_-]+))").unwrap();
        let caps = re.captures(&self.config.name).unwrap();
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
        let build = Build {
            config: CrateConfig {
                name: "@rsw/test".into(),
                root: None,
                out_dir: None,
                watch: None,
                build: None,
                target: None,
                mode: None,
            },
            rsw_type: "build".into(),
        };

        assert_eq!(build.get_pkg(), ("test".into(), "rsw".into()));
    }

    #[test]
    fn pkg_word_num() {
        let build = Build {
            config: CrateConfig {
                name: "wasm123".into(),
                root: None,
                out_dir: None,
                watch: None,
                build: None,
                target: None,
                mode: None,
            },
            rsw_type: "build".into(),
        };
        assert_eq!(build.get_pkg(), ("wasm123".into(), "".into()));
    }

    #[test]
    fn pkg_word_num_line() {
        let build = Build {
            config: CrateConfig {
                name: "@rsw-org/my_wasm".into(),
                root: None,
                out_dir: None,
                watch: None,
                build: None,
                target: None,
                mode: None,
            },
            rsw_type: "build".into(),
        };
        assert_eq!(build.get_pkg(), ("my_wasm".into(), "rsw-org".into()));
    }
}
