extern crate doxidize;
extern crate tempdir;

use tempdir::TempDir;

#[test]
fn creates_docs_dir() {
    let dir = TempDir::new("create_docs_dir").expect("could not generate temp dir");

    let dir_path = dir.path();

    doxidize::create_skeleton(&dir_path).expect("create_skeleton failed");

    assert!(dir_path.join("docs").is_dir());
}

#[test]
fn creates_root_readme() {
    let dir = TempDir::new("create_root_readme").expect("could not generate temp dir");

    let dir_path = dir.path();

    doxidize::create_skeleton(&dir_path).expect("create_skeleton failed");

    let docs_dir = dir_path.join("docs");
    let readme_path = docs_dir.join("README.md");

    assert!(readme_path.is_file());
}