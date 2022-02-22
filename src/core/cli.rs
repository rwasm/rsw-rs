//! rsw command parse

use clap::{AppSettings, Parser, Subcommand};
use std::rc::Rc;

use crate::config::{CrateConfig, RswConfig};
use crate::core::{Build, Create, Init, Watch};
use crate::utils::rsw_watch_file;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
        /// `wasm-pack new`: The URL to the template <https://github.com/rustwasm/wasm-pack-template>
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
                Cli::rsw_build();
            }
            Commands::Watch => {
                Cli::rsw_watch(Some(Box::new(|a, b| {
                    let name = &a.name;
                    let path = &b.to_string_lossy().to_string();
                    let content = ["[RSW::WATCH]: ", name, "\n\n[RSW::FILE]: ", path];
                    rsw_watch_file(content.concat().as_bytes()).unwrap();
                })));
            }
            Commands::Init => {
                Cli::rsw_init();
            }
            Commands::New {
                name,
                template,
                mode,
            } => {
                Cli::rsw_new(name, template, mode);
            }
        }
    }
    pub fn rsw_build() {
        Cli::wp_build(Rc::new(Cli::parse_toml()), "build");
    }
    pub fn rsw_watch(callback: Option<Box<dyn Fn(&CrateConfig, std::path::PathBuf)>>) {
        // initial build
        let config = Rc::new(Cli::parse_toml());
        Cli::wp_build(config.clone(), "watch");

        println!("");

        Watch::new(config, callback.unwrap()).init();
    }
    pub fn rsw_init() {
        Init::new().unwrap();
    }
    pub fn rsw_new(name: &String, template: &Option<String>, mode: &Option<String>) {
        Create::new(
            Cli::parse_toml().new.unwrap(),
            name.into(),
            template.to_owned(),
            mode.to_owned(),
        )
        .init();
    }
    pub fn parse_toml() -> RswConfig {
        let config = RswConfig::new().unwrap();
        trace!("{:#?}", config);
        config
    }
    pub fn wp_build(config: Rc<RswConfig>, rsw_type: &str) {
        for i in &config.crates {
            if i.build.as_ref().unwrap().run.unwrap() {
                Build::new(i.clone(), rsw_type).init();
            }
        }
    }
}
