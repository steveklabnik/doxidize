use comrak::{self, ComrakOptions};
use handlebars::{self, Handlebars};
use slog::Logger;
use walkdir::WalkDir;

use std::fs::{self, File};
use std::io::prelude::*;

use config::Config;
use Result;

pub fn build(config: &Config, log: &Logger) -> Result<()> {
    let log = log.new(o!("command" => "build"));
    info!(log, "starting");

    // we need to know where the docs are
    let docs_dir = config.root_path().join("docs");

    // ensure that the docs dir exists in target
    let mut target_dir = config.output_path().join("public");

    // keep track of how far we're nested
    let mut base_nesting_count = 0;

    // if we have a base_url, we need to push that on
    if !config.base_url().is_empty() {
        base_nesting_count += 1;
        target_dir.push(config.base_url());
    }

    debug!(log, "creating target directory"; "dir" => target_dir.display());
    fs::create_dir_all(&target_dir)?;

    // finally, we need to tag a `/` on so that it's added automatically in the output
    let base_url = format!("{}/", config.base_url());

    let mut handlebars = Handlebars::new();

    debug!(log, "loading handlebars templates");
    handlebars.register_template_file("page", "templates/html/page.hbs")?;
    handlebars.register_template_file("api", "templates/markdown/api.hbs")?;
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

    debug!(log, "walking directory tree to render files"; "dir" => docs_dir.display());
    // render all other *.md files as *.html, walking the tree
    for entry in WalkDir::new(&docs_dir) {
        let entry = entry?;
        let path = entry.path();
        let mut nesting_count = base_nesting_count;

        trace!(log, "processing file"; o!("path" => path.display(), "nesting_count" => nesting_count));

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

        let rendered_contents = comrak::markdown_to_html(&contents, &ComrakOptions::default());

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
            handlebars
                .render(
                    "page",
                    &json!({"contents": rendered_contents, "nest-count": nesting_count, "base-url": base_url }),
                )?
                .as_bytes(),
        )?;
    }

    info!(log, "done");
    Ok(())
}
