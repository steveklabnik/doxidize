extern crate failure;
#[macro_use]
extern crate failure_derive;

extern crate comrak;

use comrak::ComrakOptions;

use failure::Error;

use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

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

pub fn generate(dir: &Path) -> Result<()> {
    // we need to know where the docs are
    let docs_dir = dir.join("docs");

    // ensure that the docs dir exists in target
    let target_dir = dir.join("target").join("docs");
    fs::create_dir_all(&target_dir)?;

    // make the README.md render as an index.html
    let readme_path = docs_dir.join("README.md");

    let mut readme = File::open(readme_path)?;
    let mut contents = String::new();
    readme.read_to_string(&mut contents)?;

    let rendered = comrak::markdown_to_html(&contents, &ComrakOptions::default());

    let index_path = target_dir.join("index.html");
    let mut index = File::create(index_path)?;

    index.write_all(rendered.as_bytes())?;

    Ok(())
}