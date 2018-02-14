use comrak::{self, ComrakOptions};
use serde_json;
use slog::Logger;
use slugify::slugify;
use toml_edit;
use walkdir::{DirEntry, WalkDir};

use std::fs::{self, File};
use std::io::prelude::*;

use cargo;
use config::Config;
use error;
use Result;

/// metadata for each file we need to process
struct DocMarkdown {
    id: String,
    title: String,
    target: String,

    entry: DirEntry,
}

pub fn build(config: &Config, log: &Logger) -> Result<()> {
    let log = log.new(o!("command" => "build"));
    info!(log, "starting");

    // we need to know where the docs are
    let docs_dir = config.markdown_path();

    if !docs_dir.is_dir() {
        return Err(error::UninitializedProject {
            location: config.root_path().to_path_buf(),
            command: "build",
        }.into());
    }

    // ensure that the docs dir exists in target
    let mut target_dir = config.public_path();

    // keep track of how far we're nested
    let mut base_nesting_count = 0;

    // if we have a base_url, we need to push that on
    if !config.base_url().is_empty() {
        base_nesting_count += 1;
        target_dir.push(config.base_url());
    }

    let metadata = cargo::retrieve_metadata(config.manifest_path())?;
    let target = cargo::target_from_metadata(&log, &metadata)?;

    debug!(log, "creating target directory";
    "dir" => target_dir.display());
    fs::create_dir_all(&target_dir)?;

    // finally, we need to tag a `/` on so that it's added automatically in the output
    let base_url = format!("{}/", config.base_url());

    let entries = collect_files_to_process(config, &log)?;

    let menu = create_menu(config, &log, &entries)?;

    for doc_markdown in entries {
        let path = doc_markdown.entry.path();
        let mut nesting_count = base_nesting_count;

        trace!(log, "processing file";
        o!("path" => path.display(), "id" => doc_markdown.id.clone(), "nesting_count" => nesting_count));

        // we certainly have a file name, since we're looping over real files
        let file_name = path.file_name().unwrap();

        // make sure the containing directory is created
        //
        // to do this, we get the containing directory, strip off the base, and then re-apply that path
        // to where we want to write the output. tricky!
        let containing_dir = path.parent().expect("somehow this is running at the root");
        let new_containing_dir = containing_dir.strip_prefix(&docs_dir)?;
        let new_containing_dir = target_dir.join(new_containing_dir);
        trace!(log, "creating containing directory"; "dir" => new_containing_dir.display());
        fs::create_dir_all(&new_containing_dir)?;

        // now we can make the file
        let mut file = File::open(&path)?;
        let mut contents = String::new();
        trace!(log, "reading file"; "file" => path.display());
        file.read_to_string(&mut contents)?;

        let mut options = ComrakOptions::default();
        options.default_info_string = Some(String::from("rust"));

        let contents = if &contents[0..3] == "---" {
            if let Some(index) = contents[3..].find("---") {
                // three for the offset, four for ---\n
                &contents[(index + 3 + 4)..]
            } else {
                panic!("found opening --- but couldn't find closing ---");
            }
        } else {
            &contents[..]
        };

        let rendered_contents = comrak::markdown_to_html(&contents, &options);

        let rendered_path = if file_name == "README.md" {
            new_containing_dir.join("index.html")
        } else {
            new_containing_dir.join(file_name).with_extension("html")
        };

        trace!(log, "rendering to html"; "file" => path.display(), "rendered file" => rendered_path.display());
        let mut file = File::create(rendered_path)?;

        // how many levels deep are we?
        let containing_dir = path.parent().expect("somehow this is running at the root");
        let new_containing_dir = containing_dir.strip_prefix(&docs_dir)?;
        let component_count = new_containing_dir.components().count();

        nesting_count += component_count;

        trace!(log, "writing rendered file");
        file.write_all(
            config
                .handlebars()
                .render(
                    "page",
                    &json!({
                        "contents": rendered_contents,
                        "nest-count": nesting_count,
                        "base-url": base_url,
                        "menu": menu,
                        "title": doc_markdown.title.clone(),
                        "site-title": target.name.clone()
                    }),
                )?
                .as_bytes(),
        )?;
    }

    info!(log, "done");
    Ok(())
}

