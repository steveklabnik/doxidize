extern crate doxidize;

#[macro_use]
extern crate configure;

use doxidize::Config;

use std::env;

fn main() {
    use_default_config!();

    // skip the program name
    let args: Vec<String> = env::args().skip(1).collect();

    let config = Config::default();

    if args.len() == 0 {
        doxidize::ops::create_skeleton(&config).expect("could not create skeleton");
    } else if args[0] == "build" {
        doxidize::ops::build(&config).expect("could not build docs");
    } else if args[0] == "publish" {
        doxidize::ops::publish(&config).expect("could not publish docs");
    } else if args[0] == "serve" {
        doxidize::ops::serve(&config).expect("could not serve docs");
    } else {
        panic!("incorrect command {}", args[0]);
    }
}
