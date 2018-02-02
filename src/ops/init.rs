use analysis::{self, DefKind};
use handlebars::{self, Handlebars};
use slog::Logger;

use std::collections::{VecDeque, HashSet};
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;

use cargo::{self, Target};
use Config;
use error;
use Result;
use strip_leading_space;

pub fn init(config: &Config, log: &Logger) -> Result<()> {
    let log = log.new(o!("command" => "create_skeleton"));
    info!(log, "starting");

    let mut handlebars = Handlebars::new();

    debug!(log, "loading handlebars templates");

    handlebars.register_template_file("example", "templates/markdown/example.hbs")?;
    handlebars.register_template_file("page", "templates/html/page.hbs")?;
    handlebars.register_template_file("api", "templates/markdown/api.hbs")?;
    handlebars.register_template_file("mod", "templates/markdown/mod.hbs")?;
    handlebars.register_template_file("struct", "templates/markdown/struct.hbs")?;
    handlebars.register_template_file("enum", "templates/markdown/enum.hbs")?;
    handlebars.register_template_file("trait", "templates/markdown/trait.hbs")?;
    handlebars.register_template_file("function", "templates/markdown/function.hbs")?;
    handlebars.register_template_file("type", "templates/markdown/type.hbs")?;
    handlebars.register_template_file("static", "templates/markdown/static.hbs")?;
    handlebars.register_template_file("const", "templates/markdown/const.hbs")?;

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

    // create the top-level docs dir
    let docs_dir = config.root_path().join("docs");
    debug!(log, "creating top-level docs dir"; o!("dir" => docs_dir.display()));
    fs::create_dir_all(&docs_dir)?;

    // create a README.md
    let readme = docs_dir.join("README.md");
    debug!(log, "creating README"; o!("file" => readme.display()));
    OpenOptions::new().create(true).append(true).open(readme)?;

    // create a Doxidize.toml & Menu.toml
    let doxidize_config = config.root_path().join("Doxidize.toml");
    debug!(log, "creating Doxidize.toml"; o!("file" => doxidize_config.display()));
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(doxidize_config)?;

    let menu = docs_dir.join("Menu.toml");
    debug!(log, "creating Menu.toml"; o!("file" => menu.display()));
    OpenOptions::new().create(true).append(true).open(menu)?;

    // next up: examples!
    let examples_dir = docs_dir.join("examples");
    debug!(log, "creating examples dir"; o!("dir" => examples_dir.display()));
    fs::create_dir_all(&examples_dir)?;

    if config.examples_path().is_dir() {
        for entry in fs::read_dir(config.examples_path())? {
            let entry = entry?;
            let path = entry.path();

            // we want only files
            if !path.is_file() {
                continue;
            }
            trace!(log, "file is a file, continuing");

            if let Some(extension) = path.extension() {
                // we only want .rs files
                if extension != "rs" {
                    continue;
                }
            } else {
                // we don't want files with no extension
                continue;
            }
            trace!(log, "file is a rust file, continuing");

            // we certainly have a file name, since we're looping over real files
            let file_name = path.file_name().unwrap();
            let rust_file = config.examples_path().join(file_name);

            trace!(log, "reading file"; "file" => rust_file.display());
            let mut file = File::open(&rust_file)?;
            let mut code = String::new();
            file.read_to_string(&mut code)?;

            let markdown_path = examples_dir.join(file_name).with_extension("md");

            trace!(log, "rendering to markdown"; "file" => path.display(), "file" => markdown_path.display());
            let mut file = File::create(markdown_path)?;

            file.write_all(
                handlebars
                    .render(
                        "example",
                        &json!({"name": file_name.to_str().unwrap(), "code": code}),
                    )?
                    .as_bytes(),
            )?;
        }
    }

    // now the api docs

    // ensure that the api dir exists
    let api_dir = docs_dir.join("api");
    debug!(log, "creating api dir"; o!("dir" => api_dir.display()));
    fs::create_dir_all(&api_dir)?;

    let metadata = cargo::retrieve_metadata(config.manifest_path())?;
    let target = cargo::target_from_metadata(config.ui(), &metadata)?;

    generate_and_load_analysis(&config, &target, &log)?;

    let host = config.host();
    let crate_name = &target.crate_name();

    let roots = host.def_roots()?;

    // we want to keep track of all modules for the module overview page
    let mut module_set = HashSet::new();

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

    debug!(log, "creating README.md for api"; o!("file" => markdown_path.display()));
    let mut file = File::create(markdown_path)?;

    file.write_all(
        handlebars
            .render("api", &json!({"name": crate_name, "docs": strip_leading_space(&root_def.docs)}))?
            .as_bytes(),
    )?;

    let ids = host.for_each_child_def(root_id, |id, _def| id).unwrap();

    info!(log, "turning analysis into markdown");
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

    while let Some(id) = queue.pop_front() {
        host.for_each_child_def(id, |id, _def| {
            queue.push_back(id);
        })?;

        // Question: we could do this by cloning it in the call to for_each_child_def
        // above/below; is that cheaper, or is this cheaper?
        let def = host.get_def(id).unwrap();

        // if this def is a module, push its id onto the modules list for later
        match def.kind {
            DefKind::Mod => {
                module_set.insert(id);
            },
            _ => (),
        }

        let template_name = match def.kind {
            DefKind::Mod => "mod",
            DefKind::Struct => "struct",
            DefKind::Enum => "enum",
            DefKind::Trait => "trait",
            DefKind::Function => "function",
            DefKind::Type => "type",
            DefKind::Static => "static",
            DefKind::Const => "const",
            DefKind::Field => continue,
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
                .render(template_name, &json!({"name": def.name, "docs": strip_leading_space(&def.docs)}))?
                .as_bytes(),
        )?;
    }

    // now, time for modules:

    #[derive(Debug)]
    struct Module {
        id: analysis::Id,
        children: Vec<Module>,
    }

    let mut krate = Module {
        id: root_id,
        children: Vec::new(),
    };


    // is our call stack smaller than the module depth? hopefully! this is good enough for now
    fn add_children(parent: &mut Module, possible_children: &HashSet<analysis::Id>, host: &analysis::AnalysisHost) {
        let children: Vec<&analysis::Id> = possible_children.iter().filter(|child| {
            let def = host.get_def(**child).unwrap();
            def.parent == Some(parent.id)
        }).collect();

        // the base case!
        if children.is_empty() {
            return;
        }

        for child in children {
            let mut module = Module {
                id: *child,
                children: Vec::new(),
            };

            add_children(&mut module, possible_children, host);

            parent.children.push(module);
        }
    }

    add_children(&mut krate, &module_set, &host);

    // time to write out the markdown

    let markdown_path = api_dir.join("module-overview.md");

    let mut file = File::create(markdown_path)?;

    file.write_all("# Module overview\n\n".as_bytes())?;

    fn print_tree(node: &Module, depth: usize, host: &analysis::AnalysisHost, file: &mut File) {
        let def = host.get_def(node.id).unwrap();

        let name = if def.name.is_empty() {
            "doxidize".to_string()
        } else {
            def.name
        };
        
        let line = format!("{}* {}\n", ::std::iter::repeat("  ").take(depth).collect::<Vec<_>>().join(""), name);
        file.write_all(line.as_bytes()).unwrap();

        if node.children.is_empty() {
            return;
        }

        for child in &node.children {
            print_tree(child, depth + 1, host, file);
        }
    }

    print_tree(&krate, 0, &host, &mut file);

    info!(log, "done");
    Ok(())
}

/// Generate save analysis data of a crate to be used later by the RLS library later and load it
/// into the analysis host.
fn generate_and_load_analysis(config: &Config, target: &Target, log: &Logger) -> Result<()> {
    let log = log.new(o!("command" => "generate_and_load_analysis"));
    info!(log, "analysizing your source code");

    cargo::generate_analysis(config, target)?;

    let root_path = config.root_path();
    debug!(log, "analysis complete, loading");
    config.host().reload(root_path, root_path)?;

    info!(log, "done");
    Ok(())
}