fn create_menu(
    config: &Config,
    log: &Logger,
    files: &[DocMarkdown],
) -> Result<Vec<serde_json::Value>> {
    debug!(log, "reading Menu.toml");

    let mut contents = String::new();
    let mut toml_file = File::open(config.menu_path())?;
    toml_file.read_to_string(&mut contents)?;
    let doc = contents.parse::<toml_edit::Document>()?;

    let mut json = Vec::new();

    for (title, body) in doc.iter() {
        let mut body_contents = Vec::new();

        let body = body.as_value()
            .expect("toml is misformatted; body is not a value");
        let body = body.as_array()
            .expect("toml is misformatted, body is not an array");

        for id in body.iter() {
            let id = id.as_str()
                .expect("toml is misformatted, title is not a string");

            let file = files
                .iter()
                .find(|f| f.id == id)
                .expect("could not find file with this id");

            body_contents.push(json!({
                    "title": file.title,
                    "id": file.id,
                    "target": file.target,
                }));
        }

        json.push(json!({
                "title": title,
                "slug": slugify!(title),
                "contents": body_contents,
            }));
    }

    Ok(json)
}

fn collect_files_to_process(config: &Config, log: &Logger) -> Result<Vec<DocMarkdown>> {
    let docs_dir = config.markdown_path();

    debug!(log, "walking directory tree to render files"; "dir" => docs_dir.display());

    // render all other *.md files as *.html, walking the tree
    let mut entries = Vec::new();

    for entry in WalkDir::new(&docs_dir) {
        let entry = entry?;
        let path = entry.path();

        trace!(log, "processing file";
        o!("path" => path.display()));

        // we want only files
        if !path.is_file() {
            continue;
        }
        trace!(log, "file is a file, continuing");

        if let Some(extension) = path.extension() {
            // we only want .md files
            if extension != "md" {
                continue;
            }
        } else {
            // we don't want files with no extension
            continue;
        }
        trace!(log, "file is a markdown file, continuing");

        // if we're looking at api docs, just create dummy metadata; we don't use it
        if path.starts_with(&config.api_markdown_path()) {
            entries.push(DocMarkdown {
                id: String::new(),
                title: String::new(),
                target: String::new(),
                entry,
            });

            continue;
        }

        // time read out the metadata

        let mut file = File::open(path)?;

        // gonna be lazy and read it all to a string. can be fixed in the future if it matters
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        if &contents[0..3] != "---" {
            panic!("for now you need an explicit --- block in your files");
        }

        let metadata = if let Some(index) = contents[3..].find("---") {
            &contents[3..(index + 3)]
        } else {
            panic!("you need a closing --- for the metadata");
        };

        let doc = metadata.parse::<toml_edit::Document>()?;
        let id = doc["id"].as_str().expect("id must be a string").to_string();

        let title = doc["title"]
            .as_str()
            .expect("title must be a string")
            .to_string();

        // we certainly have a file name, since we're looping over real files
        let file_name = path.file_name().unwrap();

        let containing_dir = path.parent().expect("somehow this is running at the root");
        let new_containing_dir = containing_dir
            .strip_prefix(&docs_dir)
            .expect("couldn't strip prefix");

        let target = if file_name == "README.md" {
            new_containing_dir.join("index.html")
        } else {
            new_containing_dir.join(file_name).with_extension("html")
        };

        let target = target.to_string_lossy().into_owned();

        entries.push(DocMarkdown {
            id,
            title,
            target,
            entry,
        });
    }

    Ok(entries)
}
