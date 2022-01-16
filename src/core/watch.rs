use notify::{watcher, RecursiveMode, Watcher};
use notify_rust::Notification;
use std::path::{Path, PathBuf};
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
                match event {
                    notify::DebouncedEvent::Write(path) => {
                        watch_build(name, options, "write".to_owned(), path);
                    },
                    notify::DebouncedEvent::Create(path) => {
                        watch_build(name, options, "create".to_owned(), path);
                    },
                    notify::DebouncedEvent::Remove(path) => {
                        watch_build(name, options, "remove".to_owned(), path);
                    },
                    notify::DebouncedEvent::Rename(_, _) => {},
                    notify::DebouncedEvent::NoticeWrite(_) => {},
                    notify::DebouncedEvent::Chmod(_) => {},
                    notify::DebouncedEvent::NoticeRemove(_) => {},
                    notify::DebouncedEvent::Rescan => {}
                    notify::DebouncedEvent::Error(_, _) => {}
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

pub(crate) fn watch_build(name: &str, options: &CrateConfig, event: String, path: PathBuf) {
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
            let path = &path.into_os_string().into_string().unwrap();
            println!("{}", RswInfo::RswCrateChange(event.as_ref(), path));

            let mut name = "[rsw::watch] ".to_owned();
            name.push_str(&event.to_owned());

            Notification::new()
                .summary(name.as_str())
                .body(path)
                .icon("firefox")
                .timeout(3)
                .show().unwrap();
        }
        false => {
            println!("{}", RswInfo::RswCrateFail(name, "watch"));
        }
    }

    println!("\n{}\n", RswInfo::RswsSlitLine);
}
