use notify::{self, RecommendedWatcher, Watcher, RecursiveMode};
use simple_server::Server;
use slog::Logger;

use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use config::Config;
use ops;
use Result;

pub fn serve(config: Config, log: &Logger) -> Result<()> {
    let log = log.new(o!("command" => "serve"));
    info!(log, "starting");

    let config = Arc::new(config);
    let log = Arc::new(log);

    watch(config.clone(), log.clone(), env::current_dir()?)?;

    let host = "127.0.0.1";
    let port = "7878";

    // everything is handled by the static serving, so any request here is
    // an error
    let server =
        Server::new(|_request, mut response| Ok(response.body("incorrect path".as_bytes())?));

    let path = config.output_path();

    trace!(log, "setting current directory"; o!("dir" => path.display()));
    env::set_current_dir(path)?;

    let log = log.new(o!("step" => "starting server"));

    if config.base_url().is_empty() {
        info!(log, "serving docs at http://{}:{}/index.html", host, port);
    } else {
        info!(
            log,
            "serving docs at http://{}:{}/{}/index.html",
            host,
            port,
            config.base_url()
        );
    };

    server.listen(host, port);

    info!(log, "done");
    Ok(())
}

fn watch(config: Arc<Config>, log: Arc<Logger>, current_dir: PathBuf) -> notify::Result<()> {
    thread::spawn(move || {
        let (tx, rx) = channel();

        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).expect("could not create a Watcher");

        let path = current_dir.join(config.markdown_path());
        info!(log, "watching {} for changes", path.display());

        if let Err(e) = watcher.watch(path, RecursiveMode::Recursive) {
            error!(log, "error trying to watch: {}", e);
        }

        loop {
            match rx.recv() {
                Ok(_) =>  {
                    info!(log, "file changed, regenerating docs");

                    if let Err(e) = ops::build(&*config, &*log) {
                        error!(log, "error building: {:?}", e);
                    }

                    info!(log, "done");
                }
                Err(e) => error!(log, "watch error: {:?}", e),
            }
        }
    });

    Ok(())
}