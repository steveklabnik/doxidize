extern crate failure;
#[macro_use]
extern crate failure_derive;

extern crate comrak;

extern crate handlebars;

extern crate rls_analysis as analysis;
extern crate rls_data as analysis_data;

#[macro_use]
extern crate serde_json;

extern crate simple_server;

extern crate walkdir;

mod cargo;
mod error;
mod git;
mod ui;

use analysis::DefKind;

use comrak::ComrakOptions;

use failure::Error;

use handlebars::Handlebars;

use simple_server::Server;

use std::collections::VecDeque;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use cargo::Target;

use ui::{Ui, Verbosity};

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
    let target_dir = dir.join("target").join("docs").join("public");
    fs::create_dir_all(&target_dir)?;

    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("page", "templates/page.hbs")?;
    handlebars.register_template_file("api", "templates/api.hbs")?;
    handlebars.register_helper("up-dir",
        Box::new(|h: &handlebars::Helper, _: &Handlebars, rc: &mut handlebars::RenderContext| -> handlebars::HelperResult {
            let count = h.param(0).map(|v| v.value().as_u64().unwrap()).unwrap();

            for _ in 0..count {
                rc.writer.write(b"../")?;
            }

            Ok(())
      }));


    // make the README.md render as an index.html
    let readme_path = docs_dir.join("README.md");

    let mut readme = File::open(readme_path)?;
    let mut contents = String::new();
    readme.read_to_string(&mut contents)?;

    let rendered = comrak::markdown_to_html(&contents, &ComrakOptions::default());

    let index_path = target_dir.join("index.html");
    let mut index = File::create(index_path)?;

    index.write_all(handlebars.render("page", &json!({"contents": rendered, "nest-count": 0}))?.as_bytes())?;

    // render all other *.md files as *.html, walking the tree
    for entry in WalkDir::new(&docs_dir) {
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

        // make sure the containing directory is created
        //
        // to do this, we get the containing directory, strip off the base, and then re-apply that path
        // to where we want to write the output. tricky!
        let containing_dir = path.parent().expect("somehow this is running at the root");
        let new_containing_dir = containing_dir.strip_prefix(&docs_dir)?;
        let new_containing_dir = target_dir.join(new_containing_dir);
        fs::create_dir_all(&new_containing_dir)?;

        // now we can make the file
        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let rendered_contents = comrak::markdown_to_html(&contents, &ComrakOptions::default());

        let rendered_path = new_containing_dir.join(file_name).with_extension("html");
        let mut file = File::create(rendered_path)?;

        // TODO: this only works at one level down; we need to calculate the real number
        file.write_all(handlebars.render("page", &json!({"contents": rendered_contents, "nest-count": 1}))?.as_bytes())?;
    }

    // now it's time for api docs

    // ensure that the api dir exists
    let api_dir = docs_dir.join("api");
    fs::create_dir_all(&api_dir)?;

    let manifest_path = PathBuf::from("Cargo.toml");
    let verbosity = Verbosity::Normal;

    let config = Config::new(verbosity, manifest_path)?;

    let metadata = cargo::retrieve_metadata(&config.manifest_path)?;
    let target = cargo::target_from_metadata(&config.ui, &metadata)?;

    generate_and_load_analysis(&config, &target)?;

    let host = &config.host;
    let crate_name = &target.crate_name();

    // This function does a lot, so here's the plan:
    //
    // First, we need to process the root def and get its list of children.
    // Then, we process all of the children. Children may produce more children
    // to be processed too. Once we've processed them all, we're done.

    // Step one: we need to get all of the "def roots", and then find the
    // one that's our crate.
    let roots = host.def_roots()?;

    let id = roots.iter().find(|&&(_, ref name)| name == crate_name);
    let root_id = match id {
        Some(&(id, _)) => id,
        _ => {
            return Err(error::CrateErr {
                crate_name: crate_name.to_string(),
            }.into())
        }
    };

    let root_def = host.get_def(root_id)?;

    let markdown_path = api_dir.join("README.md");

    let mut file = File::create(markdown_path)?;

    file.write_all(handlebars.render("api", &json!({"name": crate_name, "docs": root_def.docs}))?.as_bytes())?;

    // Now that we have that, it's time to get the children; these are
    // the top-level items for the crate.
    let ids = host.for_each_child_def(root_id, |id, _def| id).unwrap();

    // Now, we push all of those children onto a channel. The channel functions
    // as a work queue; we take an item off, process it, and then if it has
    // children, push them onto the queue. When the queue is empty, we've processed
    // everything.
    //
    // Additionally, we generate relationships between the crate itself and
    // these ids, as they're at the top level and hence linked with the crate.

    let mut queue = VecDeque::new();

    for id in ids {
        queue.push_back(id);

        let def = host.get_def(id).unwrap();

        match def.kind {
            DefKind::Mod => (),
            DefKind::Struct => (),
            DefKind::Enum => (),
            DefKind::Trait => (),
            DefKind::Function => (),
            DefKind::Type => (),
            DefKind::Static => (),
            DefKind::Const => (),
            DefKind::Field => (),
            DefKind::Tuple => continue,
            DefKind::Local => continue,
            // The below DefKinds are not supported in rls-analysis
            // DefKind::Union => (String::from("union"), String::from("unions")),
            // DefKind::Macro => (String::from("macro"), String::from("macros")),
            // DefKind::Method => (String::from("method"), String::from("methods")),
            _ => continue,
        };

    }

    // The loop below is basically creating this vector.
    while let Some(id) = queue.pop_front() {
        // push each child to be processed itself, and also record
        // their ids so we can create the relationships for later
        host.for_each_child_def(id, |id, _def| {
            queue.push_back(id);
        })?;

        // Question: we could do this by cloning it in the call to for_each_child_def
        // above/below; is that cheaper, or is this cheaper?
        let def = host.get_def(id).unwrap();

        // Using the item's metadata we create a new `Document` type to be put in the eventual
        // serialized JSON.
        match def.kind {
            DefKind::Mod => (),
            DefKind::Struct => (),
            DefKind::Enum => (),
            DefKind::Trait => (),
            DefKind::Function => (),
            DefKind::Type => (),
            DefKind::Static => (),
            DefKind::Const => (),
            DefKind::Field => (),
            DefKind::Tuple => continue,
            DefKind::Local => continue,
            // The below DefKinds are not supported in rls-analysis
            // DefKind::Union => (String::from("union"), String::from("unions")),
            // DefKind::Macro => (String::from("macro"), String::from("macros")),
            // DefKind::Method => (String::from("method"), String::from("methods")),
            _ => continue,
        };

        let markdown_path = api_dir.join(&format!("{}.md", def.name));

        let mut file = File::create(markdown_path)?;

        file.write_all(handlebars.render("api", &json!({"name": def.name, "docs": def.docs}))?.as_bytes())?;
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

pub fn serve(directory: &Path) -> Result<()> {
    let host = "127.0.0.1";
    let port = "7878";

    // everything is handled by the static serving, so any request here is
    // an error
    let server = Server::new(|_request, mut response| {
        Ok(response.body("incorrect path".as_bytes())?)
    });

    env::set_current_dir(directory)?;

    println!("serving docs at http://{}:{}", host, port);

    server.listen(host, port);

    Ok(())
}

/// A structure that contains various fields that hold data in order to generate doc output.
#[derive(Debug)]
pub struct Config {
    /// Interactions with the user interface.
    ui: Ui,

    /// Path to the `Cargo.toml` file for the crate being analyzed
    manifest_path: PathBuf,

    /// Path to place rustdoc output
    output_path: Option<PathBuf>,

    /// Contains the Cargo analysis output for the crate being documented
    host: analysis::AnalysisHost,
}

impl Config {
    /// Create a new `Config` based off the location of the manifest as well as assets generated
    /// during the build phase
    ///
    /// ## Arguments
    ///
    /// - `manifest_path`: The path to the `Cargo.toml` of the crate being documented
    pub fn new(verbosity: Verbosity, manifest_path: PathBuf) -> Result<Config> {
        let host = analysis::AnalysisHost::new(analysis::Target::Debug);

        if !manifest_path.is_file() || !manifest_path.ends_with("Cargo.toml") {
            return Err(failure::err_msg(
                "The --manifest-path must be a path to a Cargo.toml file",
            ));
        }

        Ok(Config {
            ui: Ui::new(verbosity),
            manifest_path,
            output_path: None,
            host,
        })
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
            None => self.root_path().join("target").join("doc"),
        }
    }

    /// Set the directory where output files should be placed
    pub fn set_output_path(&mut self, output_path: PathBuf) {
        self.output_path = Some(output_path);
    }

    /// Returns the path to the generated documentation.
    pub fn documentation_path(&self) -> PathBuf {
        self.output_path().join("data.json")
    }
}

/// Generate save analysis data of a crate to be used later by the RLS library later and load it
/// into the analysis host.
///
/// ## Arguments:
///
/// - `config`: Contains data for what needs to be output or used. In this case the path to the
///             `Cargo.toml` file
/// - `target`: The target to document
fn generate_and_load_analysis(config: &Config, target: &Target) -> Result<()> {
    let analysis_result = cargo::generate_analysis(config, target, |_| {
    });

    if analysis_result.is_err() {
        return analysis_result;
    }

    let root_path = config.root_path();
    config.host.reload(root_path, root_path)?;

    Ok(())
}