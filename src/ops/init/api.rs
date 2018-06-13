use analysis::{AnalysisHost, Def, DefKind, Id as DefId};
use slog::Logger;

use std::collections::{HashSet, VecDeque};
use std::fs::{self, File};
use std::io::prelude::*;
use std::iter;
use std::path::{Path, PathBuf};

use cargo::{self, Target};
use error;
use strip_leading_space;
use Config;
use Result;

/// Creates the "API Reference" section.
///
/// Returns the paths of all files and directories created by this function.
pub fn create(config: &Config, log: &Logger) -> Result<HashSet<PathBuf>> {
    // ensure that the api dir exists
    let api_dir = config.api_markdown_path();
    debug!(log, "creating api dir";
    o!("dir" => api_dir.display()));
    fs::create_dir_all(&api_dir)?;

    let metadata = cargo::retrieve_metadata(config.manifest_path())?;
    let target = cargo::target_from_metadata(log, &metadata)?;

    generate_and_load_analysis(config, &target, log)?;

    let host = config.host();
    let crate_name = &target.crate_name();

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

    compile_analysis_to_markdown(config, log, host, api_dir, crate_name, root_id)
}

/// Generate save analysis data of a crate to be used later by the RLS library later and load it
/// into the analysis host.
fn generate_and_load_analysis(config: &Config, target: &Target, log: &Logger) -> Result<()> {
    let log = log.new(o!("step" => "analyzing your source code"));
    info!(log, "starting");

    cargo::generate_analysis(config, target)?;

    let root_path = config.root_path();
    debug!(log, "analysis complete, loading");
    config.host().reload(root_path, root_path)?;

    info!(log, "done");
    Ok(())
}

/// Compile the crate analysis into markdown files.
fn compile_analysis_to_markdown<P: AsRef<Path>>(
    config: &Config,
    log: &Logger,
    host: &AnalysisHost,
    api_dir: P,
    crate_name: &str,
    root_id: DefId,
) -> Result<HashSet<PathBuf>> {
    let log = log.new(o!("step" => "compiling analysis into markdown"));

    // Keep track of the list of files and directories that we created. This is needed so that
    // `update` can clean up what was left.
    let mut created_paths = HashSet::new();

    // First, compile the crate docs into the root README.
    debug!(log, "creating README.md for api");
    let api_readme_path = config.api_readme_path();
    o!("file" => api_readme_path.display());

    let root_def = host.get_def(root_id)?;
    fs::write(
        &api_readme_path,
        config.handlebars().render(
            "api",
            &json!({"name": crate_name, "docs": strip_leading_space(&root_def.docs)}),
        )?,
    )?;
    created_paths.insert(api_readme_path);

    // We have a separate overview page for modules, structs, and traits. The module page
    // contains a tree of existing modules, so we handle it separately later. Since
    // structs and traits are simpler, we just save their IDs as we come across them.
    let mut struct_ids = vec![];
    let mut trait_ids = vec![];

    let mut queue = VecDeque::new();
    queue.extend(host.for_each_child_def(root_id, |id, _def| id)?);

    // Write markdown for each def.
    while let Some(id) = queue.pop_front() {
        let def = host.get_def(id)?;

        // Push any children of the current def onto the queue.
        queue.extend(host.for_each_child_def(id, |id, _def| id)?);
        match def.kind {
            DefKind::Struct => {
                struct_ids.push(id);
            }
            DefKind::Trait => {
                trait_ids.push(id);
            }
            _ => (),
        }

        let containing_path = api_dir.as_ref().join(name_to_path(&def.qualname));
        debug!(log, "creating"; o!("dir" => containing_path.display()));
        fs::create_dir_all(&containing_path)?;
        created_paths.insert(containing_path.clone());

        let markdown_path = containing_path.join(&format!("{}.md", def.name));
        let template_name = match template_for_defkind(def.kind) {
            Some(template_name) => template_name,
            None => continue,
        };
        let markdown = config.handlebars().render(
            template_name,
            &json!({
                "name": def.name,
                "docs": strip_leading_space(&def.docs),
                "signature": def.value
            }),
        )?;
        debug!(log, "writing"; o!("file" => markdown_path.display()));
        fs::write(&markdown_path, markdown)?;
        created_paths.insert(markdown_path);
    }

    // Write the "All Modules" hierarchy file.
    let module_tree = ModuleTree::new(&host, root_id)?;
    let mut module_overview = File::create(config.api_module_overview_path())?;

    debug!(log, "writing"; o!("file" => config.api_module_overview_path().display()));
    writeln!(module_overview, "# Module overview\n")?;
    module_tree.write(host, config.base_url(), &mut module_overview, 0)?;
    created_paths.insert(config.api_module_overview_path().to_owned());

    debug!(log, "writing"; o!("file" => config.api_struct_overview_path().display()));
    let mut struct_overview = File::create(config.api_struct_overview_path())?;
    writeln!(struct_overview, "# Struct overview\n")?;
    struct_ids.sort_by_key(|id| host.get_def(*id).unwrap().name);

    for id in struct_ids {
        let def = host.get_def(id)?;
        writeln!(
            struct_overview,
            "* {}",
            link_for_def(config.base_url(), &def)
        )?;
    }
    created_paths.insert(config.api_struct_overview_path().to_owned());

    debug!(log, "writing"; o!("file" => config.api_trait_overview_path().display()));
    let mut trait_overview = File::create(config.api_trait_overview_path())?;
    writeln!(trait_overview, "# Trait overview\n")?;

    trait_ids.sort_by_key(|id| host.get_def(*id).unwrap().name);
    for id in trait_ids {
        let def = host.get_def(id)?;
        writeln!(
            trait_overview,
            "* {}",
            link_for_def(config.base_url(), &def)
        )?;
    }
    created_paths.insert(config.api_trait_overview_path().to_owned());

    info!(log, "done");

    Ok(created_paths)
}

