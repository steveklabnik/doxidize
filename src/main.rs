extern crate doxidize;

#[macro_use]
extern crate configure;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use slog::Drain;
use structopt::StructOpt;

use doxidize::Config;

#[derive(StructOpt, Debug)]
#[structopt(name = "doxidize", about = "Execllent documentation tooling for Rust")]
struct Opt {
    #[structopt(subcommand)]
    command: Option<Command>,

    #[structopt(long = "manifest-path",
                help = "The path to a Cargo.toml, defaults to `./Cargo.toml`")]
    manifest_path: Option<String>,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "build")]
    Build,
    #[structopt(name = "clean")]
    Clean,
    #[structopt(name = "publish")]
    Publish,
    #[structopt(name = "serve")]
    Serve,
    #[structopt(name = "init")]
    Init,
}

fn main() {
    use_default_config!();

    let doxidize_version = env!("CARGO_PKG_VERSION");

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!("version" => doxidize_version));

    let opts = Opt::from_args();

    let config = if let Some(ref manifest_path) = opts.manifest_path {
        Config::with_manifest_path(manifest_path)
    } else {
        Config::default()
    };

    let result = match opts {
        Opt { ref command, .. } if command.is_some() => {
            // we just checked that it's Some
            match command.as_ref().unwrap() {
                &Command::Build => doxidize::ops::build(&config, &log),
                &Command::Clean => doxidize::ops::clean(&config, &log),
                &Command::Publish => doxidize::ops::publish(&config, &log),
                &Command::Serve => doxidize::ops::serve(config, &log),
                &Command::Init => doxidize::ops::init(&config, &log),
            }
        }
        _ => {
            // default with no command
            doxidize::ops::init(&config, &log)
        }
    };

    if let Err(err) = result {
        eprintln!("error! {}", err);
        std::process::exit(1);
    }
}
