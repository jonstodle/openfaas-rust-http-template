use hyper::{Body, Request, Response};
use futures::{Future, Stream};

/// Handle a request, where `req` is the HTTP request
pub fn handle(req : Request<Body>) -> impl Future<Item = Response<Body>, Error = hyper::Error> {
    req.into_body()
        .concat2()
        .map(|body|{
            let content = String::from_utf8(body.to_vec()).unwrap();
            Response::new(Body::from(format!("Hello, Rust. You said: {}", content)))
        })
}
