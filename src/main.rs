extern crate doxidize;

#[macro_use]
extern crate configure;

use doxidize::Config;

use std::env;

fn main() {
    use_default_config!();

    let current_dir = env::current_dir().expect("failed to get the current directory");

    // skip the program name
    let args: Vec<String> = env::args().skip(1).collect();

    // some commands need to fetch the target directory
    let mut target_dir = current_dir.join("target");
    target_dir.push("docs");

    let config = Config::default();

    if args.len() == 0 {

        doxidize::ops::create_skeleton(&current_dir, &config).expect("could not create skeleton");
    } else if args[0] == "build" {
        doxidize::ops::build(&current_dir, &config).expect("could not build docs");
    } else if args[0] == "publish" {
        // we want to publish the public directory, straight-up
        target_dir.push("public");

        doxidize::ops::publish(&current_dir, &target_dir, &config).expect("could not publish docs");
    } else if args[0] == "serve" {
        doxidize::ops::serve(&target_dir, &config).expect("could not serve docs");
    } else {
        panic!("incorrect command {}", args[0]);
    }
}
