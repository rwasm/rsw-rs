use crate::config::CrateConfig;
use crate::core;
use crate::core::RswErr;
use clap::{App, AppSettings};

pub(crate) fn new(options: &CrateConfig) {
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
            if *options.build.as_ref().unwrap().run.as_ref().unwrap() {
                core::build(options);
            }
        }
        // watch (--dev)
        Some(("watch", _)) => {
            // println!("TODO => watch {}", options.name);
            core::watch(options);
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
