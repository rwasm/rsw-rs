//! rsw watch

use ignore::gitignore::Gitignore;
use notify::{DebouncedEvent::*, RecursiveMode::*, Watcher};
use path_clean::PathClean;
use std::{
    collections::HashMap, path::PathBuf, sync::mpsc::channel, sync::Arc, thread::sleep,
    time::Duration,
};

use crate::config::{CrateConfig, RswConfig};
use crate::core::{Build, RswErr, RswInfo};

use crate::utils::{get_root, print};

pub struct Watch {
    config: Arc<RswConfig>,
    callback: Arc<dyn Fn(&CrateConfig, PathBuf) + Send + Sync + 'static>,
}

impl Watch {
    pub fn new(
        config: Arc<RswConfig>,
        callback: Arc<dyn Fn(&CrateConfig, PathBuf) + Send + Sync + 'static>,
    ) -> Watch {
        Watch { config, callback }
    }
    pub fn init(self) {
        let config = self.config;
        let caller = self.callback;
        let mut crate_map = HashMap::new();
        let mut path_map = HashMap::new();
        let (tx, rx) = channel();
        // Keep the root as a path instead
        let cwd = get_root();

        let mut watcher = match notify::watcher(tx, Duration::from_secs(1)) {
            Ok(w) => w,
            Err(e) => {
                print(RswErr::WatchFile(e));
                std::process::exit(1);
            }
        };

        for i in &config.crates {
            // TODO: local deps watch

            // fix: https://github.com/rwasm/rsw-rs/issues/5#issuecomment-1242822143
            // Make sure the crate root path is absolute
            let crate_root = {
                let cr = PathBuf::from(i.root.as_deref().unwrap()).join(&i.name);
                if cr.is_relative() {
                    cwd.to_owned().join(cr)
                } else {
                    cwd.to_owned()
                }
            };

            let _ = watcher.watch(crate_root.join("src"), Recursive);
            let _ = watcher.watch(crate_root.join("Cargo.toml"), NonRecursive);

            crate_map.insert(&i.name, i);
            path_map.insert(&i.name, crate_root.clean());

            if i.watch.as_ref().unwrap().run.unwrap() {
                print(RswInfo::RunWatch(i.name.clone()));
            }
        }

        print(RswInfo::SplitLine);

        let (gitignore, _) = Gitignore::new("./.watchignore");
        let mut build_task_join_handle: Option<tokio::task::JoinHandle<()>> = None;

        loop {
            let first_event = rx.recv().unwrap();
            sleep(Duration::from_millis(config.interval.unwrap()));
            let other_events = rx.try_iter();

            let all_events = std::iter::once(first_event).chain(other_events);
            for event in all_events {
                debug!("{:?}", event);

                match event {
                    Write(path) | Remove(path) | Rename(_, path) => {
                        // Simplify gitignore matching code
                        // strip_prefix is simpler to use
                        let project_path = match path.strip_prefix(&cwd) {
                            Ok(p) => p,
                            Err(_) => continue,
                        };

                        if gitignore.matched(&project_path, false).is_ignore() {
                            continue;
                        }

                        for (key, val) in &path_map {
                            // Use starts_with instead of regex comparing strings
                            // This way we avoid potential issues with extra slashes
                            if !path.starts_with(val) {
                                continue;
                            }

                            let crate_config = (*crate_map.get(key).unwrap()).clone();
                            print(RswInfo::CrateChange(path.clone().to_path_buf()));

                            if let Some(join_handle) = build_task_join_handle {
                                debug!("abort building task");
                                join_handle.abort();
                            }

                            let config = config.clone();
                            let caller = caller.clone();
                            let path = path.clone();
                            let join_handle = tokio::spawn(async move {
                                let is_ok = Build::new(
                                    crate_config.clone(),
                                    "watch",
                                    config.cli.to_owned().unwrap(),
                                    false,
                                )
                                .init();

                                if is_ok {
                                    caller(&crate_config, path.clone());
                                }
                            });

                            build_task_join_handle = Some(join_handle);

                            break;
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
