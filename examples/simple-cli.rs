extern crate doxidize;

#[macro_use]
extern crate configure;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use slog::Drain;

use std::env;

use doxidize::Config;

fn main() {
    use_default_config!();

    let doxidize_version = env!("CARGO_PKG_VERSION");

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!("version" => doxidize_version));

    // skip the program name
    let args: Vec<String> = env::args().skip(1).collect();

    let config = Config::default();

    if args.len() == 0 {
        doxidize::ops::init(&config, &log).expect("could not init docs");
    } else if args[0] == "build" {
        doxidize::ops::build(&config, &log).expect("could not build docs");
    } else if args[0] == "publish" {
        doxidize::ops::publish(&config, &log).expect("could not publish docs");
    } else if args[0] == "serve" {
        doxidize::ops::serve(&config, &log).expect("could not serve docs");
    } else {
        panic!("incorrect command {}", args[0]);
    }
}
