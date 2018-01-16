extern crate failure;
#[macro_use]
extern crate failure_derive;

use std::fs::{self, File};
use std::path::Path;

use failure::Error;

type Result<T> = std::result::Result<T, Error>;

pub fn create_skeleton(dir: &Path) -> Result<()> {

    let docs_dir = dir.join("docs");

    fs::create_dir(&docs_dir)?;

    let readme = docs_dir.join("README.md");

    File::create(readme)?;

    Ok(())
}