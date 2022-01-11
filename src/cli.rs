use clap::{App, AppSettings};

pub(crate) fn new() {
  let matches = App::new("rsw")
    .about("wasm-pack based build tool")
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .setting(AppSettings::AllowExternalSubcommands)
    .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
    .subcommand(
      App::new("build")
      .about("build crate")
    )
    .subcommand(
      App::new("watch")
      .about("watch crate")
    )
    .get_matches();

  match matches.subcommand() {
    Some(("build", _)) => {
      println!("TODO => build crates");
    }
    Some(("watch", _)) => {
      println!("TODO => watch crates");
    }
    _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
  }

  // Continued program logic goes here...
}
