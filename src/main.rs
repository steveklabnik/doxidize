extern crate doxidize;

use std::env;

fn main() {
    let current_dir = env::current_dir().expect("failed to get the current directory");

    // skip the program name
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        doxidize::create_skeleton(&current_dir).expect("could not create skeleton");
    } else if args[0] == "generate" {
        doxidize::generate(&current_dir).expect("could not generate docs");
    } else if args[0] == "publish" {
        let mut target_dir = current_dir.join("target");
        target_dir.push("docs");

        doxidize::publish(&current_dir, &target_dir).expect("could not publish docs");
    }
}
