extern crate env_logger;
#[macro_use]
extern crate log;

extern crate simple_test_bbarekas;

use simple_test_bbarekas::Server;

fn main() {
    let host = "127.0.0.1";
    let port = "7878";

    let server = Server::new(|request, mut response| {
        info!("Request received. {} {}", request.method(), request.uri());
        Ok(response.body("Hello Rust!".as_bytes().to_vec())?)
    });

    info!("Starting server ... {}:{}", host, port);
    server.listen(host, port);
}
