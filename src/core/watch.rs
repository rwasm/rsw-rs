use notify::{DebouncedEvent::*, RecursiveMode::*, Watcher};
use regex::Regex;
use std::{
    fs,
    path::PathBuf,
    thread::sleep,
    time::Duration,
    sync::mpsc::channel,
    collections::HashMap,
};

use crate::config::{CrateConfig, RswConfig};
use crate::core::{RswErr, RswInfo};

pub struct Watch;

impl Watch {
    pub fn new(config: &RswConfig, callback: Box<dyn Fn(&CrateConfig, PathBuf)>) {
        let mut crate_map = HashMap::new();
        let mut path_map = HashMap::new();
        let (tx, rx) = channel();

        let mut watcher = match notify::watcher(tx, Duration::from_secs(1)) {
            Ok(w) => w,
            Err(e) => {
                println!("{}", RswErr::WatchFile(e));
                std::process::exit(1);
            }
        };

        for i in &config.crates {
            if i.watch.as_ref().unwrap().run.unwrap() {
                // TODO: local deps watch
                println!("{}", RswInfo::RunWatch(i.name.clone()));
                let crate_root = PathBuf::from(&i.root.as_ref().unwrap()).join(&i.name);
                let _ = watcher.watch(crate_root.join("src"), Recursive);
                let _ = watcher.watch(crate_root.join("Cargo.toml"), NonRecursive);

                crate_map.insert(&i.name, i);
                path_map.insert(&i.name, fs::canonicalize(&crate_root).unwrap().to_owned());
            }
        }

        loop {
            let first_event = rx.recv().unwrap();
            sleep(Duration::from_millis(config.interval.unwrap()));
            let other_events = rx.try_iter();

            let all_events = std::iter::once(first_event).chain(other_events);

            for event in all_events {
                // println!("event {:?}", event);

                match event {
                    Write(path) | Remove(path) | Rename(_, path) => {
                        for (key, val) in &path_map {
                            if Regex::new(val.to_str().unwrap())
                                .unwrap()
                                .is_match(path.to_owned().to_str().unwrap())
                            {
                                callback(crate_map.get(key).unwrap(), path);
                                break;
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
