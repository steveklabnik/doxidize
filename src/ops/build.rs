use Result;
use std::path::Path;
use std::fs::{self, File};
use handlebars::{self, Handlebars};
use walkdir::WalkDir;
use comrak::{self, ComrakOptions};
use std::io::prelude::*;

pub fn build(dir: &Path) -> Result<()> {
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

        // TODO: this only works at one level down; we need to calculate the real number
        file.write_all(handlebars.render("page", &json!({"contents": rendered_contents, "nest-count": 1}))?.as_bytes())?;
    }

    Ok(())
}
