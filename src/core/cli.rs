//! rsw command parse

use clap::{AppSettings, Parser, Subcommand};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use crate::config::{CrateConfig, RswConfig};
use crate::core::{Build, Clean, Create, Init, Link, Watch};
use crate::utils::{init_rsw_crates, rsw_watch_file};

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
    /// generate `rsw.toml` configuration file
    Init,
    /// build rust crates, useful for shipping to production
    Build,
    /// automatically rebuilding local changes, useful for development and debugging
    Watch,
    /// clean - `npm link` and `wasm-pack build`
    Clean,
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
            Commands::Init => Cli::rsw_init(),
            Commands::Clean => Cli::rsw_clean(),
            Commands::Build => {
                Cli::rsw_build();
            }
            Commands::Watch => {
                Cli::rsw_watch(Some(Box::new(|a, b| {
                    let name = &a.name;
                    let path = &b.to_string_lossy().to_string();
                    let info_content = format!(
                        "[RSW::OK]\n[RSW::NAME] :~> {}\n[RSW::PATH] :~> {}",
                        name, path
                    );
                    rsw_watch_file(info_content.as_bytes(), "".as_bytes(), "info".into()).unwrap();
                })));
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
    pub fn rsw_clean() {
        Clean::new(Cli::parse_toml());
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

        let mut crates = Vec::new();
        for i in &config.crates {
            let name = &i.name;
            let out = i.out_dir.as_ref().unwrap();
            crates.push(format!(
                "{} :~> {}",
                name,
                PathBuf::from(name).join(out).to_string_lossy().to_string()
            ));
        }
        init_rsw_crates(crates.join("\n").as_bytes()).unwrap();

        config
    }
    pub fn wp_build(config: Rc<RswConfig>, rsw_type: &str) {
        // let crates_map = Rc::new(RefCell::new(HashMap::new()));
        let crates_map = Rc::new(RefCell::new(HashMap::new()));

        let cli = &config.cli.to_owned().unwrap();
        let mut has_crates = false;

        for i in &config.crates {
            if cli == "npm" && i.link.unwrap() {
                has_crates = true;
                let rsw_crate = i.clone();
                let crate_path = PathBuf::from(rsw_crate.root.as_ref().unwrap())
                    .join(&i.name)
                    .join(rsw_crate.out_dir.unwrap());
                crates_map.borrow_mut().insert(
                    rsw_crate.name.to_string(),
                    crate_path.to_string_lossy().to_string(),
                );
            }

            if i.build.as_ref().unwrap().run.unwrap() {
                Build::new(i.clone(), rsw_type, cli.into()).init();
            }
        }

        // npm link foo bar ...
        let crates = crates_map.borrow();
        if cli == "npm" && has_crates {
            Link::npm_link(
                cli.into(),
                &Vec::from_iter(crates.values().map(|i| i.as_str())),
            );
        }
    }
}
