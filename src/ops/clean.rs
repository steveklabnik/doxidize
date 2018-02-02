use slog::Logger;

use std::fs;

use Config;
use Result;

pub fn clean(config: &Config, log: &Logger) -> Result<()> {
    let log = log.new(o!("command" => "clean"));
    info!(log, "starting");

    fs::remove_dir_all(config.output_path())?;

    info!(log, "done");
    Ok(())
}