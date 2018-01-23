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
        return Err(format!("couldn't cargo init:\n{}", String::from_utf8_lossy(&output.stderr)).into());
    }

    Ok(())
}
