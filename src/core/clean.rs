//! rsw clean

use std::collections::HashMap;
use std::path::PathBuf;

use crate::config::RswConfig;
use crate::core::{Link, RswInfo};
use crate::utils::{path_exists, print};

pub struct Clean;

// clean: link & build
impl Clean {
    pub fn init(config: RswConfig) {
        let mut crates: Vec<String> = Vec::new();
        let mut path_map: HashMap<String, PathBuf> = HashMap::new();

        for i in &config.crates {
            let rsw_crate = i.clone();
            let crate_path = PathBuf::from(rsw_crate.root.as_ref().unwrap())
                .join(&i.name)
                .join(rsw_crate.out_dir.unwrap());

            let pkg_path = crate_path.to_string_lossy().to_string();

            if path_exists(&crate_path) {
                path_map.insert(pkg_path, crate_path);
            }

            crates.push(rsw_crate.name);
        }

        Link::unlink(&config.cli.unwrap(), crates);

        for (_crate, _path) in path_map {
            std::fs::remove_dir_all(_path).unwrap();
            print(RswInfo::Clean("package".into(), _crate));
        }
    }
}
