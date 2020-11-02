extern crate env_logger;
#[macro_use]
extern crate log;

extern crate http;
extern crate simple_test_bbarekas;

use http::header;
use simple_test_bbarekas::Server;

fn main() {
    let host = "127.0.0.1";
    let port = "7878";

    let server = Server::new(|request, mut response| {
        info!("Request received. {} {}", request.method(), request.uri());
        let res = "Hello Rust!".as_bytes().to_vec();
        response.header(header::CONTENT_TYPE, "text/plain".as_bytes());
        response.header(header::CONTENT_LENGTH, res.len());
        Ok(response.body(res)?)
    });

    server.listen(host, port);
}
