mod api;
mod examples;

use slog::Logger;

use std::fs::{self, OpenOptions};

use Config;
use Result;

pub fn init(config: &Config, log: &Logger) -> Result<()> {
    let log = log.new(o!("command" => "init"));
    info!(log, "starting");

    // this function is huge, so I'm splitting it up

    create_top_level_docs_dir(config, &log)?;

    create_docs_readme(config, &log)?;
    create_doxidize_config(config, &log)?;
    create_menu_toml(config, &log)?;

    examples::create(config, &log)?;

    api::create(config, &log)?;

    info!(log, "done");
    Ok(())
}

fn create_top_level_docs_dir(config: &Config, log: &Logger) -> Result<()> {
    let docs_dir = config.markdown_path();

    debug!(log, "creating top-level docs dir"; o!("dir" => docs_dir.display()));
    fs::create_dir_all(&docs_dir)?;

    Ok(())
}

fn create_docs_readme(config: &Config, log: &Logger) -> Result<()> {
    // create a README.md
    let readme = config.readme_path();

    debug!(log, "creating README"; o!("file" => readme.display()));

    OpenOptions::new().create(true).append(true).open(readme)?;

    Ok(())
}

fn create_doxidize_config(config: &Config, log: &Logger) -> Result<()> {
    let doxidize_config = config.config_path();

    debug!(log, "creating Doxidize.toml"; o!("file" => doxidize_config.display()));
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(doxidize_config)?;

    Ok(())
}

fn create_menu_toml(config: &Config, log: &Logger) -> Result<()> {
    let menu = config.menu_path();

    debug!(log, "creating Menu.toml"; o!("file" => menu.display()));
    OpenOptions::new().create(true).append(true).open(menu)?;

    Ok(())
}


