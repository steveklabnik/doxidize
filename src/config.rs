use analysis;
use ui::{Ui, Verbosity};
use Result;
use std::path::{Path, PathBuf};
use failure;
use std::default::Default;
use serde::Deserializer;

/// A structure that contains various fields that hold data in order to generate doc output.
#[derive(Debug, Deserialize, Configure)]
#[serde(default)]
pub struct Config {
    /// Interactions with the user interface.
    ui: Ui,

    /// Path to the `Cargo.toml` file for the crate being analyzed
    manifest_path: PathBuf,

    /// Path to place rustdoc output
    output_path: Option<PathBuf>,

    /// Contains the Cargo analysis output for the crate being documented
    #[serde(deserialize_with = "default_host")]
    host: analysis::AnalysisHost,

    base_url: String,
}

impl Default for Config {
    fn default() -> Config {
        let ui = Ui::new(Verbosity::Normal);
        let manifest_path = PathBuf::from("Cargo.toml");
        let host = analysis::AnalysisHost::new(analysis::Target::Debug);

        Config {
            ui,
            manifest_path,
            host,
            output_path: None,
            base_url: String::from(""),
        }
    }
}

fn default_host<'de, D>(_: D) -> ::std::result::Result<analysis::AnalysisHost, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(analysis::AnalysisHost::new(analysis::Target::Debug))
}

impl Config {
    /// Create a new `Config` based off the location of the manifest as well as assets generated
    /// during the build phase
    ///
    /// ## Arguments
    ///
    /// - `manifest_path`: The path to the `Cargo.toml` of the crate being documented
    pub fn new(verbosity: Verbosity, manifest_path: PathBuf, base_url: String) -> Result<Config> {
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
            base_url,
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

    pub fn ui(&self) -> &Ui {
        &self.ui
    }

    pub fn manifest_path(&self) -> &Path {
        &self.manifest_path
    }

    pub fn host(&self) -> &analysis::AnalysisHost {
        &self.host
    }
}