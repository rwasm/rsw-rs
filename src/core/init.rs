//! rsw init

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::core::RswInfo;
use crate::{
    config, template,
    utils::{path_exists, print},
};

pub struct Init;

impl Init {
    pub fn init() -> std::io::Result<()> {
        if !path_exists(Path::new(config::RSW_FILE)) {
            File::create(config::RSW_FILE)?.write_all(template::RSW_TOML)?;
            print(RswInfo::RswTomlOk);
        } else {
            print(RswInfo::RswTomExist);
        }

        Ok(())
    }
}
