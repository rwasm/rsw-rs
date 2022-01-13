use clap::{App, AppSettings};
use crate::config::CrateConfig;

pub(crate) fn new(options: &CrateConfig) {
    let matches = App::new("rsw")
        .about("wasm-pack based build tool")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
        .subcommand(App::new("build").about("build crate"))
        .subcommand(App::new("watch").about("watch crate"))
        .subcommand(App::new("new").about("new crate"))
        .get_matches();

    match matches.subcommand() {
        Some(("build", _)) => {
            println!("TODO => build {}", options.name);
            // println!("{:?}", options);
        }
        Some(("watch", _)) => {
            println!("TODO => watch {}", options.name);
        }
        Some(("new", _)) => {
            println!("TODO => new crate");
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    // Continued program logic goes here...
}
