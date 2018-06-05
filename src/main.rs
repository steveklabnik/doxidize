extern crate doxidize;

extern crate failure;
#[macro_use]
extern crate configure;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::path::PathBuf;

use failure::Error;
use slog::Drain;
use structopt::StructOpt;

use doxidize::Config;

#[derive(StructOpt, Debug)]
#[structopt(name = "doxidize", about = "Excellent documentation tooling for Rust")]
struct Opt {
    #[structopt(subcommand)]
    command: Option<Command>,

    #[structopt(long = "manifest-path",
                help = "The path to Cargo.toml",
                default_value = "./Cargo.toml",
                parse(from_os_str))]
    manifest_path: PathBuf,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "build")]
    Build,
    #[structopt(name = "clean")]
    Clean,
    #[structopt(name = "publish")]
    Publish,
    #[structopt(name = "serve", about = "Serve documentation on a local HTTP server")]
    Serve,
    #[structopt(name = "init")]
    Init,
    #[structopt(name = "update")]
    Update,
}

fn run(opts: Opt, log: &slog::Logger) -> Result<(), Error> {
    let config = Config::new(opts.manifest_path)?;

    info!(log, "doxidizing `{}`", config.root_path().display());

    if let Some(command) = opts.command {
        match command {
            Command::Build => doxidize::ops::build(&config, &log),
            Command::Clean => doxidize::ops::clean(&config, &log),
            Command::Publish => doxidize::ops::publish(&config, &log),
            Command::Serve => doxidize::ops::serve(&config, &log),
            Command::Init => doxidize::ops::init(&config, &log),
            Command::Update => doxidize::ops::update(&config, &log),
        }
    } else {
        doxidize::ops::init(&config, &log)
    }
}

fn main() {
    use_default_config!();

    let doxidize_version = env!("CARGO_PKG_VERSION");

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!("version" => doxidize_version));

    let opts = Opt::from_args();

    if let Err(err) = run(opts, &log) {
        error!(log, "{}", err);

        // Ensure the log is flushed before exiting.
        drop(log);

        std::process::exit(1);
    }
}
