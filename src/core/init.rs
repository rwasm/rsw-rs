use std::fs::File;
use std::io::prelude::*;

use crate::core::RswInfo;
use crate::{config, utils, template};

pub struct Init;

impl Init {
    pub fn new() -> std::io::Result<()> {
        if !utils::path_exists(config::RSW_FILE) {
            File::create(config::RSW_FILE)?
                .write_all(template::RSW_TOML)?;
        } else {
            println!("{}", RswInfo::FileExist);
        }

        Ok(())
    }
}
