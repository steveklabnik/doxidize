extern crate doxidize;
extern crate tempdir;

use tempdir::TempDir;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

#[test]
fn generate_renders_readme() {
    let dir = TempDir::new("create_root_readme").expect("could not generate temp dir");

    let dir_path = dir.path();

    doxidize::create_skeleton(&dir_path).expect("create_skeleton failed");

    let docs_dir = dir_path.join("docs");
    let readme_path = docs_dir.join("README.md");

    let mut readme = OpenOptions::new().create(true).append(true).open(readme_path).expect("could not open README file");

    readme.write_all(b"# Testing

testing").expect("could not write to README");

    doxidize::generate(&dir_path).expect("generate failed");

    let mut output_dir = dir_path.join("target");
    output_dir.push("docs");

    let rendered_readme_path = output_dir.join("index.html");

    let mut rendered_readme = File::open(rendered_readme_path).expect("could not open rendered README");

    let mut contents = String::new();
    rendered_readme.read_to_string(&mut contents).expect("could not read README");

    assert_eq!(contents, "<h1>Testing</h1>
<p>testing</p>
");
}

#[test]
fn generate_renders_additional_markdown_files() {
    let dir = TempDir::new("create_additional_markdown").expect("could not generate temp dir");

    let dir_path = dir.path();

    doxidize::create_skeleton(&dir_path).expect("create_skeleton failed");

    let docs_dir = dir_path.join("docs");
    let guide_path = docs_dir.join("guide.md");

    let mut guide = OpenOptions::new().create(true).append(true).open(guide_path).expect("could not open guide file");

    guide.write_all(b"# Testing

testing").expect("could not write to guide");

    doxidize::generate(&dir_path).expect("generate failed");

    let mut output_dir = dir_path.join("target");
    output_dir.push("docs");

    let rendered_guide_path = output_dir.join("guide.html");

    let mut rendered_guide = File::open(rendered_guide_path).expect("could not open rendered guide");

    let mut contents = String::new();
    rendered_guide.read_to_string(&mut contents).expect("could not read rendered_guide");

    assert_eq!(contents, "<h1>Testing</h1>
<p>testing</p>
");
}