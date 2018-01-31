extern crate comrak;
#[macro_use]
extern crate configure;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate handlebars;
extern crate rls_analysis as analysis;
extern crate rls_data as analysis_data;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate simple_server;
#[macro_use]
extern crate slog;
extern crate toml_edit;
extern crate walkdir;

mod cargo;
mod config;
mod error;
pub mod examples;
mod git;
pub mod ops;
mod ui;

pub use config::Config;

use failure::Error;

use ui::Verbosity;

type Result<T> = std::result::Result<T, Error>;
