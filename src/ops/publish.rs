use Result;
use std::path::Path;
use git;

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
