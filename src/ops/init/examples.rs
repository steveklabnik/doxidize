use slog::Logger;

use std::fs::{self, File};
use std::io::prelude::*;

use Config;
use Result;

pub fn create(config: &Config, log: &Logger) -> Result<()> {
    let examples_dir = config.examples_markdown_path();
    debug!(log, "creating examples dir";
    o!("dir" => examples_dir.display()));
    fs::create_dir_all(&examples_dir)?;

    if config.examples_path().is_dir() {
        for entry in fs::read_dir(config.examples_path())? {
            let entry = entry?;
            let path = entry.path();

            // we want only files
            if !path.is_file() {
                continue;
            }
            trace!(log, "file is a file, continuing");

            if let Some(extension) = path.extension() {
                // we only want .rs files
                if extension != "rs" {
                    continue;
                }
            } else {
                // we don't want files with no extension
                continue;
            }
            trace!(log, "file is a rust file, continuing");

            // we certainly have a file name, since we're looping over real files
            let file_name = path.file_name().unwrap();
            let rust_file = config.examples_path().join(file_name);

            trace!(log, "reading file";
            "file" => rust_file.display());
            let mut file = File::open(&rust_file)?;
            let mut code = String::new();
            file.read_to_string(&mut code)?;

            let markdown_path = examples_dir.join(file_name).with_extension("md");

            trace!(log, "rendering to markdown";
            "file" => path.display(), "file" => markdown_path.display());
            let mut file = File::create(markdown_path)?;

            file.write_all(
                config.handlebars()
                    .render(
                        "example",
                        &json!({"name": file_name.to_str().unwrap(), "code": code}),
                    )?
                    .as_bytes(),
            )?;
        }
    }



    Ok(())
}

