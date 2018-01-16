extern crate failure;
#[macro_use]
extern crate failure_derive;

use std::fs::{self, OpenOptions};
use std::path::Path;

use failure::Error;

type Result<T> = std::result::Result<T, Error>;

pub fn create_skeleton(dir: &Path) -> Result<()> {
    // create the top-level docs dir
    let docs_dir = dir.join("docs");
    fs::create_dir_all(&docs_dir)?;

    // create a README.md
    let readme = docs_dir.join("README.md");
    OpenOptions::new().create(true).append(true).open(readme)?;

    Ok(())
}