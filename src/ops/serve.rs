use Result;
use simple_server::Server;
use std::env;
use config::Config;
use slog::Logger;

pub fn serve(config: &Config, log: &Logger) -> Result<()> {
    let log = log.new(o!("command" => "serve"));
    info!(log, "starting");

    let host = "127.0.0.1";
    let port = "7878";

    // everything is handled by the static serving, so any request here is
    // an error
    let server =
        Server::new(|_request, mut response| Ok(response.body("incorrect path".as_bytes())?));

    let path = config.output_path();
    trace!(log, "setting current directory"; o!("dir" => path.display()));
    env::set_current_dir(path)?;

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
