use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use ctrlc;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use simple_server::Server;
use slog::Logger;

use config::Config;
use ops;
use Result;

static SERVER_ADDR: (&str, &str) = ("127.0.0.1", "7878");

/// Requests sent internally to the serve function from other threads.
enum Request {
    /// Trigger a rebuild.
    Build,

    /// Terminate the server.
    Terminate,
}

/// Serve documentation locally with an HTTP server.
///
/// In addition, this function registers a file watcher on the markdown source folder and triggers
/// a rebuild of documentation when any of those files change.
///
/// Lastly, it registers a Ctrl-C handler to cleanly exit when the user requests shutdown.
/// Otherwise, the server continues indefinitely.
pub fn serve(config: &Config, log: &Logger) -> Result<()> {
    let log = log.new(o!("command" => "serve"));
    info!(log, "starting");

    let watcher_log = log.new(o!("step" => "watching"));
    let (watcher_tx, watcher_rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(watcher_tx, Duration::from_secs(2))?;
    let watch_path = config.markdown_path();
    info!(log, "watching {} for changes", watch_path.display());
    watcher.watch(watch_path, RecursiveMode::Recursive)?;

    let (request_tx, request_rx) = channel::<Request>();

    // If we receive an event from the watcher, send a rebuild request.
    let watcher_request_tx = request_tx.clone();
    thread::spawn(move || {
        for event in watcher_rx {
            trace!(watcher_log, "received event from watcher: {:?}", event);
            watcher_request_tx.send(Request::Build).unwrap();
        }
    });

    // Register a Ctrl-C handler for nice cleanup once the server is started.
    let ctrlc_request_tx = request_tx.clone();
    ctrlc::set_handler(move || {
        ctrlc_request_tx.send(Request::Terminate).unwrap();
    })?;

    // everything is handled by the static serving, so any request here is an error
    let mut server =
        Server::new(|_request, mut response| Ok(response.body(b"incorrect path".to_vec())?));

    let path = config.output_path();

    trace!(log, "setting static directory"; o!("dir" => path.display()));
    server.set_static_directory(path);

    let log = log.new(o!("step" => "starting server"));

    let (host, port) = SERVER_ADDR;
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

    // Start the server. This thread will never return, so we just expect it to be cleaned up on
    // program exit.
    let server_log = log.clone();
    thread::spawn(move || {
        info!(server_log, "press Ctrl+C to quit");
        server.listen(host, port);
    });

    for request in request_rx {
        match request {
            Request::Build => {
                info!(log, "file changed, regenerating docs");

                if let Err(e) = ops::build(config, &log) {
                    error!(log, "error building: {:?}", e);
                }

                info!(log, "done");
            }
            Request::Terminate => {
                info!(log, "termination signal received, shutting down");
                break;
            }
        }
    }

    Ok(())
}
