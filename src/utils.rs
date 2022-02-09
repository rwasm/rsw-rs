use anyhow::Result;
use regex::Regex;
use std::io::{Read, Write};
use std::{
    env,
    fs::{self, File},
    path::Path,
};
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

pub fn path_exists(path: &Path) -> bool {
    fs::metadata(path).is_ok()
}

// https://docs.npmjs.com/creating-a-package-json-file#required-name-and-version-fields
pub fn get_pkg(name: &str) -> (String, String) {
    // example: @rsw/test | rsw-test | wasm123
    let re = Regex::new(r"(?x)(@([\w\d_-]+)/)?((?P<pkg_name>[\w\d_-]+))").unwrap();
    let caps = re.captures(name).unwrap();
    let mut scope = "".to_string();
    if caps.get(2) != None {
        scope = caps.get(2).unwrap().as_str().to_string();
    }

    (caps["pkg_name"].to_owned(), scope)
}

// Checks if a file exists, if so, the destination buffer will be filled with
// its contents.
pub fn load_file_contents<P: AsRef<Path>>(filename: P, dest: &mut Vec<u8>) -> Result<()> {
    let filename = filename.as_ref();

    let mut buffer = Vec::new();
    File::open(filename)?.read_to_end(&mut buffer)?;

    // We needed the buffer so we'd only overwrite the existing content if we
    // could successfully load the file into memory.
    dest.clear();
    dest.append(&mut buffer);

    Ok(())
}

// @see: https://github.com/rust-lang/mdBook/blob/2213312938/src/utils/fs.rs#L61-L72
// This function creates a file and returns it. But before creating the file
// it checks every directory in the path to see if it exists,
// and if it does not it will be created.
pub fn create_file(path: &Path) -> Result<File> {
    println!("Creating {}", path.display());

    // Construct path
    if let Some(p) = path.parent() {
        println!("Parent directory is: {:?}", p);

        fs::create_dir_all(p)?;
    }

    File::create(path).map_err(Into::into)
}

// @see: https://github.com/rust-lang/mdBook/blob/2213312938/src/utils/fs.rs#L16-L20
// Write the given data to a file, creating it first if necessary
pub fn write_file<P: AsRef<Path>>(build_dir: &Path, filename: P, content: &[u8]) -> Result<()> {
    let path = build_dir.join(filename);

    create_file(&path)?.write_all(content).map_err(Into::into)
}

// recursive copy folder
pub fn copy_dirs(source: impl AsRef<Path>, target: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&target)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let entry_target = target.as_ref().join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dirs(entry.path(), entry_target)?;
        } else {
            fs::copy(entry.path(), entry_target)?;
        }
    }
    Ok(())
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

#[cfg(test)]
mod pkg_name_tests {
    use super::*;

    #[test]
    fn pkg_word() {
        assert_eq!(get_pkg("@rsw/test".into()), ("test".into(), "rsw".into()));
    }

    #[test]
    fn pkg_word_num() {
        assert_eq!(get_pkg("wasm123".into()), ("wasm123".into(), "".into()));
    }

    #[test]
    fn pkg_word_num_line() {
        assert_eq!(
            get_pkg("@rsw-org/my_wasm".into()),
            ("my_wasm".into(), "rsw-org".into())
        );
    }
}
