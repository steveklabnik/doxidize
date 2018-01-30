use slog;
use slog_term;
use slog_async;

use std::error::Error;
use std::path::Path;
use std::process::Command;

pub fn cargo_init(path: &Path) -> Result<(), Box<Error>> {
    let output = Command::new("cargo")
        .args(&["init", "--name", "example"])
        .current_dir(path)
        .output()
        .expect("failed to execute cargo init");

    if !output.status.success() {
        return Err(format!(
            "couldn't cargo init:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ).into());
    }

    Ok(())
}

/// by default we suppress all logging output
pub fn make_logger() -> slog::Logger {
    // use this if you want to enable it
    /*
    use slog::Drain;

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, o!())
    */

    slog::Logger::root(slog::Discard, o!())
}