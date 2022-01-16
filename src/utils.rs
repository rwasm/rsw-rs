use std::{env, fs};
use toml::Value;

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
    let content = fs::read_to_string(crate_root).unwrap();
    let value = content.parse::<Value>().unwrap();
    value
}
