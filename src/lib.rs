extern crate failure;
#[macro_use]
extern crate failure_derive;

extern crate comrak;

extern crate handlebars;

#[macro_use]
extern crate serde_json;

use comrak::ComrakOptions;

use failure::Error;

use handlebars::Handlebars;

use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

type Result<T> = std::result::Result<T, Error>;

pub fn create_skeleton(dir: &Path) -> Result<()> {
    // create the top-level docs dir
    let docs_dir = dir.join("docs");
    fs::create_dir_all(&docs_dir)?;

    // create a README.md
    let readme = docs_dir.join("README.md");
    OpenOptions::new().create(true).append(true).open(readme)?;

    // create a Menu.toml
    let readme = docs_dir.join("Menu.toml");
    OpenOptions::new().create(true).append(true).open(readme)?;

    Ok(())
}

pub fn generate(dir: &Path) -> Result<()> {
    // we need to know where the docs are
    let docs_dir = dir.join("docs");

    // ensure that the docs dir exists in target
    let target_dir = dir.join("target").join("docs");
    fs::create_dir_all(&target_dir)?;

    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("page", "templates/page.hbs")?;


    // make the README.md render as an index.html
    let readme_path = docs_dir.join("README.md");

    let mut readme = File::open(readme_path)?;
    let mut contents = String::new();
    readme.read_to_string(&mut contents)?;

    let rendered = comrak::markdown_to_html(&contents, &ComrakOptions::default());

    let index_path = target_dir.join("index.html");
    let mut index = File::create(index_path)?;

    index.write_all(handlebars.render("page", &json!({"contents": rendered}))?.as_bytes())?;

    // render all other *.md files as *.html
    for entry in fs::read_dir(docs_dir)? {
        let entry = entry?;
        let path = entry.path();

        // we want only files
        if !path.is_file() { continue; }

        if let Some(extension) = path.extension() {
            // we only want .md files
            if extension != "md" {
                continue;
            }
        } else {
            // we don't want files with no extension
            continue;
        }

        // we certainly have a file name, since we're looping over real files
        let file_name = path.file_name().unwrap();

        // we don't want READMEs
        if file_name == "README.md" {
            continue;
        }

        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let rendered_contents = comrak::markdown_to_html(&contents, &ComrakOptions::default());

        let rendered_path = target_dir.join(file_name).with_extension("html");
        let mut file = File::create(rendered_path)?;

        file.write_all(handlebars.render("page", &json!({"contents": rendered_contents}))?.as_bytes())?;
    }

    Ok(())
}

// adapted from https://github.com/rtomayko/rocco/blob/2586dc3bd4b0e9fa9bd076f492cdbf2924527199/Rakefile#L46
pub fn publish(dir: &Path, target_dir: &Path) -> Result<()> {
    let git_dir = target_dir.join(".git");

    // this name is totally arbitrary, but o was chosen to match rocco's Rakefile above.
    let remote_name = "o";
    /*

    # Update the pages/ directory clone
    file 'docs/.git' => ['docs/', '.git/refs/heads/gh-pages'] do |f|
        sh "cd docs && git init -q && git remote add o ../.git" if !File.exist?(f.name)
        sh "cd docs && git fetch -q o && git reset -q --hard o/gh-pages && touch ."
    end
    */

    // our sub-repo uses 'master' as its branch name, even though it pushes it to a branch
    // named 'gh-pages'. Subtle!
    let pages_head = {
        let mut path = git_dir.join("refs");
        path.push("heads");
        path.push("master");
        path
    };


    // if this file doesn't exist, then we don't have the gh-pages remote set up
    // to set it up we need to initialize the git repositry, add the remote, and sync the two
    if !pages_head.is_file() {
        git::init(&target_dir)?;
        git::initialize_remote(&target_dir, remote_name)?;
        git::reset_to_remote_head(&target_dir, remote_name)?;
    }

    let git_revision = git::head_revision(&dir)?;

    git::add_all(&target_dir)?;
    git::commit(&target_dir, &git_revision)?;

    git::sync_pages_branch(&target_dir, remote_name)?;

    git::push(&dir)?;

    Ok(())
}

