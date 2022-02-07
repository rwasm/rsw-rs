use std::process::Command;

use crate::config::NewOptions;

pub struct Create;

/// wasm-pack new
///
/// <https://rustwasm.github.io/docs/wasm-pack/commands/new.html>
impl Create {
    pub fn new(config: &NewOptions, name: &str, template: &Option<String>, mode: &Option<String>) {
        // println!("{:?}", config);

        let mut args = vec!["new", name];
        let arg_tpl = template.as_deref();
        let arg_mode = mode.as_deref();
        let arg_use = config.using.as_ref().unwrap();

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
    }
}
