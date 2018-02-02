extern crate doxidize;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate tempdir;

mod util;

use doxidize::Config;

use tempdir::TempDir;

#[test]
fn clean_deletes_directory_in_target() {
    let dir = TempDir::new("create_root_readme").expect("could not generate temp dir");
    let log = util::make_logger();

    let dir_path = dir.path();

    util::cargo_init(dir_path).expect("Could not create sample crate");

    let config = Config::with_manifest_path(dir_path.join("Cargo.toml"));

    doxidize::ops::init(&config, &log).expect("init failed");
    doxidize::ops::build(&config, &log).expect("build failed");

    let target_docs_dir = dir_path.join("target").join("docs");
    assert!(target_docs_dir.is_dir(), format!("{} is not a directory", target_docs_dir.display()));

    doxidize::ops::clean(&config, &log).expect("clean failed");

    assert!(!target_docs_dir.is_dir());
}