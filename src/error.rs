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
