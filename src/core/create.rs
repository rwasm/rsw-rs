use std::process::Command;

use crate::config::NewOptions;

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
    pub fn init(self) {
        // println!("{:?}", self.config);
        let mut args = vec!["new", self.name.as_str()];
        let arg_tpl = self.template.as_deref();
        let arg_mode = self.mode.as_deref();
        let arg_use = self.config.using.as_ref().unwrap();

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

        // use wasm-pack
        if arg_use == "wasm-pack" || !arg_tpl.is_none() {
            println!("{:?}", args);
            Command::new("wasm-pack")
                .args(args)
                .status()
                .expect("failed to execute process");
        }

        if arg_use == "rsw" {
            // TODO:
        }
    }
}
