extern crate doxidize;

use doxidize::create_skeleton;

use std::env;

fn main() {
    // for now, the only thing we can do is generate the skeleton, so let's do it!
    let current_dir = env::current_dir().expect("failed to get the current directory");

    create_skeleton(&current_dir).expect("could not create skeleton");
}
