//! rsw new

use anyhow::Result;
use regex::Regex;
use std::process::Command;

use crate::config::NewOptions;
use crate::core::RswInfo;
use crate::template::Template;
use crate::utils::{self, path_exists, print, write_file};

pub struct Create {
    name: String,
    config: NewOptions,
    template: Option<String>,
    mode: Option<String>,
}

/// wasm-pack new
///
/// <https://rustwasm.github.io/docs/wasm-pack/commands/new.html>
impl Create {
    pub fn new(
        config: NewOptions,
        name: String,
        template: Option<String>,
        mode: Option<String>,
    ) -> Create {
        Create {
            name,
            config,
            template,
            mode,
        }
    }
    pub fn init(&self) {
        // println!("{:?}", self.config);
        let name = self.name.as_str();
        let (name2, scope) = utils::get_pkg(name);

        let scope2 = format!("@{}", scope);

        self.check_scope(&scope2);

        let mut args = vec!["new", &name2];
        let arg_tpl = self.template.as_deref();
        let arg_mode = self.mode.as_deref();
        let arg_use = self.config.using.as_ref().unwrap();
        let user_dirs = self.config.dir.as_ref().unwrap();

        self.check_crate();

        // --template: https://rustwasm.github.io/docs/wasm-pack/commands/new.html#template
        if !arg_tpl.is_none() {
            args.push("--template");
            args.push(arg_tpl.unwrap());
        }

        // --mode: https://rustwasm.github.io/docs/wasm-pack/commands/new.html#mode
        if !arg_mode.is_none() {
            args.push("--mode");
            args.push(arg_mode.unwrap());
        }

        // wasm-pack
        if arg_use == "wasm-pack" || !arg_tpl.is_none() {
            self.wp_cmd(&args, &scope2);
        }

        // built-in template
        if arg_use == "rsw" {
            self.create_crate().unwrap();
        }

        // custom template
        if arg_use == "user" {
            if user_dirs.is_empty() {
                self.wp_cmd(&args, &scope2);
            } else {
                self.user_crate(user_dirs);
            }
        }

        print(RswInfo::CrateNewOk(name.into()));
    }
    fn check_scope(&self, scope: &String) {
        if scope != "@" {
            let scope_dir = std::env::current_dir().unwrap().join(scope);
            if !path_exists(scope_dir.as_path()) {
                std::fs::create_dir(&scope_dir).unwrap();
            }
        }
    }
    fn check_crate(&self) {
        let name = &self.name;
        let path = std::env::current_dir().unwrap().join(name);
        if utils::path_exists(path.as_path()) {
            print(RswInfo::CrateNewExist(name.into()));
            std::process::exit(1);
        }
    }
    fn wp_cmd(&self, args: &Vec<&str>, scope: &String) {
        let mut scope_dir = std::env::current_dir().unwrap();
        if scope != "@" {
            scope_dir = scope_dir.join(scope);
        }
        Command::new("wasm-pack")
            .current_dir(scope_dir)
            .args(args)
            .status()
            .expect("failed to execute process");
    }
    fn create_crate(&self) -> Result<()> {
        let target = std::env::current_dir().unwrap().join(&self.name);
        let root = std::path::Path::new(&target);
        let template = Template::new(&root);

        let (name, _) = utils::get_pkg(&self.name);
        let re = Regex::new(r"(?P<name>rsw-template)").unwrap();
        let cargo = re.replace(std::str::from_utf8(&template.cargo).unwrap(), name);
        let readme = re.replace(std::str::from_utf8(&template.readme).unwrap(), &self.name);

        write_file(root, "README.md", readme.as_bytes())?;
        write_file(root, "Cargo.toml", &cargo.as_bytes())?;
        write_file(root, "src/lib.rs", &template.lib)?;

        Ok(())
    }
    fn user_crate(&self, dir: &str) {
        let root = std::env::current_dir().unwrap();
        let source = root.join(dir);
        if !path_exists(source.as_path()) {
            print(RswInfo::ConfigNewDir(dir.into(), source));
            std::process::exit(1);
        }
        utils::copy_dirs(root.join(dir), root.join(&self.name)).unwrap();
    }
}
