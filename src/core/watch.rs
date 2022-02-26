//! rsw watch

use notify::{DebouncedEvent::*, RecursiveMode::*, Watcher};
use regex::Regex;
use std::{
    collections::HashMap, fs, path::PathBuf, rc::Rc, sync::mpsc::channel, thread::sleep,
    time::Duration,
};

use crate::config::{CrateConfig, RswConfig};
use crate::core::{Build, RswErr, RswInfo};

use crate::utils::print;

pub struct Watch {
    config: Rc<RswConfig>,
    callback: Box<dyn Fn(&CrateConfig, PathBuf)>,
}

impl Watch {
    pub fn new(config: Rc<RswConfig>, callback: Box<dyn Fn(&CrateConfig, PathBuf)>) -> Watch {
        Watch { config, callback }
    }
    pub fn init(self) {
        let config = self.config;
        let caller = self.callback;
        let mut crate_map = HashMap::new();
        let mut path_map = HashMap::new();
        let (tx, rx) = channel();

        let mut watcher = match notify::watcher(tx, Duration::from_secs(1)) {
            Ok(w) => w,
            Err(e) => {
                print(RswErr::WatchFile(e));
                std::process::exit(1);
            }
        };

        for i in &config.crates {
            if i.watch.as_ref().unwrap().run.unwrap() {
                // TODO: local deps watch
                print(RswInfo::RunWatch(i.name.clone()));
                let crate_root = PathBuf::from(i.root.as_ref().unwrap()).join(&i.name);
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
                debug!("{:?}", event);

                match event {
                    Write(path) | Remove(path) | Rename(_, path) => {
                        let path = Rc::new(path);
                        for (key, val) in &path_map {
                            if Regex::new(val.to_str().unwrap())
                                .unwrap()
                                .is_match(path.to_owned().to_str().unwrap())
                            {
                                let crate_config = *crate_map.get(key).unwrap();
                                // TODO: build crate
                                print(RswInfo::CrateChange(path.clone().to_path_buf()));
                                // caller(crate_config, e);
                                let is_ok = Build::new(
                                    crate_config.clone(),
                                    "watch",
                                    config.cli.to_owned().unwrap(),
                                )
                                .init();

                                if is_ok {
                                    caller(crate_config, path.clone().to_path_buf());
                                }

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
