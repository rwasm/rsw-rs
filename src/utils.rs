use std::env;
use std::fs;

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
