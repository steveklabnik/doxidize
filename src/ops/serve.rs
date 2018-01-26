use Result;
use simple_server::Server;
use std::env;
use config::Config;

pub fn serve(config: &Config) -> Result<()> {
    let host = "127.0.0.1";
    let port = "7878";

    // everything is handled by the static serving, so any request here is
    // an error
    let server =
        Server::new(|_request, mut response| Ok(response.body("incorrect path".as_bytes())?));

    env::set_current_dir(config.output_path())?;

    println!("serving docs at http://{}:{}", host, port);

    server.listen(host, port);

    Ok(())
}
