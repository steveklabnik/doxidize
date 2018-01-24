extern crate failure;
#[macro_use]
extern crate failure_derive;

extern crate comrak;

#[macro_use]
extern crate configure;

extern crate handlebars;

extern crate rls_analysis as analysis;
extern crate rls_data as analysis_data;

extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate simple_server;

extern crate walkdir;

mod cargo;
mod config;
mod error;
mod git;
pub mod ops;
mod ui;

pub use config::Config;

use failure::Error;

use ui::Verbosity;

type Result<T> = std::result::Result<T, Error>;

