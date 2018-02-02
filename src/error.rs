use std::fmt;
use std::path::PathBuf;

/// Thrown whenever Cargo fails to run properly when getting data for `rustdoc`
#[derive(Debug, Fail)]
#[fail(display = "Cargo failed with status {}. stderr:\n{}", status, stderr)]
pub struct Cargo {
    /// The status Cargo gave us
    pub status: ::std::process::ExitStatus,
    /// The contents of Cargo's stderr
    pub stderr: String,
}

/// Thrown whenever a crate cannot be found
#[derive(Debug, Fail)]
#[fail(display = "Crate not found: \"{}\"", crate_name)]
pub struct CrateErr {
    /// The name of the crate that couldn't be found
    pub crate_name: String,
}

/// Thrown whenever the `JSON` grabbed from somewhere else is not what is expected.
/// This is usually thrown when grabbing data output from `Cargo`
#[derive(Debug, Fail)]
#[fail(display = "Unexpected JSON response from {}", location)]
pub struct Json {
    /// The location of the unexpected JSON
    pub location: String,
}

/// An error when a command is run on a project that wasn't initalized for use with Doxidize.
#[derive(Debug, Fail)]
pub struct UninitializedProject {
    /// the path for the project that wasn't initialized
    pub location: PathBuf,
}

// we have to impl Display manually for UninitializedProject because we want to
// call .display() on the PathBuf, but that has a lifetime parameter, so the
// attribute doesn't work.
impl fmt::Display for UninitializedProject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Package at {} doesn't look like it's ready for doxidize; consider `doxidize init` instead of `doxidize build`", self.location.display())
    }
}