mod git {
    use Result;
    use std::fmt;
    use std::path::Path;
    use std::process::{self, Command};

    #[derive(Debug, Fail)]
    pub struct GitFailure {
        command_name: &'static str,
        output: process::Output,
    }

    // stdout/stderr need some massaging, so let's not derive Display, as this way is clearer.
    impl fmt::Display for GitFailure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let stdout = String::from_utf8_lossy(&self.output.stdout);
            let stderr = String::from_utf8_lossy(&self.output.stderr);

            write!(f, "An error occurred while running '{}'\n\nstdout:\n{}\n\nstderr:\n{}", self.command_name, stdout, stderr)
        }
    }

    pub fn init(git_dir: &Path) -> Result<()> {
        let output = Command::new("git")
            .arg("-C")
            .arg(git_dir.as_os_str())
            .arg("init")
            .output()
            .expect("failed to execute git init");

        if !output.status.success() {
            return Err(GitFailure {
                command_name: "git init",
                output: output,
            }.into());
        }

        Ok(())
    }

    pub fn initialize_remote(git_dir: &Path, remote_name: &str) -> Result<()> {
        let output = Command::new("git")
            .arg("-C")
            .arg(git_dir.as_os_str())
            .args(&["remote", "add", remote_name, "../../.git"])
            .output()
            .expect("failed to execute git init");

        if !output.status.success() {
            return Err(GitFailure {
                command_name: "git remote add",
                output: output,
            }.into());
        }

        Ok(())
    }

    pub fn reset_to_remote_head(git_dir: &Path, remote_name: &str) -> Result<()> {
        let output = Command::new("git")
            .arg("-C")
            .arg(git_dir.as_os_str())
            .args(&["fetch", remote_name])
            .output()
            .expect("failed to execute git fetch");

        if !output.status.success() {
            return Err(GitFailure {
                command_name: "git fetch",
                output: output,
            }.into());
        }

        Ok(())
    }

    pub fn head_revision(git_dir: &Path) -> Result<String> {
        let output = Command::new("git")
            .arg("-C")
            .arg(git_dir.as_os_str())
            .args(&["rev-parse", "--short", "HEAD"])
            .output()
            .expect("failed to execute git rev-parse");

        if !output.status.success() {
            return Err(GitFailure {
                command_name: "git rev-parse",
                output: output,
            }.into());
        }

        // we need to strip off the \n
        let mut output = output.stdout;
        output.pop();

        Ok(String::from_utf8(output)?)
    }

    pub fn add_all(git_dir: &Path) -> Result<()> {
        let output = Command::new("git")
            .arg("-C")
            .arg(git_dir.as_os_str())
            .args(&["add", "."])
            .output()
            .expect("failed to execute git add");

        if !output.status.success() {
            return Err(GitFailure {
                command_name: "git add",
                output: output,
            }.into());
        }

        Ok(())
    }

    pub fn commit(git_dir: &Path, revision: &str) -> Result<()> {
        let output = Command::new("git")
            .arg("-C")
            .arg(git_dir.as_os_str())
            .args(&["commit", "-m"])
            .arg(format!("\"rebuild pages from {}\"", revision))
            .output()
            .expect("failed to execute git commit");

        if !output.status.success() {
            return Err(GitFailure {
                command_name: "git commit",
                output: output,
            }.into());
        }

        Ok(())
    }

    pub fn sync_pages_branch(git_dir: &Path, remote_name: &str) -> Result<()> {
        let output = Command::new("git")
            .arg("-C")
            .arg(git_dir.as_os_str())
            .arg("push")
            .arg(remote_name)
            .arg("HEAD:gh-pages")
            .output()
            .expect("failed to execute git push");

        if !output.status.success() {
            return Err(GitFailure {
                command_name: "git push",
                output: output,
            }.into());
        }

        Ok(())
    }

    pub fn push(git_dir: &Path) -> Result<()> {
        let output = Command::new("git")
            .arg("-C")
            .arg(git_dir.as_os_str())
            .args(&["push", "origin", "gh-pages"])
            .output()
            .expect("failed to execute git push");

        if !output.status.success() {
            return Err(GitFailure {
                command_name: "git push",
                output: output,
            }.into());
        }

        Ok(())
    }
}