use toml_edit;
use slog::Logger;

use std::fs::File;
use std::io::prelude::*;

use config::Config;
use git;
use Result;

// adapted from https://github.com/rtomayko/rocco/blob/2586dc3bd4b0e9fa9bd076f492cdbf2924527199/Rakefile#L46
pub fn publish(config: &Config, log: &Logger) -> Result<()> {
    let log = log.new(o!("command" => "publish"));
    info!(log, "starting");

    // load up our Doxidize.toml so we can handle any base urls
    let path = config.config_path();
    let mut contents = String::new();
    let mut toml_file = File::open(path)?;
    toml_file.read_to_string(&mut contents)?;

    let doc = contents
        .parse::<toml_edit::Document>()
        .expect("invalid doxidize.toml");

    let base_url = doc["docs"]["base-url"]
        .as_value()
        .map(|v| v.as_str().expect("value of base-url was not a string"))
        .unwrap_or_default()
        .to_string();

    let mut target_dir = config.output_path().join("public");

    if !base_url.is_empty() {
        target_dir.push(&base_url)
    }

    let git_dir = target_dir.join(".git");

    // this name is totally arbitrary, but o was chosen to match rocco's Rakefile above.
    let remote_name = "o";

    // our sub-repo uses 'master' as its branch name, even though it pushes it to a branch
    // named 'gh-pages'. Subtle!
    let pages_head = {
        let mut path = git_dir.join("refs");
        path.push("heads");
        path.push("master");
        path
    };

    // if this file doesn't exist, then we don't have the gh-pages remote set up
    // to set it up we need to initialize the git repositry, add the remote, and sync the two
    if !pages_head.is_file() {
        git::init(&target_dir)?;
        git::initialize_remote(&target_dir, remote_name, !base_url.is_empty())?;
        git::reset_to_remote_head(&target_dir, remote_name)?;
    }

    let git_revision = git::head_revision(config.root_path())?;

    git::add_all(&target_dir)?;
    git::commit(&target_dir, &git_revision)?;

    git::sync_pages_branch(&target_dir, remote_name)?;

    git::push(config.root_path())?;

    info!(log, "done");
    Ok(())
}
