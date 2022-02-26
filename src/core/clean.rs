//! rsw clean

use std::path::PathBuf;

use crate::config::RswConfig;
use crate::core::{Link, RswInfo};
use crate::utils::{path_exists, print};

pub struct Clean;

// clean: link & build
impl Clean {
    pub fn new(config: RswConfig) {
        let mut crates: Vec<String> = [].to_vec();

        for i in &config.crates {
            let rsw_crate = i.clone();
            let crate_path = PathBuf::from(rsw_crate.root.as_ref().unwrap())
                .join(&i.name)
                .join(rsw_crate.out_dir.unwrap());

            let pkg_path = crate_path.to_string_lossy().to_string();

            if path_exists(&crate_path) {
                std::fs::remove_dir_all(crate_path).unwrap();
                print(RswInfo::Clean("package".into(), pkg_path));
            }

            crates.push(rsw_crate.name);
        }

        Link::unlink(&config.cli.unwrap().to_owned(), &crates);
    }
}
