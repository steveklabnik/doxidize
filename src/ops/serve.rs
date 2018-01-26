use Result;
use std::path::Path;
use simple_server::Server;
use std::env;

pub fn serve(directory: &Path) -> Result<()> {
    let host = "127.0.0.1";
    let port = "7878";

    // everything is handled by the static serving, so any request here is
    // an error
    let server =
        Server::new(|_request, mut response| Ok(response.body("incorrect path".as_bytes())?));

    env::set_current_dir(directory)?;

    println!("serving docs at http://{}:{}", host, port);

    server.listen(host, port);

    Ok(())
}
