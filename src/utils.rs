use std::{env, fs};
use toml::Value;

use crate::core::RswErr;

// https://stackoverflow.com/questions/35045996/check-if-a-command-is-in-path-executable-as-process
pub fn check_env_cmd(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

// get fields from `Cargo.toml`
pub fn get_crate_metadata(name: &str) -> Value {
    let crate_root = env::current_dir().unwrap().join(name).join("Cargo.toml");
    let content = fs::read_to_string(crate_root).unwrap_or_else(|e| {
        // TODO: create crate
        println!("{}", RswErr::Crate(name.to_string(), e));
        std::process::exit(1);
    });
    let value = content.parse::<Value>().unwrap();
    value
}

pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

// Parse a single key-value pair
// pub fn parse_key_val<T: std::fmt::Display>(s: &Option<String>, msg: T) -> (&str, &str) {
//     let s = s.as_deref().unwrap();
//     let pos = s
//         .find('=')
//         .unwrap_or_else(|| {
//             println!("invalid KEY=value: no `=` found in `{}`", s);
//             println!("{}", msg);
//             std::process::exit(1);
//         });

//     (&s[..pos], &s[pos + 1..])
// }