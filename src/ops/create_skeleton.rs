use Result;
use std::path::Path;
use std::fs::{self, OpenOptions};
use Config;
use cargo::{self, Target};
use handlebars::{self, Handlebars};
use analysis::DefKind;
use std::fs::File;
use std::collections::VecDeque;
use error;
use std::io::prelude::*;
use ui::Verbosity;

pub fn create_skeleton(dir: &Path) -> Result<()> {
    // create the top-level docs dir
    let docs_dir = dir.join("docs");
    fs::create_dir_all(&docs_dir)?;

    // create a README.md
    let readme = docs_dir.join("README.md");
    OpenOptions::new().create(true).append(true).open(readme)?;

    // create a Doxidize.toml & Menu.toml
    let config = dir.join("Doxidize.toml");
    OpenOptions::new().create(true).append(true).open(config)?;

    let menu = docs_dir.join("Menu.toml");
    OpenOptions::new().create(true).append(true).open(menu)?;

    // now the api docs
    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("page", "templates/page.hbs")?;
    handlebars.register_template_file("api", "templates/api.hbs")?;
    handlebars.register_helper(
        "up-dir",
        Box::new(
            |h: &handlebars::Helper,
             _: &Handlebars,
             rc: &mut handlebars::RenderContext|
             -> handlebars::HelperResult {
                let count = h.param(0).map(|v| v.value().as_u64().unwrap()).unwrap();

                for _ in 0..count {
                    rc.writer.write(b"../")?;
                }

                Ok(())
            },
        ),
    );

    // ensure that the api dir exists
    let api_dir = docs_dir.join("api");
    fs::create_dir_all(&api_dir)?;

    let manifest_path = dir.join("Cargo.toml");
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

    file.write_all(
        handlebars
            .render("api", &json!({"name": crate_name, "docs": root_def.docs}))?
            .as_bytes(),
    )?;

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

        file.write_all(
            handlebars
                .render("api", &json!({"name": def.name, "docs": def.docs}))?
                .as_bytes(),
        )?;
    }

    Ok(())
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
    let analysis_result = cargo::generate_analysis(config, target, |_| {});

    if analysis_result.is_err() {
        return analysis_result;
    }

    let root_path = config.root_path();
    config.host.reload(root_path, root_path)?;

    Ok(())
}
