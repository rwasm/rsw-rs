use clap::{App, AppSettings};

use crate::config::RswConfig;
use crate::core::{error::RswErr, Build, RswInfo, Watch};

pub struct Cli;

impl Cli {
    pub fn new(config: &RswConfig) {
        let matches = App::new("rsw")
            .about("wasm-pack based build tool")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .setting(AppSettings::AllowExternalSubcommands)
            .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
            .subcommand(App::new("build").about("build rust crates, useful for shipping to production"))
            .subcommand(App::new("watch").about("automatically rebuilding local changes, useful for development and debugging"))
            // .subcommand(App::new("new").about("new crate"))
            .get_matches();

        match matches.subcommand() {
            Some(("build", _)) => {
                Cli::build(config, &"build".to_string());
            }
            Some(("watch", _)) => {
                // initial build
                Cli::build(config, &"watch".to_string());

                Watch::new(
                    config,
                    Box::new(|crate_config, e| {
                        // TODO: build crate
                        println!("{}", RswInfo::RswCrateChange(e));
                        Build::new(&crate_config, &"watch".to_string());
                    }),
                );
            }
            // TODO: crate template
            Some(("new", _)) => {
                println!("TODO => new crate");
            }
            _ => {
                println!("{}", RswErr::CmdErr);
            } // If all subcommands are defined above, anything else is unreachabe!()
        }

        // Continued program logic goes here...
    }

    pub fn build(config: &RswConfig, rsw_type: &String) {
        for i in &config.crates {
            if i.build.as_ref().unwrap().run.unwrap() {
                Build::new(i, rsw_type);
            }
        }
    }
}
