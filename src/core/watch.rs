use notify::{DebouncedEvent::*, RecursiveMode::*, Watcher};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::thread::sleep;
use std::time::Duration;

use crate::config::{CrateConfig, RswConfig};

pub struct Watch;

impl Watch {
    pub fn new(config: &RswConfig, callback: Box<dyn Fn(&CrateConfig, &PathBuf)>) {
        let mut crate_map = HashMap::new();
        let mut path_map = HashMap::new();
        let (tx, rx) = channel();

        let mut watcher = match notify::watcher(tx, Duration::from_secs(1)) {
            Ok(w) => w,
            Err(e) => {
                println!("Error while trying to watch the files:\n\n\t{:?}", e);
                std::process::exit(1)
            }
        };

        for i in &config.crates {
            // TODO: local deps watch
            let crate_root = PathBuf::from(&i.root.as_ref().unwrap()).join(&i.name);
            let _ = watcher.watch(crate_root.join("src"), Recursive);
            let _ = watcher.watch(crate_root.join("Cargo.toml"), NonRecursive);

            crate_map.insert(&i.name, i);
            path_map.insert(&i.name, crate_root);
        }

        println!("{:#?}", crate_map);

        loop {
            let first_event = rx.recv().unwrap();
            // TODO: rsw options
            sleep(Duration::from_millis(50));
            let other_events = rx.try_iter();

            let all_events = std::iter::once(first_event).chain(other_events);

            for event in all_events {
                println!("event {:?}", event);

                match event {
                    Create(path) | Write(path) | Remove(path) | Rename(_, path) => {
                        // TODO: match package name
                        callback(&config.crates[0], &path);
                    }
                    _ => (),
                }
            }
        }
    }
}
