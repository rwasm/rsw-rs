//! rsw template

use std::path::Path;

use crate::utils::load_file_contents;

// config
pub static RSW_TOML: &[u8] = include_bytes!("rsw.toml");

// crate
pub static CARGO_TOML: &[u8] = include_bytes!("rsw_cargo.toml");
pub static LIB_RS: &[u8] = include_bytes!("rsw_lib.rs");
pub static README: &[u8] = include_bytes!("rsw_readme.md");

#[derive(Debug)]
pub struct Template {
    pub cargo: Vec<u8>,
    pub readme: Vec<u8>,
    pub lib: Vec<u8>,
}

impl Template {
    pub fn new<P: AsRef<Path>>(template_dir: P) -> Self {
        // Creates a `Template` from the given `template_dir`.
        // If a file is found in the template dir, it will override the default version.
        let template_dir = template_dir.as_ref();
        let mut template = Template::default();

        // If the theme directory doesn't exist there's no point continuing...
        if !template_dir.exists() || !template_dir.is_dir() {
            return template;
        }

        // Check for individual files, if they exist copy them across
        {
            let files = vec![
                (template_dir.join("Cargo.tmol"), &mut template.cargo),
                (template_dir.join("README.md"), &mut template.readme),
                (template_dir.join("src/lib.rs"), &mut template.lib),
            ];

            let load_with_warn = |filename: &Path, dest| {
                if !filename.exists() {
                    // Don't warn if the file doesn't exist.
                    return false;
                }
                if let Err(e) = load_file_contents(filename, dest) {
                    println!("Couldn't load custom file, {}: {}", filename.display(), e);
                    false
                } else {
                    true
                }
            };

            for (filename, dest) in files {
                load_with_warn(&filename, dest);
            }
        }

        template
    }
}

impl Default for Template {
    fn default() -> Template {
        Template {
            cargo: CARGO_TOML.to_owned(),
            readme: README.to_owned(),
            lib: LIB_RS.to_owned(),
        }
    }
}
