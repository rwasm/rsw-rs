use clap::{AppSettings, Parser, Subcommand};

use crate::config::RswConfig;
use crate::core::{Build, Create, Init, RswInfo, Watch};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// build rust crates, useful for shipping to production
    Build,
    /// automatically rebuilding local changes, useful for development and debugging
    Watch,
    /// generate `rsw.toml` configuration file
    Init,
    /// quickly generate a crate with `wasm-pack new`, or set a custom template in `rsw.toml [new]`
    New {
        /// the name of the project
        name: String,
        /// `wasm-pack new`: The URL to the template [default: https://github.com/rustwasm/wasm-pack-template]
        #[clap(short = 't', long)]
        template: Option<String>,
        /// `wasm-pack new`: Should we install or check the presence of binary tools. [possible values: no-install, normal, force] [default: normal]
        #[clap(short = 'm', long)]
        mode: Option<String>,
    },
}

impl Cli {
    pub fn new() {
        match &Cli::parse().command {
            Commands::Build => {
                Cli::build(&Cli::parse_toml(), &"build".to_string());
            }
            Commands::Watch => {
                // initial build
                let config = &Cli::parse_toml();
                Cli::build(config, &"watch".to_string());

                Watch::new(
                    config,
                    Box::new(|crate_config, e| {
                        // TODO: build crate
                        println!("{}", RswInfo::CrateChange(e));
                        Build::new(&crate_config, &"watch".to_string());
                    }),
                );
            }
            Commands::Init => {
                Init::new().unwrap();
            }
            Commands::New { name, template, mode } => {
                let config = &Cli::parse_toml();
                let new_config = config.new.as_ref().unwrap();
                Create::new(new_config, name, template, mode);
            }
        }
    }

    pub fn parse_toml() -> RswConfig {
        RswConfig::new().unwrap()
    }

    pub fn build(config: &RswConfig, rsw_type: &String) {
        for i in &config.crates {
            if i.build.as_ref().unwrap().run.unwrap() {
                Build::new(i, rsw_type);
            }
        }
    }
}