/// Returns the name of the template for a given DefKind.
fn template_for_defkind(def_kind: DefKind) -> Option<&'static str> {
    let template_name = match def_kind {
        DefKind::Mod => "mod",
        DefKind::Struct => "struct",
        DefKind::Enum => "enum",
        DefKind::Trait => "trait",
        DefKind::Function => "function",
        DefKind::Type => "type",
        DefKind::Static => "static",
        DefKind::Const => "const",
        DefKind::Field => return None,
        DefKind::Tuple => return None,
        DefKind::Local => return None,
        // The below DefKinds are not supported in rls-analysis
        // DefKind::Union => ...
        // DefKind::Macro => ...
        // DefKind::Method => ...
        _ => return None,
    };

    Some(template_name)
}

/// Returns a markdown-formatted link to the API documentation for the Def.
///
/// The link is relative from the documentation root.
fn link_for_def(base_url: &str, def: &Def) -> String {
    // The qualname contains the entire path, including the crate name and the item name itself. We
    // don't need either of these, so we just remove them.
    let mut path = def.qualname.split("::").skip(1).collect::<Vec<_>>();
    path.pop();

    let base_url = format!("/{}", base_url);

    let (name, url) = if def.name.is_empty() {
        // If the name is empty, we're probably looking at the root crate. The link should point to
        // the crate overview, and the name should be the first component of the qualname.
        let name = def.qualname.split("::").next().unwrap();
        let url = format!("{}/api/index.html", base_url);
        (name, url)
    } else {
        let path = if path.is_empty() {
            String::from("")
        } else {
            path.join("/") + "/"
        };

        let url = format!("{}/api/{}{}.html", base_url, path, def.name);
        (def.name.as_str(), url)
    };

    format!("[{}]({})", name, url)
}

#[derive(Debug)]
struct ModuleTree {
    id: DefId,
    children: Vec<ModuleTree>,
}

impl ModuleTree {
    fn new(host: &AnalysisHost, root_id: DefId) -> Result<ModuleTree> {
        let mut child_module_ids = host
            .for_each_child_def(root_id, |id, def| match def.kind {
                DefKind::Mod => Some(id),
                _ => None,
            })?
            .into_iter()
            .flat_map(|id| id)
            .collect::<Vec<_>>();

        child_module_ids.sort_by_key(|id| host.get_def(*id).unwrap().name);
        let mut children = vec![];
        for id in child_module_ids {
            children.push(ModuleTree::new(host, id.clone())?);
        }

        Ok(ModuleTree {
            id: root_id,
            children,
        })
    }

    fn write<W: Write>(
        &self,
        host: &AnalysisHost,
        base_url: &str,
        writer: &mut W,
        depth: usize,
    ) -> Result<()> {
        let def = host.get_def(self.id)?;

        let indent = iter::repeat("  ").take(depth).collect::<String>();
        writeln!(writer, "{}* {}", indent, link_for_def(base_url, &def))?;

        for child in &self.children {
            child.write(host, base_url, writer, depth + 1)?;
        }

        Ok(())
    }
}

fn name_to_path(name: &str) -> PathBuf {
    // we skip the first bit since it's the crate name
    let mut path = name
        .split("::")
        .skip(1)
        .fold(PathBuf::new(), |mut path, component| {
            path.push(component);
            path
        });

    // we want the containing directory here, so we *also* have to pop off the last part
    path.pop();

    path
}

#[cfg(test)]
mod tests {
    mod name_to_path {
        use super::super::name_to_path;
        use std::path::PathBuf;

        #[test]
        fn nest_level1() {
            let name = "doxidize::examples";

            assert_eq!(PathBuf::new(), name_to_path(name));
        }

        #[test]
        fn nest_level2() {
            let name = "doxidize::examples::nested_module";

            let path = PathBuf::from("examples");
            assert_eq!(path, name_to_path(name));
        }

        #[test]
        fn nest_level3() {
            let name = "doxidize::examples::nested_module::second_nested_module";

            let path = PathBuf::from("examples").join("nested_module");
            assert_eq!(path, name_to_path(name));
        }

        #[test]
        fn nest_level4() {
            let name = "doxidize::examples::nested_module::second_nested_module::third";

            let path = PathBuf::from("examples")
                .join("nested_module")
                .join("second_nested_module");
            assert_eq!(path, name_to_path(name));
        }
    }
}
