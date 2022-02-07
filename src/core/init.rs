use std::fs::File;
use std::io::prelude::*;

use crate::core::RswInfo;
use crate::{config, template, utils};

pub struct Init;

impl Init {
    pub fn new() -> std::io::Result<()> {
        if !utils::path_exists(config::RSW_FILE) {
            File::create(config::RSW_FILE)?.write_all(template::RSW_TOML)?;
            println!("{}", RswInfo::RswTomlOk);
        } else {
            println!("{}", RswInfo::RswTomExist);
        }

        Ok(())
    }
}
