use notify::{watcher, RecursiveMode, Watcher};
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::config::CrateConfig;
use crate::core::RswInfo;
use crate::utils;

pub(crate) fn new(options: &CrateConfig) {
    let name = options.name.as_str();
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = match watcher(tx, Duration::from_millis(1500)) {
        Ok(w) => w,
        Err(e) => {
            println!("Error while trying to watch the files:\n\n\t{:?}", e);
            std::process::exit(1)
        }
    };

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(Path::new(name).join("src"), RecursiveMode::Recursive)
        .unwrap();

    for _ in rx.iter() {
        match rx.recv() {
            Ok(event) => {
                watch_build(name, options, event);
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

pub(crate) fn watch_build(name: &str, options: &CrateConfig, event: notify::DebouncedEvent) {
    let profile = options.watch.as_ref().unwrap().profile.as_ref().unwrap();
    let mut args = vec!["build", name];
    let arg_profile = ["--", profile].join("");

    args.push(&arg_profile);

    let metadata = utils::get_crate_metadata(name);
    println!("{}", RswInfo::RswBuildCmd(&args.join(" ")));

    let status = Command::new("wasm-pack")
        .args(args)
        .status()
        .expect("failed to execute process");

    println!("");

    match status.success() {
        true => {
            println!(
                "{}",
                RswInfo::RswCrateOk(name, "watch", metadata["package"]["version"].as_str())
            );
            println!("{}", RswInfo::RswCrateChange(event));
        }
        false => {
            println!("{}", RswInfo::RswCrateFail(name, "watch"));
        }
    }

    println!("\n{}\n", RswInfo::RswsSlitLine);
}
