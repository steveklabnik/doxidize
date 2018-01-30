use analysis;
use ui::{Ui, Verbosity};
use std::path::{Path, PathBuf};
use std::default::Default;
use serde::Deserializer;
use toml_edit;
use std::fs::File;
use std::io::prelude::*;

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

        let config_path = PathBuf::from("Doxidize.toml");
        let mut contents = String::new();

        let base_url = (|| {
            let mut toml_file = File::open(config_path)?;
            toml_file.read_to_string(&mut contents)?;
            let doc = contents.parse::<toml_edit::Document>()?;

            Ok((|| {
                let value = doc["docs"]["base-url"].as_value()?;
                let value = value.as_str()?;
                Some(value.to_string())
            })()
                .ok_or("")?)
        })()
            .unwrap_or_else(|_: Box<::std::error::Error>| String::from(""));

        Config {
            ui,
            manifest_path,
            host,
            output_path: None,
            base_url,
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
    pub fn with_manifest_path(manifest_path: PathBuf) -> Config {
        let ui = Ui::new(Verbosity::Normal);
        let host = analysis::AnalysisHost::new(analysis::Target::Debug);

        let config_path = manifest_path.parent().unwrap().join("Doxidize.toml");
        let mut contents = String::new();

        let base_url = (|| {
            let mut toml_file = File::open(config_path)?;
            toml_file.read_to_string(&mut contents)?;
            let doc = contents.parse::<toml_edit::Document>()?;

            Ok((|| {
                let value = doc["docs"]["base-url"].as_value()?;
                let value = value.as_str()?;
                Some(value.to_string())
            })()
                .ok_or("")?)
        })()
            .unwrap_or_else(|_: Box<::std::error::Error>| String::from(""));

        Config {
            ui,
            manifest_path,
            host,
            output_path: None,
            base_url,
        }
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
            None => self.root_path().join("target").join("docs"),
        }
    }

    /// Set the directory where output files should be placed
    pub fn set_output_path(&mut self, output_path: PathBuf) {
        self.output_path = Some(output_path);
    }

    pub fn examples_path(&self) -> PathBuf {
        self.root_path().join("examples")
    }

    pub fn ui(&self) -> &Ui {
        &self.ui
    }

    pub fn manifest_path(&self) -> &Path {
        &self.manifest_path
    }

    pub fn set_manifest_path(&mut self, path: PathBuf) {
        self.manifest_path = path;
    }

    pub fn host(&self) -> &analysis::AnalysisHost {
        &self.host
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}
