extern crate failure;
#[macro_use]
extern crate failure_derive;

extern crate comrak;

extern crate handlebars;

extern crate rls_analysis as analysis;
extern crate rls_data as analysis_data;

#[macro_use]
extern crate serde_json;

extern crate simple_server;

extern crate walkdir;

mod cargo;
mod error;
mod git;
pub mod ops;
mod ui;

use failure::Error;

use std::path::{Path, PathBuf};

use ui::{Ui, Verbosity};

type Result<T> = std::result::Result<T, Error>;

/// A structure that contains various fields that hold data in order to generate doc output.
#[derive(Debug)]
pub struct Config {
    /// Interactions with the user interface.
    ui: Ui,

    /// Path to the `Cargo.toml` file for the crate being analyzed
    manifest_path: PathBuf,

    /// Path to place rustdoc output
    output_path: Option<PathBuf>,

    /// Contains the Cargo analysis output for the crate being documented
    host: analysis::AnalysisHost,
}

impl Config {
    /// Create a new `Config` based off the location of the manifest as well as assets generated
    /// during the build phase
    ///
    /// ## Arguments
    ///
    /// - `manifest_path`: The path to the `Cargo.toml` of the crate being documented
    pub fn new(verbosity: Verbosity, manifest_path: PathBuf) -> Result<Config> {
        let host = analysis::AnalysisHost::new(analysis::Target::Debug);

        if !manifest_path.is_file() || !manifest_path.ends_with("Cargo.toml") {
            return Err(failure::err_msg(
                "The --manifest-path must be a path to a Cargo.toml file",
            ));
        }

        Ok(Config {
            ui: Ui::new(verbosity),
            manifest_path,
            output_path: None,
            host,
        })
    }

    /// Returns the directory containing the `Cargo.toml` of the crate being documented.
    pub fn root_path(&self) -> &Path {
        // unwrap() is safe, as manifest_path will point to a file
        self.manifest_path.parent().unwrap()
    }

    /// Returns the directory where output files should be placed
    pub fn output_path(&self) -> PathBuf {
        match self.output_path {
            Some(ref path) => path.clone(),
            None => self.root_path().join("target").join("doc"),
        }
    }

    /// Set the directory where output files should be placed
    pub fn set_output_path(&mut self, output_path: PathBuf) {
        self.output_path = Some(output_path);
    }

    /// Returns the path to the generated documentation.
    pub fn documentation_path(&self) -> PathBuf {
        self.output_path().join("data.json")
    }
}