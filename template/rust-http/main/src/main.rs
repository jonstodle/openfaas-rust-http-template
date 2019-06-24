use hyper::Server;
use hyper::service::service_fn;
use hyper::rt::Future;

fn main() {
    let address = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&address)
        .serve(|| service_fn(handler::handle))
        .map_err(|e| eprintln!("Server error: {}", e));

    hyper::rt::run(server);
}
