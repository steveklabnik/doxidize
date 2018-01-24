extern crate doxidize;

use std::env;

fn main() {
    let current_dir = env::current_dir().expect("failed to get the current directory");

    // skip the program name
    let args: Vec<String> = env::args().skip(1).collect();

    // some commands need to fetch the target directory
    let mut target_dir = current_dir.join("target");
    target_dir.push("docs");

    if args.len() == 0 {
        doxidize::create_skeleton(&current_dir).expect("could not create skeleton");
    } else if args[0] == "build" {
        doxidize::build(&current_dir).expect("could not build docs");
    } else if args[0] == "publish" {
        // we want to publish the public directory, straight-up
        target_dir.push("public");

        doxidize::publish(&current_dir, &target_dir).expect("could not publish docs");
    } else if args[0] == "serve" {
        doxidize::serve(&target_dir).expect("could not serve docs");
    } else {
        panic!("incorrect command {}", args[0]);
    }
}
