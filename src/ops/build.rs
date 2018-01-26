use Result;
use std::path::Path;
use std::fs::{self, File};
use handlebars::{self, Handlebars};
use walkdir::WalkDir;
use comrak::{self, ComrakOptions};
use std::io::prelude::*;
use toml_edit;
use config::Config;

pub fn build(dir: &Path, config: &Config) -> Result<()> {
    // load up our Doxidize.toml so we can handle any base urls
    let path = dir.join("Doxidize.toml");
    let mut contents = String::new();
    let mut toml_file = File::open(path)?;
    toml_file.read_to_string(&mut contents)?;

    let doc = contents.parse::<toml_edit::Document>().expect("invalid doxidize.toml");

    let base_url = doc["docs"]["base-url"].as_value().map(|v| v.as_str().expect("value of base-url was not a string")).unwrap_or_default().to_string();

    // we need to know where the docs are
    let docs_dir = dir.join("docs");

    // ensure that the docs dir exists in target
    let mut target_dir = dir.join("target").join("docs").join("public");

    // keep track of how far we're nested
    let mut base_nesting_count = 0;

    // if we have a base_url, we need to push that on
    if !base_url.is_empty() {
        base_nesting_count += 1;
        target_dir.push(&base_url);
    }

    fs::create_dir_all(&target_dir)?;

    // finally, we need to tag a `/` on so that it's added automatically in the output
    let base_url = base_url + "/";

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

    // render all other *.md files as *.html, walking the tree
    for entry in WalkDir::new(&docs_dir) {
        let entry = entry?;
        let path = entry.path();
        let mut nesting_count = base_nesting_count;

        // we want only files
        if !path.is_file() {
            continue;
        }

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

        let rendered_path = if file_name == "README.md" {
            new_containing_dir.join("index.html")
        } else {
            new_containing_dir.join(file_name).with_extension("html")
        };

        let mut file = File::create(rendered_path)?;

        // how many levels deep are we?
        let containing_dir = path.parent().expect("somehow this is running at the root");
        let new_containing_dir = containing_dir.strip_prefix(&docs_dir)?;
        let component_count = new_containing_dir.components().count();

        nesting_count += component_count;

        file.write_all(
            handlebars
                .render(
                    "page",
                    &json!({"contents": rendered_contents, "nest-count": nesting_count, "base-url": base_url }),
                )?
                .as_bytes(),
        )?;
    }

    Ok(())
}
