use clap::{App, AppSettings};

use crate::config::RswConfig;
use crate::core::error::RswErr;
use crate::core::Watch;
use crate::core::Build;

pub struct Cli;

impl Cli {
    pub fn new(config: &RswConfig) {
        let matches = App::new("rsw")
            .about("wasm-pack based build tool")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .setting(AppSettings::AllowExternalSubcommands)
            .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
            .subcommand(App::new("build").about("build crates"))
            .subcommand(App::new("watch").about("watch crates"))
            .subcommand(App::new("new").about("new crate"))
            .get_matches();

        match matches.subcommand() {
            // build --(dev | profiling | release)
            Some(("build", _)) => {
                // println!("config {:?}", config.crates);
                for i in &config.crates {
                    Build::new(i, "build".to_string());
                }
            }
            // watch (--dev)
            Some(("watch", _)) => {
                Watch::new(config, Box::new(|crate_config, e| {
                    // TODO: buid crate
                    println!("[rsw::fs] {:?}", e);
                    Build::new(&crate_config, "watch".to_string());
                }));
            }
            Some(("new", _)) => {
                println!("TODO => new crate");
            }
            _ => {
                println!("{}", RswErr::CmdErr);
                // unreachable!()
            } // If all subcommands are defined above, anything else is unreachabe!()
        }

        // Continued program logic goes here...
    }
}
