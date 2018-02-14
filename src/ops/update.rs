use slog::Logger;
use walkdir::WalkDir;

use std::collections::HashSet;
use std::fs::{remove_file, remove_dir};

use error;
use ops::init::api;
use Config;
use Result;

pub fn update(config: &Config, log: &Logger) -> Result<()> {
    let log = log.new(o!("command" => "update"));
    info!(log, "starting");

    let api_dir = config.api_markdown_path();

    debug!(log, "checking that the api docs folder exists"; o!("dir" => api_dir.display()));

    if !api_dir.is_dir() {
        // this folder is unilaterally created during init, even if there's no api to document
        return Err(error::UninitializedProject {
            location: config.root_path().to_path_buf(),
            command: "update",
        }.into());
    }

    debug!(log, "walking the api docs folder for existing markdown files"; o!("dir" => api_dir.display()));

    let mut existing_files = HashSet::new();

    for entry in WalkDir::new(api_dir).into_iter().filter_entry(|e| e.path().extension() == Some("md".as_ref())) {
        existing_files.insert(entry?.path().to_path_buf());
    }

    debug!(log, "recreating api docs");

    let new_files = api::create(config, &log)?;

    debug!(log, "deleting old files and directories that aren't in the new api docs");

    let mut directories = Vec::new();

    for orphan in existing_files.difference(&new_files) {
        if orphan.is_dir() {
            // leave directories until later in case we need to delete files within them first
            directories.push(orphan.clone());
        } else {
            debug!(log, "deleting file"; o!("file" => orphan.display()));
            remove_file(orphan)?;
        }
    }

    for orphan_dir in directories {
        debug!(log, "deleting directory"; o!("dir" => orphan_dir.display()));
        remove_dir(&orphan_dir)?;
    }

    info!(log, "done");
    Ok(())
}
